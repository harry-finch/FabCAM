//! Current private collection documents. No caller-supplied filesystem paths.
use ivac_core::presets::{parse_collection, safe_id, MAX_COLLECTION_BYTES};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCollection {
    pub json: String,
    pub token: String,
}
fn token(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
fn prepare(dir: &Path) -> Result<(), String> {
    for p in dir.ancestors() {
        if let Ok(m) = std::fs::symlink_metadata(p) {
            if m.file_type().is_symlink() {
                return Err("Collection directory cannot contain symlinks".into());
            }
        }
    }
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())
}
fn root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("presets")
        .join("collections");
    prepare(&dir)?;
    Ok(dir)
}
fn path(dir: &Path, id: &str) -> Result<PathBuf, String> {
    if !safe_id(id) {
        return Err("Invalid collection ID".into());
    }
    Ok(dir.join(format!("{id}.json")))
}
fn read(path: &Path) -> Result<Option<StoredCollection>, String> {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > MAX_COLLECTION_BYTES as u64
    {
        return Err("Invalid or oversized collection file".into());
    }
    let mut json = String::new();
    std::fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX_COLLECTION_BYTES as u64 + 1)
        .read_to_string(&mut json)
        .map_err(|e| e.to_string())?;
    parse_collection(&json)?;
    Ok(Some(StoredCollection {
        token: token(&json),
        json,
    }))
}
struct Lock(PathBuf);
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn lock(dir: &Path) -> Result<Lock, String> {
    let p = dir.join(".write-lock");
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&p)
        .map_err(|_| {
            "Collection storage is busy; retry after the other write finishes".to_string()
        })?;
    Ok(Lock(p))
}
fn write(dir: &Path, json: &str, expected: Option<&str>) -> Result<StoredCollection, String> {
    let collection = parse_collection(json)?;
    prepare(dir)?;
    let _lock = lock(dir)?;
    let path = path(dir, &collection.id)?;
    let current = read(&path)?;
    if current.as_ref().map(|c| c.token.as_str()) != expected {
        return Err("Collection changed since it was opened; reload before replacing".into());
    }
    if current.is_none()
        && std::fs::read_dir(dir)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|v| v == "json"))
            .count()
            >= 32
    {
        return Err("Maximum 32 collections".into());
    }
    let json = serde_json::to_string(&collection).map_err(|e| e.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(dir).map_err(|e| e.to_string())?;
    temp.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(&path).map_err(|e| e.error.to_string())?;
    Ok(StoredCollection {
        token: token(&json),
        json,
    })
}
fn remove(dir: &Path, id: &str, expected: &str) -> Result<(), String> {
    prepare(dir)?;
    let _lock = lock(dir)?;
    let p = path(dir, id)?;
    if read(&p)?.as_ref().map(|c| c.token.as_str()) != Some(expected) {
        return Err("Collection changed; reload before deleting".into());
    }
    std::fs::remove_file(p).map_err(|e| e.to_string())
}
#[tauri::command]
// Tauri command arguments must be owned transport values.
#[allow(clippy::needless_pass_by_value)]
pub fn list_preset_collections(app: tauri::AppHandle) -> Result<Vec<StoredCollection>, String> {
    let dir = root(&app)?;
    let mut paths = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    let mut out = vec![];
    for p in paths {
        if p.extension().is_some_and(|x| x == "json") {
            if out.len() >= 32 {
                return Err("Collection count exceeds limit".into());
            }
            if let Some(c) = read(&p)? {
                out.push(c);
            }
        }
    }
    Ok(out)
}
#[tauri::command]
// Tauri command arguments must be owned transport values.
#[allow(clippy::needless_pass_by_value)]
pub fn save_preset_collection(
    app: tauri::AppHandle,
    json: String,
    expected: Option<String>,
) -> Result<StoredCollection, String> {
    write(&root(&app)?, &json, expected.as_deref())
}
#[tauri::command]
// Tauri command arguments must be owned transport values.
#[allow(clippy::needless_pass_by_value)]
pub fn delete_preset_collection(
    app: tauri::AppHandle,
    id: String,
    expected: String,
) -> Result<(), String> {
    remove(&root(&app)?, &id, &expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qual1a2_numeric_reopen_preserves_original_token_until_explicit_save() {
        let d = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let original = r#"{"format":"fabcam-preset-collection","version":1,"id":"numeric","name":"Numeric","presets":[{"id":"p","name":"P","context":{"machine_id":"9007199254740993","machine_label":"M","material_id":"18446744073709551615","material_label":"S","signature":"{}"},"parameters":{"rates":[{"role":"rough","tool_id":1,"rpm":10000,"feed_mm_min":600,"plunge_mm_min":200}],"schedule":{"kind":"step","step":-15.1227061426145610,"finish_step":null},"pocket_overlap":null,"planar_stepover_mm":null},"tested":true,"test_date":null,"notes":null}]}"#;
        let path = d.path().join("numeric.json");
        std::fs::write(&path, original).unwrap();
        let reopened = read(&path).unwrap().unwrap();
        assert_eq!(reopened.token, token(original));
        assert_eq!(reopened.json, original);
        let c = parse_collection(original).unwrap();
        assert_eq!(c.presets[0].context.machine_id, "9007199254740993");
        assert_eq!(c.presets[0].context.material_id, "18446744073709551615");
        let ivac_core::presets::DepthSchedule::Step { step, .. } = c.presets[0].parameters.schedule
        else {
            panic!("step")
        };
        assert_eq!(step.unwrap().to_bits(), 0xc02e_3ed3_56eb_1880);
        let saved = write(d.path(), original, Some(&reopened.token)).unwrap();
        assert_ne!(saved.token, reopened.token); // Noncanonical trailing zero removed on explicit save.
        assert_eq!(read(&path).unwrap().unwrap().token, saved.token);
        assert!(write(d.path(), original, Some(&reopened.token)).is_err());
        assert_eq!(std::fs::read_to_string(path).unwrap(), saved.json);
        let canonical = write(d.path(), &saved.json, Some(&saved.token)).unwrap();
        assert_eq!(canonical.token, saved.token); // Identical canonical bytes keep their token.
    }
    fn doc(name: &str) -> String {
        serde_json::json!({"format":"fabcam-preset-collection","version":1,"id":"lab","name":name,"presets":[]}).to_string()
    }
    #[test]
    fn atomic_current_document_and_conflicts() {
        let d = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let a = write(d.path(), &doc("A"), None).unwrap();
        assert!(write(d.path(), &doc("B"), None).is_err());
        let b = write(d.path(), &doc("B"), Some(&a.token)).unwrap();
        assert_ne!(a.token, b.token);
        assert!(write(d.path(), &doc("C"), Some(&a.token)).is_err());
        assert_eq!(
            read(&d.path().join("lab.json")).unwrap().unwrap().json,
            b.json
        );
        assert!(remove(d.path(), "lab", &a.token).is_err());
        remove(d.path(), "lab", &b.token).unwrap();
    }
    #[test]
    fn malformed_oversized_busy_and_traversal_preserve_original() {
        let d = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let a = write(d.path(), &doc("A"), None).unwrap();
        assert!(write(d.path(), "{}", Some(&a.token)).is_err());
        assert!(write(
            d.path(),
            &" ".repeat(MAX_COLLECTION_BYTES + 1),
            Some(&a.token)
        )
        .is_err());
        assert!(remove(d.path(), "../lab", &a.token).is_err());
        let _l = lock(d.path()).unwrap();
        assert!(write(d.path(), &doc("B"), Some(&a.token)).is_err());
        assert_eq!(
            read(&d.path().join("lab.json")).unwrap().unwrap().token,
            a.token
        );
    }
    #[cfg(unix)]
    #[test]
    fn symlink_destination_and_directory_refused() {
        let d = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let target = d.path().join("user.json");
        std::fs::write(&target, doc("Original")).unwrap();
        std::os::unix::fs::symlink(&target, d.path().join("lab.json")).unwrap();
        assert!(write(d.path(), &doc("Changed"), None).is_err());
        let link = d.path().join("linked");
        std::os::unix::fs::symlink(d.path(), &link).unwrap();
        assert!(write(&link, &doc("Changed"), None).is_err());
        assert_eq!(std::fs::read_to_string(target).unwrap(), doc("Original"));
    }
}
