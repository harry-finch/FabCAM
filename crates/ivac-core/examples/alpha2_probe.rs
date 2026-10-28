//! Read-only diagnostic probe. Reports indices/gaps, never source coordinates.
//! Not a repair implementation or a machining qualification.
use ivac_core::drawing::{Drawing, Source, Transform};
use ivac_core::geometry::BBox;
fn main() {
    let path = std::env::args().nth(1).expect("DXF path required");
    let bytes = std::fs::read(&path).unwrap();
    let input = ivac_core::input::import_bytes(
        "private.dxf",
        &bytes,
        &ivac_core::input::ImportOptions::default(),
    )
    .unwrap();
    if let Some(output) = std::env::args().nth(2) {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output)
            .unwrap();
        f.write_all(&serde_json::to_vec(&input).unwrap()).unwrap();
    }
    let fuzzy_closed = input.object_meta.iter().filter(|c| c.closed).count();
    let d = Drawing::from_sources(vec![Source {
        id: 1,
        original: input.segments,
        legacy_objects: input.objects,
        pivot: ivac_core::Point2::new(0., 0.),
        transform: Transform::default(),
        next_entity: 1,
    }])
    .unwrap();
    let evaluated = d.evaluate().unwrap();
    println!(
        "segments={} contours={} fuzzy_closed={} exact_closed={}",
        evaluated.segments.len(),
        d.contours.len(),
        fuzzy_closed,
        evaluated.closed.iter().filter(|x| **x).count()
    );
    println!("global_validation={:?}", d.validate_for_generation());
    for (ci, c) in d.contours.iter().enumerate() {
        let max_gap = c
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| {
                e.geometry
                    .end
                    .distance(c.edges[(i + 1) % c.edges.len()].geometry.start)
            })
            .fold(0f64, f64::max);
        if !evaluated.closed[ci] {
            println!(
                "gap contour_index={} identity={:?} max_mm={:.12}",
                ci + 1,
                c.id,
                max_gap
            );
        }
    }
    // Preserve each original primitive, transform and pair adjacency. Only this
    // diagnostic clone receives a fresh contour identity for nonadjacent edges.
    // Existing complete validator remains the predicate oracle.
    let edges: Vec<_> = d
        .contours
        .iter()
        .enumerate()
        .flat_map(|(ci, c)| c.edges.iter().enumerate().map(move |(ei, e)| (ci, ei, e)))
        .collect();
    let bounds: Vec<_> = evaluated
        .segments
        .iter()
        .map(|s| BBox::from_segments(std::slice::from_ref(s)))
        .collect();
    let mut failures = 0;
    for i in 0..edges.len() {
        for j in i + 1..edges.len() {
            let a = bounds[i];
            let b = bounds[j];
            if a.max_x < b.min_x || b.max_x < a.min_x || a.max_y < b.min_y || b.max_y < a.min_y {
                continue;
            }
            let (ci, ei, e) = edges[i];
            let (cj, ej, f) = edges[j];
            let adjacent = ci == cj
                && (ei.abs_diff(ej) == 1 || (ei == 0 && ej + 1 == d.contours[ci].edges.len()));
            let mut pair = d.clone();
            pair.contours.clear();
            let mut first = d.contours[ci].clone();
            first.edges = vec![e.clone()];
            if adjacent {
                first.edges.push(f.clone());
                pair.contours.push(first);
            } else {
                pair.contours.push(first);
                let mut second = d.contours[cj].clone();
                second.edges = vec![f.clone()];
                if ci == cj {
                    second.id.entity = pair.imports[0].next_entity;
                    pair.imports[0].next_entity += 1;
                }
                pair.contours.push(second);
            }
            if let Err(reason) = pair.validate_for_generation() {
                failures += 1;
                if failures <= 40 {
                    println!("pair contours={}/{} edges={:?}/{:?} ordinals={:?}/{:?} adjacent={} reason={}",ci+1,cj+1,e.id,f.id,e.original_indices,f.original_indices,adjacent,reason);
                }
            }
        }
    }
    println!("characterized_pair_failures={failures}; printed_limit=40");
    let report =
        ivac_core::drawing::diagnostics::analyze(&d, &ivac_core::cancel::CancelToken::new());
    println!(
        "v2_complete={} issues={}",
        report.complete,
        report.issues.len()
    );
    for f in d.analyze(0.001).unwrap() {
        let Some(ivac_core::drawing::repair::Repair::Gap { contour, edge, .. }) = f.repair else {
            continue;
        };
        for connector in [false, true] {
            use ivac_core::drawing::scoped::{RepairContext, ScopedPreview, ScopedRequest};
            let result = ivac_core::drawing::scoped::evaluate(
                ScopedRequest::Preview {
                    version: 2,
                    input: Box::new(ScopedPreview {
                        context: RepairContext {
                            drawing: d.clone(),
                            project: ivac_core::project::Project {
                                segments: evaluated.segments.clone(),
                                ..ivac_core::project::Project::default()
                            },
                            bindings: vec![],
                            document: serde_json::json!({"probe":true}),
                        },
                        repairs: vec![ivac_core::drawing::repair::Repair::Gap {
                            contour,
                            edge,
                            connector,
                            tolerance_mm: 0.001,
                        }],
                    }),
                },
                &ivac_core::cancel::CancelToken::new(),
            );
            match result {
                Ok(r) => println!(
                    "v2_gap {:?} connector={connector}: local accepted; full_CAM={:?}",
                    contour,
                    r.geometry.drawing.validate_for_generation()
                ),
                Err(reason) => {
                    println!("v2_gap {contour:?} connector={connector}: refused: {reason}");
                }
            }
        }
    }
}
