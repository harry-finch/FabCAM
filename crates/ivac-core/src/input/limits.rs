//! Product admission bounds before library parsing and geometric expansion.
use crate::{Error, Result};
use std::io::Read;
pub const MAX_INPUT_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_PRIMITIVES: usize = 200_000;
pub fn check_bytes(bytes: &[u8]) -> Result<()> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(Error::limit("Import exceeds the 64 MiB input budget"));
    }
    Ok(())
}
pub fn read(path: &std::path::Path) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    check_bytes(&bytes)?;
    Ok(bytes)
}
/// Decompress with a hard output cap; never call usvg's unbounded SVGZ reader.
pub fn svg(bytes: &[u8]) -> Result<Vec<u8>> {
    check_bytes(bytes)?;
    const MAX_XML: usize = 16 * 1024 * 1024;
    let mut data = Vec::new();
    if bytes.starts_with(&[0x1f, 0x8b]) {
        flate2::read::MultiGzDecoder::new(bytes)
            .take((MAX_XML + 1) as u64)
            .read_to_end(&mut data)
            .map_err(|e| Error::bad_input(format!("SVG compression: {e}")))?;
    } else {
        if bytes.len() > MAX_XML {
            return Err(Error::limit("SVG exceeds the 16 MiB XML budget"));
        }
        data.extend_from_slice(bytes);
    }
    if data.len() > MAX_XML {
        return Err(Error::limit(
            "SVG decompression exceeds the 16 MiB XML budget",
        ));
    }
    let text = std::str::from_utf8(&data).map_err(|e| Error::bad_input(e.to_string()))?;
    let doc = usvg::roxmltree::Document::parse_with_options(
        text,
        usvg::roxmltree::ParsingOptions {
            nodes_limit: 50_000,
            ..Default::default()
        },
    )
    .map_err(|e| Error::bad_input(format!("SVG admission: {e}")))?;
    let mut ids = std::collections::BTreeMap::new();
    for node in doc.descendants() {
        if let Some(id) = node.attribute("id") {
            if ids.insert(id, node).is_some() {
                return Err(Error::bad_input(
                    "SVG duplicate IDs are ambiguous; use unique IDs",
                ));
            }
        }
        let href = node.attribute("href");
        let xlink = node.attribute(("http://www.w3.org/1999/xlink", "href"));
        if href.is_some() && xlink.is_some() && href != xlink {
            return Err(Error::bad_input(
                "SVG conflicting href/xlink:href is ambiguous",
            ));
        }
    }
    let mut work = 0usize;
    for node in doc.descendants().filter(usvg::roxmltree::Node::is_element) {
        if node.ancestors().take(66).count() > 64 {
            return Err(Error::limit("SVG nesting exceeds 64 levels"));
        }
        charge(node, &mut work)?;
        if node.tag_name().name() == "use" {
            let href = node
                .attribute("href")
                .or_else(|| node.attribute(("http://www.w3.org/1999/xlink", "href")))
                .unwrap_or("");
            let id = href
                .strip_prefix('#')
                .ok_or_else(|| Error::unsupported("SVG use requires a local fragment"))?;
            let target = ids
                .get(id)
                .copied()
                .ok_or_else(|| Error::bad_input("SVG use target missing"))?;
            if target
                .descendants()
                .any(|n| n.is_element() && n.tag_name().name() == "use")
            {
                return Err(Error::limit("Nested or recursive SVG use expansion is not supported; flatten references first"));
            }
            // Include path-data work for each cloned subtree, not just node count.
            for child in target.descendants() {
                charge(child, &mut work)?;
            }
        }
    }
    Ok(data)
}

/// Count shapes and numeric point/path work before usvg clones referenced trees.
fn charge(node: usvg::roxmltree::Node<'_, '_>, work: &mut usize) -> Result<()> {
    if node.is_element() && node.tag_name().name() == "marker" {
        return Err(Error::unsupported(
            "SVG marker expansion is not budget-qualified; convert markers to explicit paths first",
        ));
    }
    *work = work.saturating_add(16);
    if let Some(d) = node.attribute("d") {
        for item in svgtypes::PathParser::from(d) {
            item.map_err(|e| Error::bad_input(format!("SVG path: {e}")))?;
            *work = work.saturating_add(1);
            if *work > MAX_PRIMITIVES {
                return Err(Error::limit("SVG expanded primitive budget exceeded"));
            }
        }
    }
    if let Some(points) = node.attribute("points") {
        for _ in svgtypes::PointsParser::from(points) {
            *work = work.saturating_add(1);
            if *work > MAX_PRIMITIVES {
                return Err(Error::limit("SVG expanded points budget exceeded"));
            }
        }
    }
    if *work > MAX_PRIMITIVES {
        return Err(Error::limit("SVG expanded node budget exceeded"));
    }
    Ok(())
}
