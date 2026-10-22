//! Private app-data storage; never repository/resources/network. JSON only.
use ivac_core::feeds_speeds::is_lab_pack_id;
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
    let _lock = source_write_lock(dir, &pack.manifest.identity.id)?;
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
    let lab = is_lab_pack_id(&pack.manifest.identity.id);
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
                .filter(|n| !is_lab_pack_id(n.split('@').next().unwrap_or(n)))
                .count()
                >= 32)
    {
        return Err("Local source storage revision limit reached; export a backup before managing old revisions".into());
    }
    if lab
        && names
            .iter()
            .filter_map(|n| {
                n.strip_prefix(&format!("{}@", pack.manifest.identity.id))
                    .and_then(|n| n.strip_suffix(".json"))
            })
            .any(|v| v >= pack.manifest.identity.revision.as_str())
    {
        return Err(
            "This or a newer preset pack revision is already installed; current knowledge was kept"
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
// owned argument is the transport/command contract.
#[allow(clippy::needless_pass_by_value)]
pub fn import_source_pack(app: tauri::AppHandle, json: String) -> Result<String, String> {
    store(&root(&app)?, &json)
}
#[tauri::command]
// owned argument is the transport/command contract.
#[allow(clippy::needless_pass_by_value)]
pub fn list_source_packs(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    list_local_packs(&root(&app)?)
}
fn list_local_packs(dir: &Path) -> Result<Vec<String>, String> {
    let mut rows = Vec::new();
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    paths.sort_by_key(std::fs::DirEntry::file_name);
    let mut lab_heads = std::collections::BTreeMap::new();
    for entry in &paths {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some((id, _)) = name.split_once('@') {
            if is_lab_pack_id(id)
                && Path::new(&name)
                    .extension()
                    .is_some_and(|ext| ext == "json")
            {
                lab_heads.insert(id.to_string(), name);
            }
        }
    }
    for entry in paths {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some((id, _)) = name.split_once('@') {
            if is_lab_pack_id(id) && lab_heads.get(id).is_some_and(|head| head != &name) {
                continue;
            }
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
#[cfg(test)]
fn lab_write_lock(dir: &Path) -> Result<LabWriteLock, String> {
    source_write_lock(dir, "sorbonne-fablab")
}
fn source_write_lock(dir: &Path, id: &str) -> Result<LabWriteLock, String> {
    if !ivac_core::feeds_speeds::safe_pack_id(id) {
        return Err("Invalid source id".into());
    }
    let lock_path = dir.join(if id == "sorbonne-fablab" {
        ".sorbonne-write-lock".to_string()
    } else {
        format!(".{id}-write-lock")
    });
    let _file = std::fs::OpenOptions::new().write(true).create_new(true).open(&lock_path).map_err(|_| "Another source change is in progress; retry. If the application crashed, close all instances before removing the source write-lock marker".to_string())?;
    Ok(LabWriteLock(lock_path))
}
// owned argument is the transport/command contract.
#[allow(clippy::needless_pass_by_value)]
fn save_lab(dir: &Path, json: &str, expected: Option<String>) -> Result<String, String> {
    save_lab_guarded(dir, json, expected, None)
}
#[derive(serde::Deserialize)]
pub struct SourceRevision {
    id: String,
    revision: String,
}
// Retained legacy helper mirrors owned command arguments during staged replacement.
#[allow(clippy::needless_pass_by_value)]
fn save_lab_guarded(
    dir: &Path,
    json: &str,
    expected: Option<String>,
    source: Option<SourceRevision>,
) -> Result<String, String> {
    prepare_directory(dir)?;
    let pack = ivac_core::feeds_speeds::validate_source_pack_json(json)?;
    let mut ids = vec![pack.manifest.identity.id.as_str()];
    if let Some(ref guard) = source {
        ids.push(&guard.id);
    }
    ids.sort_unstable();
    ids.dedup();
    let _locks = ids
        .into_iter()
        .map(|id| source_write_lock(dir, id))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(ref guard) = source {
        if head_revision(dir, &guard.id)?.as_deref() != Some(guard.revision.as_str()) {
            return Err("Source changed; reopen and review the copy again".into());
        }
    }
    if !is_lab_pack_id(&pack.manifest.identity.id)
        || serde_json::to_value(pack.manifest.visibility).map_err(|e| e.to_string())? != "private"
        || serde_json::to_value(pack.manifest.distribution).map_err(|e| e.to_string())?
            != "local_only"
    {
        return Err("Only private local presets can be authored".into());
    }
    let head = head_revision(dir, &pack.manifest.identity.id)?;
    if head != expected
        || head
            .as_ref()
            .is_some_and(|v| v >= &pack.manifest.identity.revision)
    {
        return Err("Presets changed; reopen and retry".into());
    }
    store_locked(dir, json)
}
fn head_revision(dir: &Path, id: &str) -> Result<Option<String>, String> {
    let mut candidates = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    candidates.retain(|e| {
        e.file_name()
            .to_string_lossy()
            .starts_with(&format!("{id}@"))
            && e.path().extension().is_some_and(|v| v == "json")
    });
    candidates.sort_by_key(std::fs::DirEntry::file_name);
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
    Ok(head)
}
#[tauri::command]
// owned argument is the transport/command contract.
#[allow(clippy::needless_pass_by_value)]
pub fn save_lab_pack(
    app: tauri::AppHandle,
    json: String,
    expected_revision: Option<String>,
) -> Result<String, String> {
    save_lab(&root(&app)?, &json, expected_revision)
}

// Separate command fails closed when the installed native backend predates guarded copying.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn save_adapted_lab_pack(
    app: tauri::AppHandle,
    json: String,
    expected_revision: Option<String>,
    source_guard: Option<SourceRevision>,
) -> Result<String, String> {
    save_lab_guarded(&root(&app)?, &json, expected_revision, source_guard)
}

// Only validated ids are accepted, never a caller-provided filesystem path.
fn remove_source(dir: &Path, id: &str, expected_head: &str) -> Result<(), String> {
    if !ivac_core::feeds_speeds::safe_pack_id(id)
        || !ivac_core::feeds_speeds::safe_pack_id(expected_head)
    {
        return Err("Invalid source id or revision".into());
    }
    prepare_directory(dir)?;
    let _lock = source_write_lock(dir, id)?;
    let prefix = format!("{id}@");
    let mut entries = std::fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    entries.retain(|e| {
        e.file_name().to_string_lossy().starts_with(&prefix)
            && e.path().extension().is_some_and(|v| v == "json")
    });
    entries.sort_by_key(std::fs::DirEntry::file_name);
    if entries.last().map(std::fs::DirEntry::file_name)
        != Some(format!("{id}@{expected_head}.json").into())
    {
        return Err("Source changed; refresh and retry".into());
    }
    // Preflight every entry; do not follow links, descend into directories, or touch other ids.
    for entry in &entries {
        let meta = entry.file_type().map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.is_symlink() {
            return Err("Unexpected source file type".into());
        }
    }
    // Oldest first: an I/O failure keeps the current head. A retry can finish removing history.
    for entry in entries {
        std::fs::remove_file(entry.path())
            .map_err(|e| format!("Source removal incomplete; refresh and retry: {e}"))?;
    }
    Ok(())
}
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn remove_source_pack(
    app: tauri::AppHandle,
    id: String,
    expected_head: String,
) -> Result<(), String> {
    remove_source(&root(&app)?, &id, &expected_head)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adapted_copy_checks_source_and_destination_under_shared_locks() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().canonicalize().unwrap();
        let mut pack: serde_json::Value = serde_json::from_str(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        pack["manifest"]["visibility"] = serde_json::json!("private");
        pack["manifest"]["distribution"] = serde_json::json!("local_only");
        pack["records"] = serde_json::json!([]);
        pack["dataset"]["profile_set"]["profiles"] = serde_json::json!([]);
        let source = "lab-00000000-0000-4000-8000-000000000001";
        let target = "lab-00000000-0000-4000-8000-000000000002";
        for id in [source, target] {
            pack["manifest"]["identity"] =
                serde_json::json!({"id":id,"revision":"0000000000000001"});
            save_lab(&dir, &pack.to_string(), None).unwrap();
        }
        pack["manifest"]["identity"] =
            serde_json::json!({"id":target,"revision":"0000000000000002"});
        let guard = |revision: &str| {
            Some(SourceRevision {
                id: source.into(),
                revision: revision.into(),
            })
        };
        // Stale source and stale destination both leave exactly the original two files.
        assert!(save_lab_guarded(
            &dir,
            &pack.to_string(),
            Some("0000000000000001".into()),
            guard("0000000000000000")
        )
        .is_err());
        assert!(
            save_lab_guarded(&dir, &pack.to_string(), None, guard("0000000000000001")).is_err()
        );
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
        let lock = source_write_lock(&dir, source).unwrap();
        assert!(save_lab_guarded(
            &dir,
            &pack.to_string(),
            Some("0000000000000001".into()),
            guard("0000000000000001")
        )
        .is_err());
        drop(lock);
        save_lab_guarded(
            &dir,
            &pack.to_string(),
            Some("0000000000000001".into()),
            guard("0000000000000001"),
        )
        .unwrap();
        // Same-pack source/destination uses one lock and still compares the pinned head.
        pack["manifest"]["identity"] =
            serde_json::json!({"id":source,"revision":"0000000000000002"});
        save_lab_guarded(
            &dir,
            &pack.to_string(),
            Some("0000000000000001".into()),
            guard("0000000000000001"),
        )
        .unwrap();
        pack["manifest"]["identity"] =
            serde_json::json!({"id":target,"revision":"0000000000000003"});
        assert!(save_lab_guarded(
            &dir,
            &pack.to_string(),
            Some("0000000000000002".into()),
            guard("0000000000000001")
        )
        .is_err());
        assert!(!dir.join(format!("{target}@0000000000000003.json")).exists());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 4);
    }
    #[test]
    fn removal_checks_head_and_removes_all_revisions_only_for_the_selected_id() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().canonicalize().unwrap();
        for name in ["local@1.json", "local@2.json", "local-other@1.json"] {
            std::fs::write(dir.join(name), "{}").unwrap();
        }
        assert!(remove_source(&dir, "local", "1").is_err());
        assert!(dir.join("local@1.json").exists());
        let lock = source_write_lock(&dir, "local").unwrap();
        assert!(remove_source(&dir, "local", "2").is_err());
        drop(lock);
        remove_source(&dir, "local", "2").unwrap();
        assert!(!dir.join("local@1.json").exists());
        assert!(!dir.join("local@2.json").exists());
        assert!(dir.join("local-other@1.json").exists());
        assert!(remove_source(&dir, "../outside", "1").is_err());
        assert!(remove_source(&dir, "local-other", "../1").is_err());
    }
    #[cfg(unix)]
    #[test]
    fn removal_rejects_symlinks_before_deleting_any_revision() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().canonicalize().unwrap();
        std::fs::write(dir.join("outside"), "keep").unwrap();
        std::fs::write(dir.join("local@1.json"), "{}").unwrap();
        std::os::unix::fs::symlink(dir.join("outside"), dir.join("local@2.json")).unwrap();
        assert!(remove_source(&dir, "local", "2").is_err());
        assert!(dir.join("local@1.json").exists());
        assert_eq!(
            std::fs::read_to_string(dir.join("outside")).unwrap(),
            "keep"
        );
    }
    #[test]
    fn audit_15_16_lab_rejects_variable_width_and_text_revisions() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path().canonicalize().unwrap();
        let mut pack: serde_json::Value = serde_json::from_str(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        pack["manifest"]["identity"]["id"] = serde_json::json!("sorbonne-fablab");
        pack["manifest"]["visibility"] = serde_json::json!("private");
        pack["manifest"]["distribution"] = serde_json::json!("local_only");
        pack["records"] = serde_json::json!([]);
        pack["dataset"]["profile_set"]["profiles"] = serde_json::json!([]);
        for revision in ["2", "10", "v1", "v1-a", "2026.10"] {
            pack["manifest"]["identity"]["revision"] = serde_json::json!(revision);
            assert!(save_lab(&dir, &pack.to_string(), None).is_err());
        }
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0000000000000009");
        save_lab(&dir, &pack.to_string(), None).unwrap();
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0000000000000010");
        save_lab(&dir, &pack.to_string(), Some("0000000000000009".into())).unwrap();
        assert_eq!(std::fs::read_dir(dir).unwrap().count(), 2);
    }

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
    fn named_packs_keep_independent_heads_locks_and_import_history() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().canonicalize().unwrap();
        let mut pack: serde_json::Value = serde_json::from_str(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        pack["manifest"]["visibility"] = serde_json::json!("private");
        pack["manifest"]["distribution"] = serde_json::json!("local_only");
        pack["records"] = serde_json::json!([]);
        pack["dataset"]["profile_set"]["profiles"] = serde_json::json!([]);
        for id in [
            "sorbonne-fablab",
            "lab-00000000-0000-4000-8000-000000000001",
            "lab-00000000-0000-4000-8000-000000000002",
        ] {
            pack["manifest"]["identity"]["id"] = serde_json::json!(id);
            pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000000");
            save_lab(&dir, &pack.to_string(), None).unwrap();
        }
        pack["manifest"]["identity"]["id"] =
            serde_json::json!("lab-00000000-0000-4000-8000-000000000001");
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000001");
        assert!(save_lab(&dir, &pack.to_string(), None).is_err());
        let lock = source_write_lock(&dir, "lab-00000000-0000-4000-8000-000000000001").unwrap();
        assert!(save_lab(&dir, &pack.to_string(), Some("0001791792000000".into())).is_err());
        drop(lock);
        save_lab(&dir, &pack.to_string(), Some("0001791792000000".into())).unwrap();
        assert!(store(&dir, &pack.to_string()).is_err());
        pack["manifest"]["identity"]["revision"] = serde_json::json!("0001791792000002");
        store(&dir, &pack.to_string()).unwrap();
        let rows = list_local_packs(&dir).unwrap();
        assert_eq!(rows.len(), 3);
        let x = rows
            .iter()
            .map(|row| serde_json::from_str::<serde_json::Value>(row).unwrap())
            .find(|p| p["manifest"]["identity"]["id"] == "lab-00000000-0000-4000-8000-000000000001")
            .unwrap();
        assert_eq!(x["manifest"]["identity"]["revision"], "0001791792000002");
        assert!(dir
            .join("lab-00000000-0000-4000-8000-000000000001@0001791792000000.json")
            .exists());
        assert!(dir
            .join("lab-00000000-0000-4000-8000-000000000002@0001791792000000.json")
            .exists());
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
