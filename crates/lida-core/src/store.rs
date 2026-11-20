//! Repository persistence and atomic dual-write synchronization engine.

use crate::format::md::{parse_markdown, serialize_markdown};
use crate::format::yaml::{parse_yaml, serialize_yaml};
use crate::format::FormatError;
use crate::model::{ProjectDoc, Task};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Storage errors.
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Format error: {0}")]
    Format(#[from] FormatError),
    #[error("No TODO file found in directory")]
    NotFound,
}

/// The active storage format mode for a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageFormat {
    YamlOnly,
    MdOnly,
    Both,
}

/// Project store managing disk persistence and synchronization.
#[derive(Debug, Clone)]
pub struct Store {
    dir: PathBuf,
    format: StorageFormat,
    doc: ProjectDoc,
}

impl Store {
    /// Loads the project document from the given directory.
    ///
    /// # Errors
    /// Returns `StoreError` if files cannot be read or parsed.
    pub fn load(dir: &Path) -> Result<Self, StoreError> {
        let yaml_path = dir.join("TODO.yaml");
        let md_path = dir.join("TODO.md");

        let yaml_exists = yaml_path.exists();
        let md_exists = md_path.exists();

        if !yaml_exists && !md_exists {
            return Err(StoreError::NotFound);
        }

        let (format, doc) = match (yaml_exists, md_exists) {
            (true, true) => {
                // Both exist: dual-write mode. Read the one with the freshest mtime.
                let yaml_mtime = fs::metadata(&yaml_path)?.modified().ok();
                let md_mtime = fs::metadata(&md_path)?.modified().ok();

                let read_yaml = match (yaml_mtime, md_mtime) {
                    (Some(y), Some(m)) => y >= m,
                    _ => true,
                };

                let doc = if read_yaml {
                    parse_yaml(&fs::read_to_string(&yaml_path)?)?
                } else {
                    parse_markdown(&fs::read_to_string(&md_path)?)?
                };
                (StorageFormat::Both, doc)
            }
            (true, false) => {
                let doc = parse_yaml(&fs::read_to_string(&yaml_path)?)?;
                (StorageFormat::YamlOnly, doc)
            }
            (false, true) => {
                let doc = parse_markdown(&fs::read_to_string(&md_path)?)?;
                (StorageFormat::MdOnly, doc)
            }
            (false, false) => unreachable!(),
        };

        Ok(Self {
            dir: dir.to_path_buf(),
            format,
            doc,
        })
    }

    /// Initializes a new store with the given format in `dir`.
    ///
    /// # Errors
    /// Returns `StoreError` if files cannot be written.
    pub fn init(dir: &Path, format: StorageFormat, project: Option<String>) -> Result<Self, StoreError> {
        let doc = ProjectDoc {
            tasks: Vec::new(),
            actors: Vec::new(),
            comments: Vec::new(),
            project,
        };
        let store = Self {
            dir: dir.to_path_buf(),
            format,
            doc,
        };
        store.save()?;
        Ok(store)
    }

    #[must_use]
    pub fn format(&self) -> StorageFormat {
        self.format
    }

    pub fn set_format(&mut self, format: StorageFormat) {
        self.format = format;
    }

    #[must_use]
    pub fn doc(&self) -> &ProjectDoc {
        &self.doc
    }

    pub fn doc_mut(&mut self) -> &mut ProjectDoc {
        &mut self.doc
    }

    pub fn add_task(&mut self, task: Task) {
        self.doc.tasks.push(task);
    }

    pub fn find_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.doc.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn remove_task(&mut self, id: &str) -> bool {
        let initial_len = self.doc.tasks.len();
        self.doc.tasks.retain(|t| t.id != id);
        self.doc.tasks.len() < initial_len
    }

    /// Atomically writes to target files based on `StorageFormat`.
    ///
    /// # Errors
    /// Returns `StoreError` if writing or file replacement fails.
    pub fn save(&self) -> Result<(), StoreError> {
        match self.format {
            StorageFormat::YamlOnly => {
                let content = serialize_yaml(&self.doc)?;
                Self::atomic_write(&self.dir.join("TODO.yaml"), &content)?;
            }
            StorageFormat::MdOnly => {
                let content = serialize_markdown(&self.doc)?;
                Self::atomic_write(&self.dir.join("TODO.md"), &content)?;
            }
            StorageFormat::Both => {
                let yaml_content = serialize_yaml(&self.doc)?;
                let md_content = serialize_markdown(&self.doc)?;
                Self::atomic_write(&self.dir.join("TODO.yaml"), &yaml_content)?;
                Self::atomic_write(&self.dir.join("TODO.md"), &md_content)?;
            }
        }
        Ok(())
    }

    fn atomic_write(target: &Path, content: &str) -> Result<(), io::Error> {
        let parent = target.parent().unwrap_or_else(|| Path::new("."));
        let filename = target.file_name().unwrap_or_default().to_string_lossy();
        let tmp_path = parent.join(format!(".{filename}.tmp.{}", fastrand::u32(..)));

        fs::write(&tmp_path, content)?;
        fs::rename(&tmp_path, target)?;
        Ok(())
    }
}
