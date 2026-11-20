//! CLI specification and arguments parsed via usage-rs.

use usage_rs::{Cli, Subcommands};

#[derive(Cli, Debug)]
#[usage(bin = "lida", version = "0.1.0", about = "Dual-format project task engine (YAML · MD)")]
pub struct LidaCli {
    #[usage(subcommand)]
    pub command: Option<Subcommand>,
}

#[derive(Subcommands, Debug)]
pub enum Subcommand {
    /// Initialize a new TODO file (TODO.yaml or TODO.md)
    Init {
        /// Initialize TODO.yaml
        #[usage(long)]
        yaml: bool,

        /// Initialize TODO.md
        #[usage(long)]
        md: bool,

        /// Initialize both TODO.yaml and TODO.md with dual-write sync
        #[usage(long)]
        both: bool,

        /// Project name to embed in header
        #[usage(long)]
        project: Option<String>,
    },

    /// Add a new task (optionally with initial comment or metadata)
    Add {
        /// Positional task description shorthand
        description: Option<String>,

        /// Legacy flag task description
        #[usage(long)]
        task: Option<String>,

        /// Priority level: low, medium, high
        #[usage(short = 'p', long)]
        priority: Option<String>,

        /// Comma-separated tags
        #[usage(short = 't', long)]
        tags: Option<String>,

        /// Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)
        #[usage(short = 'd', long)]
        due: Option<String>,

        /// Code position URL or file path
        #[usage(long)]
        position: Option<String>,

        /// Initial comment to attach to the task
        #[usage(short = 'c', long)]
        comment: Option<String>,
    },

    /// List tasks, actors, or comments
    List {
        /// Show tasks only
        #[usage(long)]
        tasks: bool,

        /// Filter by status: todo, in-progress, done, blocked
        #[usage(short = 's', long)]
        status: Option<String>,

        /// Filter by tag
        #[usage(short = 't', long)]
        tag: Option<String>,

        /// Filter by priority: low, medium, high
        #[usage(short = 'p', long)]
        priority: Option<String>,

        /// Output as structured JSON array
        #[usage(long)]
        json: bool,
    },

    /// Mark a task as done
    Done {
        /// 4-character task ID
        id: String,
    },

    /// Mark a task as in-progress
    Start {
        /// 4-character task ID
        id: String,
    },

    /// Change a task's status
    Status {
        /// 4-character task ID
        id: String,

        /// Status value: todo, in-progress, done, blocked (or en-cours, bloqued)
        #[usage(long)]
        set: String,

        /// Reason if blocked
        #[usage(long)]
        reason: Option<String>,
    },

    /// View full task details
    Show {
        /// 4-character task ID
        id: String,

        /// Output as JSON
        #[usage(long)]
        json: bool,
    },

    /// Get, set, or remove task metadata
    Meta {
        /// 4-character task ID
        id: String,

        /// Action: set, get, unset
        action: String,

        /// Metadata key
        key: String,

        /// Value to set (required for 'set')
        value: Option<String>,
    },

    /// List all tags in use with task counts
    Tags,

    /// Delete a task by ID
    Delete {
        /// 4-character task ID
        id: String,
    },

    /// Check or switch active format mode
    Cwi {
        /// Target format: yaml, md, both
        format: Option<String>,
    },

    /// Generate shell completion scripts
    Completion {
        /// Shell type (bash, zsh, fish, powershell, elvish)
        shell: String,
    },

    /// Export the portable .usage.kdl specification
    Usage,
}
