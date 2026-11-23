//! Repository persistence and atomic dual-write synchronization engine.

use crate::format::FormatError;
use crate::format::md::{parse_markdown, serialize_markdown};
use crate::format::yaml::{parse_yaml, serialize_yaml};
use crate::model::{ProjectDoc, Task};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
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

impl StorageFormat {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::YamlOnly => "yaml",
            Self::MdOnly => "md",
            Self::Both => "both",
        }
    }
}

/// Project store managing disk persistence and synchronization.
#[derive(Debug, Clone)]
pub struct Store {
    dir: PathBuf,
    format: StorageFormat,
    doc: ProjectDoc,
}

impl Store {
    /// Reads the configured format from `.lida/config` or `.todo/config` if present.
    fn read_config_format(dir: &Path) -> Option<StorageFormat> {
        let lida_cfg = dir.join(".lida").join("config");
        let todo_cfg = dir.join(".todo").join("config");

        let cfg_path = if lida_cfg.exists() {
            Some(lida_cfg)
        } else if todo_cfg.exists() {
            Some(todo_cfg)
        } else {
            None
        };

        if let Some(path) = cfg_path {
            if let Ok(content) = fs::read_to_string(path) {
                return match content.trim().to_lowercase().as_str() {
                    "yaml" => Some(StorageFormat::YamlOnly),
                    "md" | "markdown" => Some(StorageFormat::MdOnly),
                    "both" => Some(StorageFormat::Both),
                    _ => None,
                };
            }
        }
        None
    }

    /// Writes the active format to `.lida/config` (or `.todo/config` if that exists).
    fn write_config_format(&self, format: StorageFormat) -> Result<(), io::Error> {
        let lida_dir = self.dir.join(".lida");
        let todo_dir = self.dir.join(".todo");

        let target_file = if todo_dir.exists() {
            todo_dir.join("config")
        } else {
            fs::create_dir_all(&lida_dir)?;
            lida_dir.join("config")
        };

        fs::write(target_file, format!("{format}\n", format = format.as_str()))
    }

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

        let config_format = Self::read_config_format(dir);

        let (format, doc) = match (yaml_exists, md_exists) {
            (true, true) => {
                // In dual-write mode, TODO.yaml is authoritative for the data model
                // because YAML preserves full rich structures (actors, comments, and extra fields).
                let doc = parse_yaml(&fs::read_to_string(&yaml_path)?)?;
                (config_format.unwrap_or(StorageFormat::Both), doc)
            }
            (true, false) => {
                let doc = parse_yaml(&fs::read_to_string(&yaml_path)?)?;
                (config_format.unwrap_or(StorageFormat::YamlOnly), doc)
            }
            (false, true) => {
                let doc = parse_markdown(&fs::read_to_string(&md_path)?)?;
                (config_format.unwrap_or(StorageFormat::MdOnly), doc)
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
            extra: BTreeMap::new(),
        };
        let store = Self {
            dir: dir.to_path_buf(),
            format,
            doc,
        };
        store.write_config_format(format)?;
        store.save()?;
        Ok(store)
    }

    #[must_use]
    pub fn format(&self) -> StorageFormat {
        self.format
    }

    /// Sets the format mode and persists it to config.
    ///
    /// # Errors
    /// Returns `StoreError` if saving the config fails.
    pub fn set_format(&mut self, format: StorageFormat) -> Result<(), StoreError> {
        self.format = format;
        self.write_config_format(format)?;
        Ok(())
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
                Self::atomic_write_both(
                    &self.dir.join("TODO.yaml"),
                    &yaml_content,
                    &self.dir.join("TODO.md"),
                    &md_content,
                )?;
            }
        }
        Ok(())
    }

    fn atomic_write(target: &Path, content: &str) -> Result<(), io::Error> {
        let parent = target.parent().unwrap_or_else(|| Path::new("."));
        let filename = target.file_name().unwrap_or_default().to_string_lossy();
        let tmp_path = parent.join(format!(".{filename}.tmp.{}", fastrand::u32(..)));

        let write_res = (|| -> Result<(), io::Error> {
            let mut file = fs::File::create(&tmp_path)?;
            file.write_all(content.as_bytes())?;
            file.sync_all()?;
            fs::rename(&tmp_path, target)?;
            Ok(())
        })();

        if write_res.is_err() {
            let _ = fs::remove_file(&tmp_path);
        }
        write_res
    }

    fn atomic_write_both(
        yaml_target: &Path,
        yaml_content: &str,
        md_target: &Path,
        md_content: &str,
    ) -> Result<(), io::Error> {
        let parent = yaml_target.parent().unwrap_or_else(|| Path::new("."));
        let rand_suffix = fastrand::u32(..);
        let tmp_yaml = parent.join(format!(".TODO.yaml.tmp.{rand_suffix}"));
        let tmp_md = parent.join(format!(".TODO.md.tmp.{rand_suffix}"));

        let write_res = (|| -> Result<(), io::Error> {
            {
                let mut f_yaml = fs::File::create(&tmp_yaml)?;
                f_yaml.write_all(yaml_content.as_bytes())?;
                f_yaml.sync_all()?;
            }
            {
                let mut f_md = fs::File::create(&tmp_md)?;
                f_md.write_all(md_content.as_bytes())?;
                f_md.sync_all()?;
            }

            fs::rename(&tmp_yaml, yaml_target)?;
            fs::rename(&tmp_md, md_target)?;
            Ok(())
        })();

        if write_res.is_err() {
            let _ = fs::remove_file(&tmp_yaml);
            let _ = fs::remove_file(&tmp_md);
        }
        write_res
    }
}
