//! Core domain models for tasks, actors, comments, and project documents.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// Status lifecycle of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
    Blocked,
}

impl Status {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "Todo",
            Self::InProgress => "InProgress",
            Self::Done => "Done",
            Self::Blocked => "Blocked",
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().replace('_', "-").as_str() {
            "todo" => Ok(Self::Todo),
            "inprogress" | "in-progress" | "en-cours" | "encours" | "progress" => Ok(Self::InProgress),
            "done" | "completed" => Ok(Self::Done),
            "blocked" | "bloqued" => Ok(Self::Blocked),
            _ => Ok(Self::Todo),
        }
    }
}

/// Priority levels for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Priority {
    #[serde(alias = "low")]
    Low,
    #[serde(alias = "medium")]
    Medium,
    #[serde(alias = "high")]
    High,
}

impl Priority {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

/// An actor who participates in or owns tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Actor {
    pub id: String,
    pub pseudo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
}

/// A comment associated with a parent task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Comment {
    pub id: String,
    pub text: String,
    #[serde(default, alias = "task")]
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors: Vec<String>,
}

/// The core task model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: Status,
    pub description: String,
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub actors: Vec<String>,
    #[serde(default)]
    pub comments: Vec<String>,
    #[serde(default)]
    pub blocked_reason: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub priority: Option<Priority>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml_ng::Value>,
}

impl Task {
    #[must_use]
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: Status::Todo,
            description: description.into(),
            due: None,
            actors: Vec::new(),
            comments: Vec::new(),
            blocked_reason: None,
            tags: Vec::new(),
            priority: None,
            created: None,
            position: None,
            meta: HashMap::new(),
            extra: HashMap::new(),
        }
    }
}

/// A project document holding tasks, actors, and comments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectDoc {
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub actors: Vec<Actor>,
    #[serde(default)]
    pub comments: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
