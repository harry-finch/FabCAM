//! DXF importer. Builds on the `dxf` crate (path: `refs/dxf-rs`) which is a
//! pure parser — block expansion, NURBS evaluation, polyline bulge → arcs,
//! and ellipse flattening live here.
//!
//! Closely parallels `viaconstructor/input_plugins/dxfread.py` so the JSON
//! output is byte-for-byte identical for shapes both importers handle.

// # CAM/sim pedantic-lint exemptions
// DXF parsing converts group-code indices and chunk counters between
// usize/i32/f64; values are bounded by file size (≪ 2^31). `p_a`/`p_b`/`p_c`
// follow the textbook polygon-vertex convention.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::similar_names
)]

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use dxf::entities::{
    Arc, Circle, Ellipse, Entity, EntityType, Line, LwPolyline, MLine, MText, ModelPoint, Polyline,
    Spline, Text, Vertex,
};
use dxf::enums::Units;
use dxf::Drawing;

use crate::errors::{Error, SourceSpan};
use crate::geometry::{BBox, Point2, Segment};
use crate::input::nurbs;
use crate::input::{summarize_layers, ImportOptions, ImportOutput};
use crate::math;

/// Minimum chord distance below which we drop a degenerate segment.
/// Matches `viaconstructor/input_plugins/dxfread.py` `MIN_DIST = 0.0001`.
const MIN_DIST: f64 = 1e-4;

/// Reserved layer that carries CAM directives in MTEXT entities.
const CAMCFG_LAYER: &str = "_CAMCFG";

/// An OCS normal counts as parallel to Z when the XY part of its unit vector
/// is at most this. Exporters write exact `0,0,±1`; this only absorbs float
/// noise from normalizing scaled normals. Any larger tilt is a plane a 2D CAM
/// import cannot represent and is refused.
const OCS_PARALLEL_TOL: f64 = 1e-9;

/// Top-level entry: read a DXF file from disk and tessellate to segments.
///
/// # Errors
///
/// Returns `Error::io` if the file can't be opened / read, or the
/// underlying error from [`import_dxf_bytes`] if the DXF content is
/// malformed.
pub fn import_dxf_path(path: &Path, opts: &ImportOptions) -> crate::Result<ImportOutput> {
    let bytes = std::fs::read(path).map_err(|e| {
        Error::io(format!("read {}: {e}", path.display()))
            .with_hint("Check the file exists and is readable.")
    })?;
    let filename = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    import_dxf_bytes(filename, &bytes, opts)
}

/// Bytes-based DXF import. Two passes: first hand the buffer to dxf-rs
/// for the entities it natively supports, then text-mode-scan the same
/// buffer for HATCH entities (which dxf-rs 0.6 silently swallows) and
/// append their boundary geometry to the result.
///
/// # Errors
///
/// Returns `Error::bad_input` if the bytes don't parse as a DXF
/// drawing, or propagates any error from [`import_drawing`].
pub fn import_dxf_bytes(
    filename: String,
    bytes: &[u8],
    opts: &ImportOptions,
) -> crate::Result<ImportOutput> {
    // DXF group keywords are ASCII even when names use a legacy encoding.
    // Do this before expansion/recovery, so no partial drawing can escape.
    if super::hatch::has_block_hatch(&String::from_utf8_lossy(bytes)) {
        return Err(Error::bad_input("DXF HATCH inside BLOCK definitions is not supported.")
            .with_hint("Explode block instances in your CAD tool and export their HATCH boundaries as top-level geometry before importing."));
    }
    let drawing = Drawing::load(&mut std::io::Cursor::new(bytes)).map_err(|e| {
        let mut err = Error::bad_input(format!("dxf parse: {e}"))
            .with_hint("File is not a valid DXF — try re-exporting from your CAD tool.");
        if let Some(span) = parse_error_span(&filename, &e) {
            err = err.with_span(span);
        }
        err
    })?;
    let mut out = import_drawing(&drawing, filename, opts)?;

    // HATCH boundary recovery pass — only meaningful for text-mode DXFs.
    if let Ok(text) = std::str::from_utf8(bytes) {
        let mut hatch_segments = Vec::new();
        let n = super::hatch::extract_hatch_boundaries(
            text,
            out.unit_scale,
            &mut hatch_segments,
            &mut out.warnings,
        );
        if n > 0 {
            // BBox + per-layer counts need a refresh.
            let colors: BTreeMap<_, _> = drawing
                .layers()
                .map(|l| (l.name.clone(), color_to_aci(&l.color, 7)))
                .collect();
            for segment in &mut hatch_segments {
                if !(1..=255).contains(&segment.color) {
                    segment.color = *colors.get(segment.layer.as_ref()).unwrap_or(&7);
                }
                if opts.color_layers {
                    segment.layer = format!("{}-c{}", segment.layer, segment.color).into();
                }
            }
            hatch_segments.retain(|s| {
                opts.select_layers.is_empty()
                    || opts.select_layers.iter().any(|l| l == s.layer.as_ref())
            });
            out.segments.extend(hatch_segments);
            (out.objects, out.object_meta) = super::object_index(&out.segments);
            out.bbox = crate::geometry::BBox::from_segments(&out.segments);
            out.layers = super::summarize_layers(
                &out.segments,
                &drawing
                    .layers()
                    .map(|l| (l.name.clone(), color_to_aci(&l.color, 7)))
                    .collect(),
            );
            out.warnings
                .push(format!("recovered {n} HATCH boundary path(s)"));
        }
    }
    Ok(out)
}

/// Importer entry point that takes an already-parsed `Drawing`. Useful for
/// tests and for the bytes-based import path.
///
/// # Errors
///
/// Returns `Error::bad_input` if the drawing has no extractable
/// entities or its CAMCFG MTEXT entries fail to parse.
pub fn import_drawing(
    drawing: &Drawing,
    filename: String,
    opts: &ImportOptions,
) -> crate::Result<ImportOutput> {
    let mut warnings = Vec::new();

    let drawing_scale = unit_scale_from_drawing(drawing);
    let unit_scale = if opts.scale > 0.0 {
        opts.scale
    } else {
        drawing_scale.unwrap_or(1.0)
    };
    let unit_source = format!(
        "$INSUNITS = {}",
        match drawing_scale {
            Some(_) => format!("{:?}", drawing.header.default_drawing_units),
            None => "absent or unitless".into(),
        }
    );
    let unit_status = if opts.scale > 0.0 {
        crate::input::ImportUnitStatus::Explicit
    } else if drawing_scale.is_some() {
        crate::input::ImportUnitStatus::Declared
    } else {
        crate::input::ImportUnitStatus::Assumed
    };
    if opts.scale <= 0.0 && drawing_scale.is_none() {
        // Unchanged policy (1 drawing unit = 1 mm), but never silent.
        warnings.push(
            "DXF drawing units are not set ($INSUNITS absent or unitless): 1 drawing unit was imported as 1 mm. \
             Check the imported size and adjust the file transform scale if the drawing is in inches or another unit."
                .into(),
        );
    }

    // Layer color seed: every named layer with its ACI color.
    let mut layer_colors: BTreeMap<String, i32> = BTreeMap::new();
    for layer in drawing.layers() {
        layer_colors.insert(layer.name.clone(), color_to_aci(&layer.color, 7));
    }

    let select_set: Option<std::collections::HashSet<&str>> = if opts.select_layers.is_empty() {
        None
    } else {
        Some(opts.select_layers.iter().map(String::as_str).collect())
    };

    // Eagerly load the font once per import. Empty bytes => no rendering
    // (Phase 4 removed the inline TEXT/MTEXT renderer — text entities
    // now flow through ImportedTextEntity into the frontend, where the
    // user picks a font. The legacy font_bytes load is gone.)
    let mut ctx = ImportCtx {
        opts,
        unit_scale,
        warnings: &mut warnings,
        segments: Vec::new(),
        layer_colors: &layer_colors,
        select: select_set.as_ref(),
        cam_setup: String::new(),
        curve_error: false,
        refusal: None,
        text_entities: Vec::new(),
        layer_intern: crate::input::intern::LayerIntern::new(),
    };

    // Build a block index for INSERT expansion. Block lookup is by the
    // block's `name` field.
    let blocks: HashMap<String, &dxf::Block> =
        drawing.blocks().map(|b| (b.name.clone(), b)).collect();

    for entity in drawing.entities() {
        // Drop entities living on the CAM-config layer; their MTEXT carries
        // user-supplied CAM directives we expose via `cam_setup`.
        if entity.common.layer == CAMCFG_LAYER {
            if let EntityType::MText(m) = &entity.specific {
                ctx.cam_setup.push_str(&m.text.replace("\\P", "\n"));
                ctx.cam_setup.push('\n');
            }
            continue;
        }
        ctx.add_entity(entity, &blocks, &Transform2D::identity(), 0);
    }

    if let Some(reason) = ctx.refusal.take() {
        return Err(Error::bad_input(reason).with_hint(
            "Flatten the drawing to the XY plane (extrusion 0,0,1) and explode MINSERT arrays in the CAD tool, then import again.",
        ));
    }
    if ctx.curve_error {
        return Err(Error::bad_input(
            "DXF curve cannot meet the 0.01 mm import budget or contains non-finite geometry",
        ));
    }
    let cam_setup_len = ctx.cam_setup.len();
    let segments = std::mem::take(&mut ctx.segments);
    let text_entities = std::mem::take(&mut ctx.text_entities);
    drop(ctx);
    if cam_setup_len > 0 {
        warnings.push(format!(
            "_CAMCFG layer carries {cam_setup_len} bytes of CAM directives (parser stub)"
        ));
    }
    let bbox = BBox::from_segments(&segments);
    let layers = summarize_layers(&segments, &layer_colors);
    let (objects, object_meta) = super::object_index(&segments);

    Ok(ImportOutput {
        filename,
        format: "dxf".into(),
        segments,
        layers,
        bbox,
        unit_scale,
        warnings,
        objects,
        object_meta,
        text_entities,
        unit_status,
        unit_source,
    })
}

fn unit_scale_from_drawing(drawing: &Drawing) -> Option<f64> {
    let units = drawing.header.default_drawing_units;
    let factor = match units {
        Units::Unitless => return None,
        Units::Inches => 25.4,
        Units::Feet => 304.8,
        Units::Miles => 1_609_344.0,
        Units::Millimeters => 1.0,
        Units::Centimeters => 10.0,
        Units::Meters => 1000.0,
        Units::Kilometers => 1_000_000.0,
        Units::Microinches => 0.0254 / 1000.0,
        Units::Mils => 0.0254,
        Units::Yards => 914.4,
        Units::Angstroms => 1e-7,
        Units::Nanometers => 1e-6,
        Units::Microns => 1e-3,
        Units::Decimeters => 100.0,
        Units::Decameters => 10_000.0,
        Units::Hectometers => 100_000.0,
        Units::Gigameters => 1e12,
        Units::AstronomicalUnits => 1.496e14,
        Units::LightYears => 9.461e18,
        Units::Parsecs => 3.086e19,
        Units::USSurveyFeet => 304.800_609_6,
        Units::USSurveyInch => 25.400_050_80,
        Units::USSurveyYard => 914.401_828_8,
        Units::USSurveyMile => 1_609_347.218_694,
    };
    Some(factor)
}

struct ImportCtx<'a> {
    opts: &'a ImportOptions,
    unit_scale: f64,
    warnings: &'a mut Vec<String>,
    segments: Vec<Segment>,
    layer_colors: &'a BTreeMap<String, i32>,
    select: Option<&'a std::collections::HashSet<&'a str>>,
    cam_setup: String,
    curve_error: bool,
    /// First entity whose coordinate system or array a 2D import refuses
    /// (tilted extrusion, MINSERT). Rejects the whole import: dropping the
    /// entity silently would remove machining geometry.
    refusal: Option<String>,
    /// TEXT / MTEXT metadata captured during import. Emitted to
    /// `ImportOutput.text_entities` so the frontend can turn each into
    /// an editable `TextLayer` instead of consuming pre-rendered glyphs.
    text_entities: Vec<crate::input::ImportedTextEntity>,
    /// Per-import layer-name cache. Interned once per resolved
    /// layer string; every Segment on that layer reuses the cached
    /// Arc<str> instead of allocating a fresh one.
    layer_intern: crate::input::intern::LayerIntern,
}

impl ImportCtx<'_> {
    fn resolve_layer_and_color(&self, entity: &Entity) -> (String, i32) {
        let common = &entity.common;
        let layer = common.layer.clone();
        // BYLAYER: inherit from the layer table.
        let color = if common.color.is_by_layer() {
            *self.layer_colors.get(&layer).unwrap_or(&7)
        } else {
            color_to_aci(&common.color, 7)
        };
        let layer = if self.opts.color_layers {
            format!("{layer}-c{color}")
        } else {
            layer
        };
        (layer, color)
    }

    fn layer_selected(&self, layer: &str) -> bool {
        self.select.map_or(true, |sel| sel.contains(layer))
    }

    fn add_entity(
        &mut self,
        entity: &Entity,
        blocks: &HashMap<String, &dxf::Block>,
        xform: &Transform2D,
        depth: usize,
    ) {
        if depth > 32 {
            self.warnings
                .push("INSERT depth limit (32) reached; cycle?".into());
            return;
        }
        let (layer, color) = self.resolve_layer_and_color(entity);
        if !self.layer_selected(&layer) {
            return;
        }
        // Intern once per entity. Every Segment this emit_* fans out
        // to clones the cached Arc<str> instead of allocating its own.
        let layer_arc = self.layer_intern.intern(&layer);
        // Entities whose points live in their Object Coordinate System are
        // mapped OCS → (block or world) coordinates first, then by `xform`.
        // LINE, POINT, SPLINE, MLINE and MTEXT points are already WCS.
        let ocs = |ctx: &mut Self, normal: &dxf::Vector, kind: &str| {
            let t = ocs_transform(normal);
            if t.is_none() && ctx.refusal.is_none() {
                ctx.refusal = Some(format!(
                    "DXF {kind} on layer '{layer}' has extrusion direction ({}, {}, {}); only planes parallel to XY (0,0,±1) can be imported",
                    normal.x, normal.y, normal.z
                ));
            }
            t.map(|t| xform.compose(&t))
        };
        match &entity.specific {
            EntityType::Line(line) => self.emit_line(line, &layer_arc, color, xform),
            EntityType::Circle(c) => {
                if let Some(x) = ocs(self, &c.normal, "CIRCLE") {
                    self.emit_circle(c, &layer_arc, color, &x);
                }
            }
            EntityType::Arc(a) => {
                if let Some(x) = ocs(self, &a.normal, "ARC") {
                    self.emit_arc(a, &layer_arc, color, &x);
                }
            }
            EntityType::Ellipse(e) => {
                // Center and major axis are WCS; the normal only orients the
                // minor axis (normal × major), i.e. the parameter direction.
                if let Some(o) = ocs_transform(&e.normal) {
                    self.emit_ellipse(e, o.a, &layer_arc, color, xform);
                } else {
                    ocs(self, &e.normal, "ELLIPSE");
                }
            }
            EntityType::ModelPoint(p) => self.emit_point(p, &layer_arc, color, xform),
            EntityType::LwPolyline(p) => {
                if let Some(x) = ocs(self, &p.extrusion_direction, "LWPOLYLINE") {
                    self.emit_lwpolyline(p, &layer_arc, color, &x);
                }
            }
            // 3D polylines (and meshes) store WCS vertices; 2D ones use OCS.
            EntityType::Polyline(p) if p.is_3d_polyline() || p.is_3d_polygon_mesh() => {
                self.emit_polyline(p, &layer_arc, color, xform);
            }
            EntityType::Polyline(p) => {
                if let Some(x) = ocs(self, &p.normal, "POLYLINE") {
                    self.emit_polyline(p, &layer_arc, color, &x);
                }
            }
            EntityType::Spline(s) => self.emit_spline(s, &layer_arc, color, xform),
            EntityType::Insert(insert) => {
                if insert.column_count > 1 || insert.row_count > 1 {
                    if self.refusal.is_none() {
                        self.refusal = Some(format!(
                            "DXF MINSERT array of block '{}' ({} x {}) on layer '{layer}' is not supported",
                            insert.name, insert.column_count, insert.row_count
                        ));
                    }
                    return;
                }
                let Some(block) = blocks.get(&insert.name) else {
                    self.warnings
                        .push(format!("INSERT references missing block '{}'", insert.name));
                    return;
                };
                let Some(placed) = ocs(self, &insert.extrusion_direction, "INSERT") else {
                    return;
                };
                // DXF: WCS = OCS(location + R(rotation)·S(sx, sy)·(p − base)).
                let block_xform = placed
                    .compose(&Transform2D::insert(
                        insert.location.x,
                        insert.location.y,
                        insert.x_scale_factor,
                        insert.y_scale_factor,
                        insert.rotation.to_radians(),
                    ))
                    .compose(&Transform2D::translation(
                        -block.base_point.x,
                        -block.base_point.y,
                    ));
                for sub in &block.entities {
                    self.add_entity(sub, blocks, &block_xform, depth + 1);
                }
            }
            EntityType::MLine(m) => self.emit_mline(m, &layer_arc, color, xform),
            EntityType::Text(t) if !self.opts.no_text => {
                if let Some(x) = ocs(self, &t.normal, "TEXT") {
                    self.emit_text(t, &layer, color, &x);
                }
            }
            EntityType::MText(t) if !self.opts.no_text => self.emit_mtext(t, &layer, color, xform),
            EntityType::AttributeDefinition(_) => {
                // Attribute definitions are template-only; ignored.
            }
            other => {
                self.warnings.push(format!(
                    "unsupported entity {} on layer '{layer}'",
                    entity_type_name(other)
                ));
            }
        }
    }

    fn push_line(&mut self, start: Point2, end: Point2, layer: &std::sync::Arc<str>, color: i32) {
        if start.distance(end) > MIN_DIST {
            self.segments
                .push(Segment::line(start, end, layer.clone(), color));
        }
    }

    fn push_arc_segments(
        &mut self,
        start: Point2,
        end: Point2,
        bulge: f64,
        center: Option<Point2>,
        layer: &std::sync::Arc<str>,
        color: i32,
    ) {
        if start.distance(end) <= MIN_DIST && bulge.abs() < 1e-12 {
            return;
        }
        let step = self.opts.arc_max_step_or_default();
        if step >= std::f64::consts::TAU || bulge.abs() < 1e-9 {
            // Single segment with bulge — preferred for the CAM core.
            self.segments.push(Segment::arc(
                start,
                end,
                bulge,
                center,
                layer.clone(),
                color,
            ));
            return;
        }
        let pts = math::tessellate_arc(start, end, bulge, step);
        // Decompose each subdivision into a small bulge arc preserving curvature.
        // Every sub-arc spans an equal slice of the total sweep, so they all
        // share one bulge value (`tan(per_step / 4)`).
        let (_c, a0, a1, _r) = math::bulge_to_arc(start, end, bulge);
        let total_sweep = {
            let mut s = a1 - a0;
            if bulge > 0.0 && s < 0.0 {
                s += std::f64::consts::TAU;
            }
            if bulge < 0.0 && s > 0.0 {
                s -= std::f64::consts::TAU;
            }
            s
        };
        let per_step = total_sweep / (pts.len() - 1) as f64;
        let sub_bulge = (per_step * 0.25).tan();
        let center_known = center.or_else(|| Some(math::bulge_to_arc(start, end, bulge).0));
        for i in 0..pts.len() - 1 {
            self.segments.push(Segment::arc(
                pts[i],
                pts[i + 1],
                sub_bulge,
                center_known,
                layer.clone(),
                color,
            ));
        }
    }

    fn emit_line(
        &mut self,
        line: &Line,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let s = xform.apply(line.p1.x, line.p1.y);
        let e = xform.apply(line.p2.x, line.p2.y);
        self.push_line(self.scale(s), self.scale(e), layer, color);
    }

    fn emit_point(
        &mut self,
        p: &ModelPoint,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let pt = self.scale(xform.apply(p.location.x, p.location.y));
        self.segments.push(Segment::point(pt, layer.clone(), color));
    }

    fn emit_circle(
        &mut self,
        circle: &Circle,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        if ![circle.center.x, circle.center.y, circle.radius]
            .iter()
            .all(|v| v.is_finite())
            || circle.radius <= 0.0
        {
            self.curve_error = true;
            return;
        }
        let c = Point2::new(circle.center.x, circle.center.y);
        if !xform.is_similarity() {
            self.emit_parametric(
                c,
                Point2::new(circle.radius, 0.0),
                Point2::new(0.0, circle.radius),
                0.0,
                std::f64::consts::TAU,
                layer,
                color,
                xform,
            );
            return;
        }
        let center = self.scale(xform.apply(c.x, c.y));
        let right = self.scale(xform.apply(c.x + circle.radius, c.y));
        let left = self.scale(xform.apply(c.x - circle.radius, c.y));
        if right.distance(center) < MIN_DIST {
            return;
        }
        let bulge = xform.orientation();
        for (start, end) in [(right, left), (left, right)] {
            self.segments.push(Segment {
                kind: crate::geometry::SegmentKind::Circle,
                start,
                end,
                bulge,
                center: Some(center),
                layer: layer.clone(),
                color,
            });
        }
    }

    fn emit_arc(
        &mut self,
        arc: &Arc,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        if ![
            arc.center.x,
            arc.center.y,
            arc.radius,
            arc.start_angle,
            arc.end_angle,
        ]
        .iter()
        .all(|v| v.is_finite())
            || arc.radius <= 0.0
        {
            self.curve_error = true;
            return;
        }
        let a0 = arc.start_angle.rem_euclid(360.0).to_radians();
        let a1 = arc.end_angle.rem_euclid(360.0).to_radians();
        let mut total = (a1 - a0).rem_euclid(std::f64::consts::TAU);
        if total == 0.0 {
            total = std::f64::consts::TAU;
        }
        let c = Point2::new(arc.center.x, arc.center.y);
        if !xform.is_similarity() {
            self.emit_parametric(
                c,
                Point2::new(arc.radius, 0.0),
                Point2::new(0.0, arc.radius),
                a0,
                total,
                layer,
                color,
                xform,
            );
            return;
        }
        let center = self.scale(xform.apply(c.x, c.y));
        let step = self
            .opts
            .arc_max_step_or_default()
            .min(std::f64::consts::PI);
        let n = ((total / step).ceil() as usize).clamp(1, 65536);
        let per = total / n as f64;
        let point = |a: f64| {
            self.scale(xform.apply(c.x + arc.radius * a.cos(), c.y + arc.radius * a.sin()))
        };
        let pts: Vec<_> = (0..=n).map(|i| point(a0 + per * i as f64)).collect();
        for i in 0..n {
            let end = if i + 1 == n && total == std::f64::consts::TAU {
                pts[0]
            } else {
                pts[i + 1]
            };
            self.segments.push(Segment::arc(
                pts[i],
                end,
                (per / 4.0).tan() * xform.orientation(),
                Some(center),
                layer.clone(),
                color,
            ));
        }
    }

    /// `normal_sign` is +1 for a +Z normal and -1 for -Z: the minor axis is
    /// `normal × major · ratio`, so -Z reverses the parameter direction.
    fn emit_ellipse(
        &mut self,
        e: &Ellipse,
        normal_sign: f64,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let mut sweep = (e.end_parameter - e.start_parameter).rem_euclid(std::f64::consts::TAU);
        if sweep == 0.0 {
            sweep = std::f64::consts::TAU;
        }
        self.emit_parametric(
            Point2::new(e.center.x, e.center.y),
            Point2::new(e.major_axis.x, e.major_axis.y),
            Point2::new(
                -normal_sign * e.major_axis.y * e.minor_axis_ratio,
                normal_sign * e.major_axis.x * e.minor_axis_ratio,
            ),
            e.start_parameter,
            sweep,
            layer,
            color,
            xform,
        );
    }

    /// Linear interpolation error <= max|p''| * step²/8, measured after
    /// the full affine transform and unit conversion. Budget: 0.01 mm.
    #[allow(clippy::too_many_arguments)]
    fn emit_parametric(
        &mut self,
        center: Point2,
        major: Point2,
        minor: Point2,
        start: f64,
        sweep: f64,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let c = self.scale(xform.apply(center.x, center.y));
        let u = self.scale(xform.vector(major));
        let v = self.scale(xform.vector(minor));
        let bound = u.x.hypot(u.y) + v.x.hypot(v.y);
        let count = (sweep.abs() / (8.0 * 0.01 / bound).sqrt()).ceil().max(1.0);
        if ![c.x, c.y, u.x, u.y, v.x, v.y, start, sweep, count]
            .iter()
            .all(|x| x.is_finite())
            || count > 65536.0
        {
            self.curve_error = true;
            return;
        }
        let n = count as usize;
        let point = |t: f64| {
            Point2::new(
                c.x + u.x * t.cos() + v.x * t.sin(),
                c.y + u.y * t.cos() + v.y * t.sin(),
            )
        };
        let first = point(start);
        let mut prev = first;
        for i in 1..=n {
            let end = if i == n && sweep.abs() == std::f64::consts::TAU {
                first
            } else {
                point(start + sweep * i as f64 / n as f64)
            };
            // Do not discard short chords: doing so opens a valid boundary.
            if prev != end {
                self.segments
                    .push(Segment::line(prev, end, layer.clone(), color));
            }
            prev = end;
        }
    }

    fn emit_poly_edge(
        &mut self,
        s: Point2,
        e: Point2,
        bulge: f64,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        if bulge.abs() <= 1e-12 {
            self.push_line(
                self.scale(xform.apply(s.x, s.y)),
                self.scale(xform.apply(e.x, e.y)),
                layer,
                color,
            );
        } else if xform.is_similarity() {
            self.push_arc_segments(
                self.scale(xform.apply(s.x, s.y)),
                self.scale(xform.apply(e.x, e.y)),
                bulge * xform.orientation(),
                None,
                layer,
                color,
            );
        } else {
            let (c, a0, _, radius) = math::bulge_to_arc(s, e, bulge);
            let before = self.segments.len();
            self.emit_parametric(
                c,
                Point2::new(radius, 0.0),
                Point2::new(0.0, radius),
                a0,
                4.0 * bulge.atan(),
                layer,
                color,
                xform,
            );
            // Keep original transformed endpoints exactly to retain chaining.
            if self.segments.len() > before {
                self.segments[before].start = self.scale(xform.apply(s.x, s.y));
                let end = self.scale(xform.apply(e.x, e.y));
                self.segments.last_mut().unwrap().end = end;
            }
        }
    }

    fn emit_lwpolyline(
        &mut self,
        poly: &LwPolyline,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let n = poly.vertices.len();
        if n < 2 {
            return;
        }
        let closed = poly.is_closed();
        let pts: Vec<(Point2, f64)> = poly
            .vertices
            .iter()
            .map(|v| (Point2::new(v.x, v.y), v.bulge))
            .collect();
        let last = if closed { n } else { n - 1 };
        for i in 0..last {
            let (s, bulge) = pts[i];
            let (e, _) = pts[(i + 1) % n];
            self.emit_poly_edge(s, e, bulge, layer, color, xform);
        }
    }

    fn emit_polyline(
        &mut self,
        poly: &Polyline,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let vertices: Vec<&Vertex> = poly.vertices().collect();
        let n = vertices.len();
        if n < 2 {
            return;
        }
        let closed = poly.is_closed();
        let pts: Vec<(Point2, f64)> = vertices
            .iter()
            .map(|v| (Point2::new(v.location.x, v.location.y), v.bulge))
            .collect();
        let last = if closed { n } else { n - 1 };
        for i in 0..last {
            let (s, bulge) = pts[i];
            let (e, _) = pts[(i + 1) % n];
            self.emit_poly_edge(s, e, bulge, layer, color, xform);
        }
    }

    /// Flatten an MLINE into the spine (centerline) plus two parallel
    /// offsets at ±`scale_factor/2` along each vertex's miter direction. Style
    /// table support (per-element offsets, caps, joints) is intentionally
    /// out of scope; the typical 2-element default style emerges naturally.
    fn emit_mline(
        &mut self,
        m: &MLine,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        if m.vertices.len() < 2 {
            return;
        }
        let closed = (m.flags & 2) != 0;
        let half = (m.scale_factor * 0.5).abs();
        let n = m.vertices.len();

        // Spine: connect vertices directly.
        let last = if closed { n } else { n - 1 };
        for i in 0..last {
            let v0 = &m.vertices[i];
            let v1 = &m.vertices[(i + 1) % n];
            let a = self.scale(xform.apply(v0.x, v0.y));
            let b = self.scale(xform.apply(v1.x, v1.y));
            self.push_line(a, b, layer, color);
        }

        if half < 1e-9 || m.miter_directions.is_empty() {
            return;
        }

        // Two parallel offsets at ±half along each vertex's miter direction.
        // We expect miter_directions.len() == n; if not, skip the offsets to
        // avoid index-out-of-range surprises with malformed files.
        if m.miter_directions.len() < n {
            self.warnings.push(format!(
                "MLINE on layer '{layer}' had {} vertices but {} miter directions; skipping parallel offsets",
                n,
                m.miter_directions.len()
            ));
            return;
        }
        for sign in [-1.0_f64, 1.0_f64] {
            for i in 0..last {
                let v0 = &m.vertices[i];
                let v1 = &m.vertices[(i + 1) % n];
                let m0 = &m.miter_directions[i];
                let m1 = &m.miter_directions[(i + 1) % n];
                let a =
                    self.scale(xform.apply(v0.x + sign * half * m0.x, v0.y + sign * half * m0.y));
                let b =
                    self.scale(xform.apply(v1.x + sign * half * m1.x, v1.y + sign * half * m1.y));
                self.push_line(a, b, layer, color);
            }
        }
    }

    fn emit_text(&mut self, t: &Text, layer: &str, _color: i32, xform: &Transform2D) {
        let origin_world = xform.apply(t.location.x, t.location.y);
        let origin = self.scale(origin_world);
        let height = t.text_height * self.unit_scale * xform.uniform_scale_factor();
        if height < 1e-6 || t.value.is_empty() {
            return;
        }
        // Phase 4: emit editable metadata instead of rendering glyphs.
        // The frontend turns each entry into a TextLayer (with a default
        // bundled font) so the user can rewrite the content / swap the
        // font / re-place. The pipeline renders glyphs at Generate time.
        let rot_deg = if xform.orientation() > 0.0 {
            (t.rotation + xform.rotation.to_degrees()) % 360.0
        } else {
            // Mirrored (OCS -Z or mirrored INSERT): keep the true baseline
            // direction. Editable text cannot itself be mirrored.
            let a = t.rotation.to_radians();
            let d = xform.vector(Point2::new(a.cos(), a.sin()));
            d.y.atan2(d.x).to_degrees().rem_euclid(360.0)
        };
        self.text_entities.push(crate::input::ImportedTextEntity {
            kind: crate::input::ImportedTextKind::Text,
            source_layer: layer.to_string(),
            text: t.value.clone(),
            size_mm: height,
            origin: (origin.x, origin.y),
            rotation_deg: rot_deg,
        });
    }

    fn emit_mtext(&mut self, t: &MText, layer: &str, _color: i32, xform: &Transform2D) {
        let origin_world = xform.apply(t.insertion_point.x, t.insertion_point.y);
        let origin = self.scale(origin_world);
        let height = t.initial_text_height * self.unit_scale * xform.uniform_scale_factor();
        if height < 1e-6 || t.text.is_empty() {
            return;
        }
        // Normalize MTEXT's `\P` line break sentinel to `\n` — TextLayer
        // uses the standard newline for multi-line content (the pipeline
        // splits on `\n` at render time).
        let text = t.text.replace("\\P", "\n");
        let rot_deg = xform.rotation.to_degrees() % 360.0;
        self.text_entities.push(crate::input::ImportedTextEntity {
            kind: crate::input::ImportedTextKind::Mtext,
            source_layer: layer.to_string(),
            text,
            size_mm: height,
            origin: (origin.x, origin.y),
            rotation_deg: rot_deg,
        });
    }

    fn emit_spline(
        &mut self,
        spline: &Spline,
        layer: &std::sync::Arc<str>,
        color: i32,
        xform: &Transform2D,
    ) {
        let degree = spline.degree_of_curve as usize;
        if degree == 0 || degree >= spline.control_points.len() {
            self.warnings.push(format!("SPLINE on layer '{layer}': invalid degree {degree}; control polygon imported, not the spline"));
        }
        let knots: Vec<f64> = spline.knot_values.clone();
        let cps: Vec<(f64, f64, f64)> = spline
            .control_points
            .iter()
            .map(|p| (p.x, p.y, 1.0))
            .collect();
        let weights: Vec<f64> = if spline.weight_values.len() == cps.len() {
            spline.weight_values.clone()
        } else {
            vec![1.0; cps.len()]
        };
        let pts = nurbs::flatten(degree, &knots, &cps, &weights, 64);
        let pts: Vec<Point2> = pts
            .into_iter()
            .map(|(x, y)| self.scale(xform.apply(x, y)))
            .collect();
        for w in pts.windows(2) {
            self.push_line(w[0], w[1], layer, color);
        }
    }

    fn scale(&self, p: Point2) -> Point2 {
        Point2::new(p.x * self.unit_scale, p.y * self.unit_scale)
    }
}

fn entity_type_name(t: &EntityType) -> &'static str {
    match t {
        EntityType::Body(_) => "BODY",
        EntityType::Face3D(_) => "3DFACE",
        EntityType::Helix(_) => "HELIX",
        EntityType::Image(_) => "IMAGE",
        EntityType::Leader(_) => "LEADER",
        EntityType::Light(_) => "LIGHT",
        EntityType::MLine(_) => "MLINE",
        EntityType::Ole2Frame(_) => "OLE2FRAME",
        EntityType::OleFrame(_) => "OLEFRAME",
        EntityType::Solid3D(_) => "3DSOLID",
        EntityType::ProxyEntity(_) => "PROXY",
        EntityType::Ray(_) => "RAY",
        EntityType::Region(_) => "REGION",
        EntityType::Section(_) => "SECTION",
        EntityType::Shape(_) => "SHAPE",
        EntityType::Solid(_) => "SOLID",
        EntityType::Tolerance(_) => "TOLERANCE",
        EntityType::Trace(_) => "TRACE",
        EntityType::Wipeout(_) => "WIPEOUT",
        EntityType::XLine(_) => "XLINE",
        _ => "UNKNOWN",
    }
}

/// Convert a `dxf::Color` to an `AutoCAD` ACI integer (1..=255). For special
/// values (BYLAYER, BYBLOCK, off, BYENTITY) returns the supplied default.
fn color_to_aci(color: &dxf::Color, default: i32) -> i32 {
    if let Some(idx) = color.index() {
        i32::from(idx)
    } else {
        default
    }
}

/// 2D affine transform used to expand INSERT block references.
#[derive(Debug, Clone, Copy)]
struct Transform2D {
    /// Column-major 2x3 matrix: [a, b, tx; c, d, ty]
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: f64,
    ty: f64,
    rotation: f64,
}

impl Transform2D {
    fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
            rotation: 0.0,
        }
    }

    fn translation(tx: f64, ty: f64) -> Self {
        Self {
            tx,
            ty,
            ..Self::identity()
        }
    }

    fn insert(tx: f64, ty: f64, sx: f64, sy: f64, rot: f64) -> Self {
        let cos_r = rot.cos();
        let sin_r = rot.sin();
        Self {
            a: sx * cos_r,
            b: -sy * sin_r,
            c: sx * sin_r,
            d: sy * cos_r,
            tx,
            ty,
            rotation: rot,
        }
    }

    fn apply(&self, x: f64, y: f64) -> Point2 {
        Point2::new(
            self.a * x + self.b * y + self.tx,
            self.c * x + self.d * y + self.ty,
        )
    }

    fn compose(&self, other: &Self) -> Self {
        // self ∘ other: first apply `other`, then `self`.
        let a = self.a * other.a + self.b * other.c;
        let b = self.a * other.b + self.b * other.d;
        let c = self.c * other.a + self.d * other.c;
        let d = self.c * other.b + self.d * other.d;
        let tx = self.a * other.tx + self.b * other.ty + self.tx;
        let ty = self.c * other.tx + self.d * other.ty + self.ty;
        Self {
            a,
            b,
            c,
            d,
            tx,
            ty,
            rotation: self.rotation + other.rotation,
        }
    }

    fn vector(&self, p: Point2) -> Point2 {
        Point2::new(self.a * p.x + self.b * p.y, self.c * p.x + self.d * p.y)
    }
    fn orientation(&self) -> f64 {
        if self.a * self.d - self.b * self.c < 0.0 {
            -1.0
        } else {
            1.0
        }
    }
    fn is_similarity(&self) -> bool {
        let x = self.a * self.a + self.c * self.c;
        let y = self.b * self.b + self.d * self.d;
        let scale = x.max(y);
        scale.is_finite()
            && scale > 0.0
            && (x - y).abs() <= 1e-12 * scale
            && (self.a * self.b + self.c * self.d).abs() <= 1e-12 * scale
    }
    fn uniform_scale_factor(&self) -> f64 {
        // Legacy editable TEXT size approximation; curves use the full matrix.
        (self.a * self.d - self.b * self.c).abs().sqrt()
    }
}

/// OCS → parent coordinates for an entity normal, by the DXF Arbitrary Axis
/// Algorithm restricted to planes parallel to XY: for N = (0, 0, s),
/// Ax = Wy × N = (s, 0, 0) and Ay = N × Ax = (0, 1, 0), so +Z is the identity
/// and -Z mirrors X (which also reverses arc direction and bulges). `None` for a
/// zero, non-finite or tilted normal. Elevation (OCS Z) is ignored, as for
/// every 2D import.
fn ocs_transform(n: &dxf::Vector) -> Option<Transform2D> {
    let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
    if !len.is_finite() || len == 0.0 {
        return None;
    }
    if (n.x / len).hypot(n.y / len) > OCS_PARALLEL_TOL {
        return None;
    }
    Some(if n.z > 0.0 {
        Transform2D::identity()
    } else {
        Transform2D {
            a: -1.0,
            // The +X baseline maps to -X; legacy text rotation reads this.
            rotation: std::f64::consts::PI,
            ..Transform2D::identity()
        }
    })
}

/// dxf-rs surfaces line numbers in its `Display` text but not as a
/// structured field. Pull a "line N" hint when present so the frontend
/// can point at the offending line.
fn parse_error_span(filename: &str, err: &dxf::DxfError) -> Option<SourceSpan> {
    let s = err.to_string();
    let line = s
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|t| t.parse::<u32>().ok())
        .find(|n| *n > 0)?;
    Some(SourceSpan {
        file: filename.to_string(),
        line,
        column: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("refs/viaconstructor/tests/data");
        root.join(name)
    }

    /// Resolve a corpus fixture, or `None` when the gitignored `refs/`
    /// scaffolding isn't checked out (e.g. CI). Mirrors the golden_corpus
    /// smoke test: refs/ is optional dev scaffolding, so a checkout without
    /// it must skip — not fail. The skip is announced (visible under
    /// `cargo test -- --nocapture`) so a green run isn't mistaken for real
    /// coverage of the DXF corpus.
    fn fixture_or_skip(name: &str) -> Option<PathBuf> {
        let path = fixture(name);
        if !path.exists() {
            eprintln!(
                "dxf_in: SKIPPED — {} not present (refs/ is optional dev \
                 scaffolding; clone viaconstructor into refs/ to run this).",
                path.display()
            );
            return None;
        }
        Some(path)
    }

    #[test]
    fn imports_simple_dxf() {
        let Some(path) = fixture_or_skip("simple.dxf") else {
            return;
        };
        let opts = ImportOptions::default();
        let out = import_dxf_path(&path, &opts).expect("import");
        assert!(!out.segments.is_empty(), "should have segments");
        assert!(out.bbox.width() > 0.0);
        assert!(out.bbox.height() > 0.0);
        assert_eq!(out.format, "dxf");
    }

    #[test]
    fn imports_all_dxf_emits_text_entities() {
        // Phase 4: TEXT/MTEXT entities flow through ImportOutput
        // .text_entities (editable metadata) instead of being rendered
        // to opaque polylines. all.dxf contains a single "Via" TEXT
        // entity at the origin.
        let Some(path) = fixture_or_skip("all.dxf") else {
            return;
        };
        let opts = ImportOptions::default();
        let out = import_dxf_path(&path, &opts).expect("import");
        assert!(
            !out.text_entities.is_empty(),
            "all.dxf should expose at least one editable text entity"
        );
        let entity = &out.text_entities[0];
        assert!(
            !entity.text.is_empty(),
            "text entity should carry the original string"
        );
        assert!(entity.size_mm > 0.0, "size_mm must be positive");
    }

    #[test]
    fn bad_dxf_returns_structured_io_error() {
        let opts = ImportOptions::default();
        let err = import_dxf_bytes("garbage.dxf".into(), b"not a dxf file at all", &opts)
            .expect_err("bad dxf should error");
        assert!(
            matches!(
                err.kind,
                crate::errors::ErrorKind::BadInput | crate::errors::ErrorKind::Io
            ),
            "kind={:?}",
            err.kind
        );
        assert!(
            err.message.to_lowercase().contains("dxf"),
            "message={}",
            err.message
        );
    }
}

#[cfg(test)]
// Test fixtures assert exact literal values and bounded indices.
#[allow(clippy::float_cmp)]
mod audit_six {
    use super::*;
    #[test]
    fn hatch_metadata_is_complete() {
        let s=b"0\nSECTION\n2\nENTITIES\n0\nHATCH\n8\nTEST\n91\n1\n92\n2\n72\n0\n73\n1\n93\n4\n10\n0\n20\n0\n10\n10\n20\n0\n10\n10\n20\n10\n10\n0\n20\n10\n97\n0\n0\nENDSEC\n0\nEOF\n";
        let out = import_dxf_bytes("h.dxf".into(), s, &ImportOptions::default()).unwrap();
        assert_eq!(out.segments.len(), 4);
        assert_eq!(out.objects.len(), 4);
        assert_eq!(out.object_meta.len(), 1);
        let opts = ImportOptions {
            color_layers: true,
            select_layers: vec!["TEST-c7".into()],
            ..Default::default()
        };
        let colored = import_dxf_bytes("h.dxf".into(), s, &opts).unwrap();
        assert_eq!(colored.segments.len(), 4);
        assert!(colored
            .segments
            .iter()
            .all(|s| s.layer.as_ref() == "TEST-c7" && s.color == 7));
        let opts = ImportOptions {
            select_layers: vec!["OTHER".into()],
            ..Default::default()
        };
        assert!(import_dxf_bytes("h.dxf".into(), s, &opts)
            .unwrap()
            .segments
            .is_empty());
    }
    #[test]
    fn mirrored_arc_follows_affine_endpoints() {
        let mut d = Drawing::new();
        let mut block = dxf::Block::default();
        block.name = "B".into();
        block.entities.push(Entity::new(EntityType::Arc(Arc {
            center: dxf::Point::new(0., 0., 0.),
            radius: 10.,
            start_angle: 0.,
            end_angle: 90.,
            ..Default::default()
        })));
        d.add_block(block);
        d.add_entity(Entity::new(EntityType::Insert(dxf::entities::Insert {
            name: "B".into(),
            x_scale_factor: -1.,
            y_scale_factor: 1.,
            ..Default::default()
        })));
        let out = import_drawing(&d, "a".into(), &ImportOptions::default()).unwrap();
        assert!(out.segments[0].start.distance(Point2::new(-10., 0.)) < 1e-9);
        assert!(out.segments.iter().all(|s| s.bulge < 0.));
    }
    fn transformed(entity: EntityType, xf: Transform2D) -> ImportOutput {
        let opts = ImportOptions::default();
        let mut warnings = vec![];
        let colors = BTreeMap::new();
        let mut ctx = ImportCtx {
            opts: &opts,
            unit_scale: 1.0,
            warnings: &mut warnings,
            segments: vec![],
            layer_colors: &colors,
            select: None,
            cam_setup: String::new(),
            curve_error: false,
            refusal: None,
            text_entities: vec![],
            layer_intern: crate::input::intern::LayerIntern::new(),
        };
        ctx.add_entity(&Entity::new(entity), &HashMap::new(), &xf, 0);
        assert!(!ctx.curve_error);
        let segments = ctx.segments;
        let (objects, object_meta) = super::super::object_index(&segments);
        ImportOutput {
            filename: String::new(),
            format: "dxf".into(),
            bbox: BBox::from_segments(&segments),
            layers: vec![],
            unit_scale: 1.0,
            warnings,
            segments,
            objects,
            object_meta,
            text_entities: vec![],
            unit_status: crate::input::ImportUnitStatus::Declared,
            unit_source: String::new(),
        }
    }
    fn chord_distance(p: Point2, s: &Segment) -> f64 {
        let dx = s.end.x - s.start.x;
        let dy = s.end.y - s.start.y;
        let t =
            (((p.x - s.start.x) * dx + (p.y - s.start.y) * dy) / (dx * dx + dy * dy)).clamp(0., 1.);
        p.distance(Point2::new(s.start.x + t * dx, s.start.y + t * dy))
    }
    #[test]
    fn nested_nonuniform_circle_and_ellipse_follow_full_matrix() {
        let xf = Transform2D::insert(13., -7., -2., 1., 0.4)
            .compose(&Transform2D::insert(2., 3., 1., 3., 0.7));
        for (major, minor, entity) in [
            (
                Point2::new(10., 0.),
                Point2::new(0., 10.),
                EntityType::Circle(Circle {
                    radius: 10.,
                    ..Default::default()
                }),
            ),
            (
                Point2::new(8., 6.),
                Point2::new(-3., 4.),
                EntityType::Ellipse(Ellipse {
                    major_axis: dxf::Vector::new(8., 6., 0.),
                    minor_axis_ratio: 0.5,
                    start_parameter: 0.,
                    end_parameter: std::f64::consts::TAU,
                    ..Default::default()
                }),
            ),
        ] {
            let out = transformed(entity, xf);
            assert!(out.segments.len() > 8);
            assert_eq!(out.segments[0].start, out.segments.last().unwrap().end);
            assert!(out.segments.iter().all(|s| s.bulge == 0.));
            for i in 0..=2000 {
                let t = std::f64::consts::TAU * f64::from(i) / 2000.;
                let p = xf.apply(
                    major.x * t.cos() + minor.x * t.sin(),
                    major.y * t.cos() + minor.y * t.sin(),
                );
                let dist = out
                    .segments
                    .iter()
                    .map(|s| chord_distance(p, s))
                    .fold(f64::INFINITY, f64::min);
                assert!(dist <= 0.010_000_01, "error {dist}");
            }
        }
    }
    #[test]
    fn mirrored_and_stretched_polyline_bulges() {
        for sx in [-1., -2.] {
            let p = LwPolyline {
                vertices: vec![
                    dxf::LwPolylineVertex {
                        x: 0.,
                        y: 0.,
                        bulge: 1.,
                        ..Default::default()
                    },
                    dxf::LwPolylineVertex {
                        x: 0.,
                        y: 10.,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            };
            let out = transformed(
                EntityType::LwPolyline(p),
                Transform2D::insert(0., 0., sx, 1., 0.),
            );
            assert!(out
                .segments
                .iter()
                .all(|s| s.start.x <= 1e-9 && s.end.x <= 1e-9));
            assert_eq!(out.segments[0].start, Point2::new(0., 0.));
            assert_eq!(out.segments.last().unwrap().end, Point2::new(0., 10.));
            if sx == -1. {
                assert!(out.segments.iter().all(|s| s.bulge < 0.));
            } else {
                assert!(out.segments.iter().all(|s| s.bulge == 0.));
            }
        }
    }
    #[test]
    fn text_in_negative_z_ocs_keeps_position_and_baseline() {
        let text = |normal: dxf::Vector| {
            let mut d = Drawing::new();
            d.add_entity(Entity::new(EntityType::Text(Text {
                location: dxf::Point::new(5., 5., 0.),
                text_height: 2.,
                value: "A".into(),
                rotation: 30.,
                normal,
                ..Default::default()
            })));
            let out = import_drawing(&d, "t.dxf".into(), &ImportOptions::default()).unwrap();
            let t = out.text_entities[0].clone();
            (t.origin, t.rotation_deg)
        };
        let (origin, rot) = text(dxf::Vector::z_axis());
        assert_eq!(origin, (5., 5.));
        assert!((rot - 30.).abs() < 1e-9);
        // OCS -Z mirrors X: (5, 5) -> (-5, 5); the 30° baseline points at 150°.
        let (origin, rot) = text(dxf::Vector::new(0., 0., -1.));
        assert_eq!(origin, (-5., 5.));
        assert!((rot - 150.).abs() < 1e-9, "{rot}");
    }

    #[test]
    fn invalid_spline_has_visible_fallback_diagnostic() {
        let mut d = Drawing::new();
        d.add_entity(Entity::new(EntityType::Spline(Spline {
            degree_of_curve: 3,
            control_points: vec![dxf::Point::new(0., 0., 0.), dxf::Point::new(10., 10., 0.)],
            ..Default::default()
        })));
        let out = import_drawing(&d, "s.dxf".into(), &ImportOptions::default()).unwrap();
        assert_eq!(out.segments.len(), 1);
        assert!(out.warnings.iter().any(|w| w.contains("control polygon")));
    }
    #[test]
    fn impossible_curve_budget_refuses_drawing() {
        let mut d = Drawing::new();
        d.add_entity(Entity::new(EntityType::Ellipse(Ellipse {
            major_axis: dxf::Vector::new(1e20, 0., 0.),
            minor_axis_ratio: 0.5,
            start_parameter: 0.,
            end_parameter: std::f64::consts::TAU,
            ..Default::default()
        })));
        assert!(import_drawing(&d, "huge.dxf".into(), &ImportOptions::default()).is_err());
    }
}
