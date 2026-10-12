//! Private app-data storage; never repository/resources/network. JSON only.
use std::path::{Path, PathBuf};
use tauri::Manager;

fn root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let dir = base.join("feeds-speeds").join("sources");
    prepare_directory(&dir)?;
    Ok(dir)
}
fn prepare_directory(dir: &Path) -> Result<(), String> {
    if dir.ancestors().any(|p| p.join(".git").exists()) {
        return Err("Private sources cannot be stored in a Git working tree".into());
    }
    for p in dir.ancestors() {
        if p.exists()
            && std::fs::symlink_metadata(p)
                .map_err(|e| e.to_string())?
                .file_type()
                .is_symlink()
        {
            return Err("Source directory cannot be a symlink".into());
        }
    }
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())
}
fn store(dir: &Path, json: &str) -> Result<String, String> {
    prepare_directory(dir)?;
    let pack = ivac_core::feeds_speeds::validate_source_pack_json(json)?;
    let _lock = if pack.manifest.identity.id == "sorbonne-fablab" {
        Some(lab_write_lock(dir)?)
    } else {
        None
    };
    store_locked(dir, json)
}
fn store_locked(dir: &Path, json: &str) -> Result<String, String> {
    prepare_directory(dir)?;
    let mut pack = ivac_core::feeds_speeds::validate_source_pack_json(json)?;
    // Import is always local-only. A manifest cannot add itself to the build allowlist.
    pack.manifest.distribution =
        serde_json::from_str("\"local_only\"").map_err(|e| e.to_string())?;
    let paths = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let names: Vec<_> = paths
        .iter()
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let lab = pack.manifest.identity.id == "sorbonne-fablab";
    let mut ids = names
        .iter()
        .map(|n| n.split('@').next().unwrap_or(n).to_string())
        .collect::<std::collections::BTreeSet<_>>();
    ids.insert(pack.manifest.identity.id.clone());
    if ids.len() > 32 {
        return Err("At most 32 distinct local source packs supported".into());
    }
    if names.len() >= 512
        || (!lab
            && names
                .iter()
                .filter(|n| !n.starts_with("sorbonne-fablab@"))
                .count()
                >= 32)
    {
        return Err("Local source storage revision limit reached; export a backup before managing old revisions".into());
    }
    if lab
        && names
            .iter()
            .filter_map(|n| {
                n.strip_prefix("sorbonne-fablab@")
                    .and_then(|n| n.strip_suffix(".json"))
            })
            .any(|v| v >= pack.manifest.identity.revision.as_str())
    {
        return Err(
            "This or a newer Sorbonne revision is already installed; current knowledge was kept"
                .into(),
        );
    }
    let name = format!(
        "{}@{}.json",
        pack.manifest.identity.id, pack.manifest.identity.revision
    );
    let path = dir.join(name);
    let text = serde_json::to_string(&pack).map_err(|e| e.to_string())?;
    // Immutable versions: don't replace an existing pack or follow a symlink.
    use std::io::Write;
    let mut options = std::fs::OpenOptions::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    Ok(text)
}
#[tauri::command]
pub fn import_source_pack(app: tauri::AppHandle, json: String) -> Result<String, String> {
    store(&root(&app)?, &json)
}
#[tauri::command]
pub fn list_source_packs(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let dir = root(&app)?;
    let mut rows = Vec::new();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    paths.sort_by_key(|p| p.file_name());
    let lab_head = paths
        .iter()
        .filter(|p| {
            p.file_name()
                .to_string_lossy()
                .starts_with("sorbonne-fablab@")
                && p.path().extension().is_some_and(|v| v == "json")
        })
        .map(|p| p.file_name())
        .max();
    for entry in paths {
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with("sorbonne-fablab@")
            && lab_head
                .as_ref()
                .is_some_and(|head| head != &entry.file_name())
        {
            continue;
        }
        let path = entry.path();
        if path.extension().map_or(true, |v| v != "json") {
            continue;
        }
        let meta = entry.file_type().map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.is_symlink() {
            return Err("Unexpected source file type".into());
        }
        if entry.metadata().map_err(|e| e.to_string())?.len() > 2 * 1024 * 1024 {
            return Err("Source pack exceeds 2 MiB".into());
        }
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let pack = ivac_core::feeds_speeds::validate_source_pack_json(&json)?;
        if serde_json::to_value(pack.manifest.distribution).map_err(|e| e.to_string())?
            != "local_only"
        {
            return Err("App-data pack must be local-only".into());
        }
        rows.push(json);
        if rows.len() > 512 {
            return Err("At most 512 installed source revisions supported".into());
        }
    }
    Ok(rows)
}
struct LabWriteLock(std::path::PathBuf);
impl Drop for LabWriteLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn lab_write_lock(dir: &Path) -> Result<LabWriteLock, String> {
    let lock_path = dir.join(".sorbonne-write-lock");
    let _file = std::fs::OpenOptions::new().write(true).create_new(true).open(&lock_path).map_err(|_| "Another Sorbonne save is in progress; retry. If the application crashed, close all instances before removing its .sorbonne-write-lock marker".to_string())?;
    Ok(LabWriteLock(lock_path))
}
fn save_lab(dir: &Path, json: &str, expected: Option<String>) -> Result<String, String> {
    prepare_directory(dir)?;
    let _lock = lab_write_lock(dir)?;
    let pack = ivac_core::feeds_speeds::validate_source_pack_json(json)?;
    if pack.manifest.identity.id != "sorbonne-fablab"
        || serde_json::to_value(pack.manifest.visibility).map_err(|e| e.to_string())? != "private"
        || serde_json::to_value(pack.manifest.distribution).map_err(|e| e.to_string())?
            != "local_only"
    {
        return Err("Only private Sorbonne presets can be authored".into());
    }
    let mut candidates = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    candidates.retain(|e| {
        e.file_name()
            .to_string_lossy()
            .starts_with("sorbonne-fablab@")
            && e.path().extension().is_some_and(|v| v == "json")
    });
    candidates.sort_by_key(|e| e.file_name());
    let head = if let Some(entry) = candidates.last() {
        let meta = std::fs::symlink_metadata(entry.path()).map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > 2 * 1024 * 1024 {
            return Err("Unexpected local revision file".into());
        }
        Some(
            ivac_core::feeds_speeds::validate_source_pack_json(
                &std::fs::read_to_string(entry.path()).map_err(|e| e.to_string())?,
            )?
            .manifest
            .identity
            .revision,
        )
    } else {
        None
    };
    if head != expected
        || head
            .as_ref()
            .is_some_and(|v| v >= &pack.manifest.identity.revision)
    {
        return Err("Sorbonne presets changed; reopen and retry".into());
    }
    store_locked(dir, json)
}
#[tauri::command]
pub fn save_lab_pack(
    app: tauri::AppHandle,
    json: String,
    expected_revision: Option<String>,
) -> Result<String, String> {
    save_lab(&root(&app)?, &json, expected_revision)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lab_revisions_use_compare_and_append_not_overwrite() {
        let dir = std::env::temp_dir()
            .canonicalize()
            .unwrap()
            .join(format!("fs38-cas-{}", std::process::id()));
        let mut pack: serde_json::Value = serde_json::from_str(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        pack["manifest"]["identity"]["id"] = serde_json::json!("sorbonne-fablab");
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000000");
        pack["manifest"]["visibility"] = serde_json::json!("private");
        pack["manifest"]["distribution"] = serde_json::json!("local_only");
        pack["records"] = serde_json::json!([]);
        pack["dataset"]["profile_set"]["profiles"] = serde_json::json!([]);
        save_lab(&dir, &pack.to_string(), None).unwrap();
        assert!(save_lab(&dir, &pack.to_string(), None).is_err());
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000001");
        assert!(save_lab(&dir, &pack.to_string(), None).is_err());
        save_lab(&dir, &pack.to_string(), Some("0001791792000000".into())).unwrap();
        assert!(dir.join("sorbonne-fablab@0001791792000000.json").exists());
        assert!(dir.join("sorbonne-fablab@0001791792000001.json").exists());
        assert!(!dir.join(".sorbonne-write-lock").exists());
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000002");
        let lock = lab_write_lock(&dir).unwrap();
        assert!(store(&dir, &pack.to_string()).is_err());
        drop(lock);
        store(&dir, &pack.to_string()).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn stores_only_valid_local_json_and_refuses_duplicate_version() {
        let dir = std::env::temp_dir()
            .canonicalize()
            .unwrap()
            .join(format!("fs37-storage-{}", std::process::id()));
        let json = include_str!("../../../data/feeds-speeds/packs/sienci-longmill.v1.json");
        let text = store(&dir, json).unwrap();
        assert!(text.contains("local_only"));
        assert!(store(&dir, json).is_err());
        assert!(store(&dir, "{}").is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn rejects_repository_storage() {
        assert!(prepare_directory(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("private-source-test")
                .as_path()
        )
        .is_err());
    }
}
