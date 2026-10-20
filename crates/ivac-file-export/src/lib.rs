//! Native per-file transactional publication. No CAM or machine policy.
//! Staging is on the destination filesystem. Publication is not pair-atomic,
//! crash-durable, or guaranteed on every network/virtual filesystem.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// A successful final-name publication, possibly with a staging cleanup warning.
/// An Err from publish always means this attempt did not publish the final name.
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[must_use]
pub struct PublishedFile {
    pub path: PathBuf,
    pub cleanup_warning: Option<String>,
}

/// A prepared destination whose contents have not yet been published.
#[derive(Debug)]
pub struct StagedFile {
    target: PathBuf,
    file: NamedTempFile,
}

/// Resolve the destination parent and reject special final entries before permission checks.
pub fn destination(path: &Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "output needs a filename"))?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let target = parent.canonicalize()?.join(name);
    match fs::symlink_metadata(&target) {
        Ok(meta) if !meta.file_type().is_file() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "output must be a regular file, not a directory or symlink",
        )),
        Ok(meta) if meta.permissions().readonly() => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "output is read-only",
        )),
        Ok(_) => Ok(target),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(target),
        Err(e) => Err(e),
    }
}

impl StagedFile {
    /// Stage beside the target without opening or truncating the target.
    pub fn new(target: &Path) -> io::Result<Self> {
        let target = destination(target)?;
        let file = tempfile::Builder::new()
            .prefix(".fabcam-export-")
            .tempfile_in(target.parent().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "missing output directory")
            })?)?;
        Ok(Self { target, file })
    }

    /// Separate writer owned by the generator. It must be finished and dropped
    /// before publication; no target bytes are modified by this handle.
    pub fn writer(&self) -> io::Result<File> {
        self.file.reopen()
    }

    /// Write an already validated complete program.
    pub fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.file.write_all(bytes)
    }

    /// Flush pending writes and surface filesystem errors before publication.
    pub fn finish(&mut self) -> io::Result<()> {
        self.file.flush()?;
        self.file.as_file().sync_all()
    }

    /// Publish one complete file. New-only publication uses a hard link:
    /// creation of the final name fails atomically if a concurrent file exists.
    /// Unsupported filesystems fail closed; no copy/truncate fallback.
    ///
    /// Replacement uses the platform rename supported by `tempfile`. Symlink
    /// targets are refused; parent resolution is pinned at staging time.
    pub fn publish(self, new_only: bool) -> io::Result<PublishedFile> {
        self.publish_with_cleanup(new_only, tempfile::TempPath::close)
    }

    // Injection is private and available to unit tests, never a transport option.
    fn publish_with_cleanup(
        mut self,
        new_only: bool,
        cleanup: impl FnOnce(tempfile::TempPath) -> io::Result<()>,
    ) -> io::Result<PublishedFile> {
        self.finish()?;
        destination(&self.target)?;
        if let Ok(meta) = fs::metadata(&self.target) {
            self.file.as_file().set_permissions(meta.permissions())?;
        }
        let Self { target, file } = self;
        let temporary = file.into_temp_path(); // close before publication
        if new_only {
            fs::hard_link(&temporary, &target)?;
            // Final publication has succeeded: cleanup can only add a warning.
            let cleanup_warning = cleanup(temporary).err().map(|error| error.to_string());
            Ok(PublishedFile {
                path: target,
                cleanup_warning,
            })
        } else {
            temporary.persist(&target).map_err(|e| e.error)?;
            Ok(PublishedFile {
                path: target,
                cleanup_warning: None,
            })
        }
    }
}

/// Refuse either existing name before starting a two-sided export.
/// Publication must also use `new_only` to handle races after this preflight.
pub fn require_new(paths: &[&Path]) -> io::Result<()> {
    for path in paths {
        let target = destination(path)?;
        match fs::symlink_metadata(target) {
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "two-sided export requires new output names; choose a new --output base",
                ))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_generation_preserves_target_and_removes_staging() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job.sbp");
        fs::write(&target, b"old").unwrap();
        {
            let mut staged = StagedFile::new(&target).unwrap();
            staged.write_all(b"partial").unwrap();
            // Simulates refusal/cancellation after partial generation.
        }
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn valid_bytes_replace_only_after_publication() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job.sbp");
        fs::write(&target, b"old").unwrap();
        let mut staged = StagedFile::new(&target).unwrap();
        let bytes = b"' program\r\nM3,1,2,-3\r\n";
        staged.write_all(bytes).unwrap();
        staged.finish().unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"old");
        let _published = staged.publish(false).unwrap();
        assert_eq!(fs::read(&target).unwrap(), bytes);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn new_only_refuses_existing_and_concurrent_destination() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job.front.sbp");
        require_new(&[&target]).unwrap();
        let mut staged = StagedFile::new(&target).unwrap();
        staged.write_all(b"new").unwrap();
        fs::write(&target, b"concurrent").unwrap();
        assert!(require_new(&[&target]).is_err());
        assert!(staged.publish(true).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"concurrent");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn missing_parent_and_directory_target_fail_without_artifacts() {
        let dir = tempfile::tempdir().unwrap();
        assert!(StagedFile::new(&dir.path().join("missing/job")).is_err());
        assert!(StagedFile::new(dir.path()).is_err());
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[test]
    fn failed_publication_preserves_existing_target() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job");
        fs::write(&target, b"old").unwrap();
        let mut staged = StagedFile::new(&target).unwrap();
        staged.write_all(b"new").unwrap();
        // Deterministic filesystem fault, without filling the real disk.
        fs::remove_file(staged.file.path()).unwrap();
        assert!(staged.publish(false).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn write_error_after_prefix_preserves_target_and_cleans_temporary() {
        struct FailingWriter {
            file: File,
            written: bool,
        }
        impl Write for FailingWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                if self.written {
                    return Err(io::Error::other("injected disk write failure"));
                }
                self.written = true;
                self.file.write(&bytes[..bytes.len().min(3)])
            }
            fn flush(&mut self) -> io::Result<()> {
                self.file.flush()
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job");
        fs::write(&target, b"old").unwrap();
        {
            let staged = StagedFile::new(&target).unwrap();
            let mut writer = FailingWriter {
                file: staged.writer().unwrap(),
                written: false,
            };
            assert!(writer.write_all(b"partially generated program").is_err());
        }
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn two_sided_preflight_preserves_both_old_generations() {
        let dir = tempfile::tempdir().unwrap();
        let front = dir.path().join("front");
        let back = dir.path().join("back");
        fs::write(&front, b"old-front").unwrap();
        fs::write(&back, b"old-back").unwrap();
        assert!(require_new(&[&front, &back]).is_err());
        assert_eq!(fs::read(&front).unwrap(), b"old-front");
        assert_eq!(fs::read(&back).unwrap(), b"old-back");
    }

    #[test]
    fn readonly_destination_is_not_replaced_via_directory_permissions() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("job");
        fs::write(&target, b"old").unwrap();
        let original = fs::metadata(&target).unwrap().permissions();
        let mut permissions = original.clone();
        permissions.set_readonly(true);
        fs::set_permissions(&target, permissions).unwrap();
        assert_eq!(
            StagedFile::new(&target).unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(fs::read(&target).unwrap(), b"old");
        fs::set_permissions(&target, original).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlink_destination_is_never_followed() {
        let dir = tempfile::tempdir().unwrap();
        let other = dir.path().join("other");
        fs::write(&other, b"untouched").unwrap();
        let link = dir.path().join("link");
        std::os::unix::fs::symlink(&other, &link).unwrap();
        assert!(StagedFile::new(&link).is_err());
        assert_eq!(fs::read(other).unwrap(), b"untouched");
    }
}

#[cfg(test)]
mod cleanup_tests {
    use super::*;
    #[test]
    fn cleanup_failure_reports_published_bytes_and_keeps_other_side_publishable() {
        let dir = tempfile::tempdir().unwrap();
        for side in ["front", "back"] {
            let target = dir.path().join(side);
            let mut staged = StagedFile::new(&target).unwrap();
            staged.write_all(b"complete program").unwrap();
            let result = staged
                .publish_with_cleanup(true, |temporary| {
                    let _retained = temporary.keep().unwrap();
                    Err(io::Error::other("injected cleanup failure"))
                })
                .unwrap();
            assert_eq!(result.path, target.canonicalize().unwrap());
            assert_eq!(
                result.cleanup_warning.as_deref(),
                Some("injected cleanup failure")
            );
            assert_eq!(fs::read(&target).unwrap(), b"complete program");
        }
    }
    #[test]
    fn second_side_failure_is_unpublished_and_first_remains_complete() {
        let dir = tempfile::tempdir().unwrap();
        let front_path = dir.path().join("front");
        let back_path = dir.path().join("back");
        let mut front = StagedFile::new(&front_path).unwrap();
        let mut back = StagedFile::new(&back_path).unwrap();
        front.write_all(b"front").unwrap();
        back.write_all(b"back").unwrap();
        let first = front.publish(true).unwrap();
        assert!(first.cleanup_warning.is_none());
        fs::write(&back_path, b"concurrent destination").unwrap();
        assert!(back.publish(true).is_err());
        assert_eq!(fs::read(front_path).unwrap(), b"front");
        assert_eq!(fs::read(back_path).unwrap(), b"concurrent destination");
    }
}
