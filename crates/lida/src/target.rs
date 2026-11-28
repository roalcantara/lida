//! Resolves the `-f`/`--file` target to the directory lida runs in.

use std::path::{Path, PathBuf};

/// Turns `file` (from `-f`/`--file` or `LIDA_FILE`) into the directory lida runs in.
///
/// A relative path is joined to `cwd`; a directory is used as is; a `TODO.yaml` or `TODO.md` path resolves to its
/// parent directory, whether or not the file exists. Anything else is an error.
///
/// # Errors
/// Returns the wrong-name error for an existing path that is neither a directory nor a TODO file, and the
/// missing-directory error for a path that does not exist.
pub fn resolve(file: Option<&str>, cwd: &Path) -> Result<PathBuf, String> {
    let Some(file) = file else {
        return Ok(cwd.to_path_buf());
    };
    let path = {
        let given = Path::new(file);
        if given.is_absolute() {
            given.to_path_buf()
        } else {
            cwd.join(given)
        }
    };
    if path.is_dir() {
        return Ok(path);
    }
    let is_todo = matches!(
        path.file_name().and_then(std::ffi::OsStr::to_str),
        Some("TODO.yaml" | "TODO.md")
    );
    if is_todo {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        if parent.is_dir() {
            return Ok(parent.to_path_buf());
        }
        return Err(format!("no such directory: {}", parent.display()));
    }
    if path.exists() {
        Err(format!(
            "--file must be a directory, TODO.yaml or TODO.md: {}",
            path.display()
        ))
    } else {
        Err(format!("no such directory: {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::resolve;
    use std::fs;

    fn workspace() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("temp dir");
        fs::create_dir(dir.path().join("sub")).expect("create sub");
        fs::write(dir.path().join("notes.txt"), "").expect("write notes");
        dir
    }

    #[test]
    fn none_is_cwd() {
        let d = workspace();
        assert_eq!(resolve(None, d.path()), Ok(d.path().to_path_buf()));
    }

    #[test]
    fn directory_is_used() {
        let d = workspace();
        assert_eq!(resolve(Some("sub"), d.path()), Ok(d.path().join("sub")));

        let other = tempfile::tempdir().expect("temp dir");
        let absolute = d.path().join("sub").to_string_lossy().into_owned();
        assert_eq!(resolve(Some(&absolute), other.path()), Ok(d.path().join("sub")));
    }

    #[test]
    fn todo_file_resolves_to_parent() {
        let d = workspace();
        for name in ["sub/TODO.yaml", "sub/TODO.md"] {
            assert_eq!(resolve(Some(name), d.path()), Ok(d.path().join("sub")));
        }
    }

    #[test]
    fn other_file_name_errors() {
        let d = workspace();
        assert_eq!(
            resolve(Some("notes.txt"), d.path()),
            Err(format!(
                "--file must be a directory, TODO.yaml or TODO.md: {}",
                d.path().join("notes.txt").display()
            ))
        );
    }

    #[test]
    fn missing_path_errors() {
        let d = workspace();
        assert_eq!(
            resolve(Some("nope"), d.path()),
            Err(format!("no such directory: {}", d.path().join("nope").display()))
        );
        assert_eq!(
            resolve(Some("nope/TODO.yaml"), d.path()),
            Err(format!("no such directory: {}", d.path().join("nope").display()))
        );
    }
}
