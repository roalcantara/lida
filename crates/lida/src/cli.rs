//! CLI specification and arguments parsed via usage-rs.

use usage_rs::completion::Completion;
use usage_rs::spec::{Candidate, CompleteCtx};
use usage_rs::{Args, Cli, Subcommands};

#[derive(Cli, Debug)]
#[usage(
    bin = "lida",
    version = env!("CARGO_PKG_VERSION"),
    about = "Dual-format project task engine (YAML · MD)",
    completion
)]
pub struct LidaCli {
    #[usage(subcommand)]
    pub command: Option<Subcommand>,
}

fn load_current_store() -> Option<lida_core::store::Store> {
    let cwd = std::env::current_dir().ok()?;
    lida_core::store::Store::load(&cwd).ok()
}

fn complete_task_id<P>(_partial: &P, _ctx: &CompleteCtx<'_>) -> Vec<Candidate<'static>> {
    let Some(store) = load_current_store() else {
        return Vec::new();
    };
    store
        .doc()
        .tasks
        .iter()
        .map(|t| Candidate::described(t.id.clone(), t.description.clone()))
        .collect()
}

fn complete_tags<P>(_partial: &P, _ctx: &CompleteCtx<'_>) -> Vec<Candidate<'static>> {
    let Some(store) = load_current_store() else {
        return Vec::new();
    };
    let mut tags = std::collections::BTreeSet::new();
    for task in &store.doc().tasks {
        for tag in &task.tags {
            tags.insert(tag.clone());
        }
    }
    tags.into_iter().map(Candidate::new).collect()
}

fn complete_priority<P>(_partial: &P, _ctx: &CompleteCtx<'_>) -> Vec<Candidate<'static>> {
    ["low", "medium", "high", "none"]
        .into_iter()
        .map(Candidate::new)
        .collect()
}

fn complete_status<P>(_partial: &P, _ctx: &CompleteCtx<'_>) -> Vec<Candidate<'static>> {
    ["todo", "in-progress", "done", "blocked"]
        .into_iter()
        .map(Candidate::new)
        .collect()
}

fn complete_cwi_format<P>(_partial: &P, _ctx: &CompleteCtx<'_>) -> Vec<Candidate<'static>> {
    ["yaml", "md", "both"].into_iter().map(Candidate::new).collect()
}

#[derive(Args, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "CLI flags for initialization format and overwrite"
)]
pub struct InitArgs {
    /// Initialize TODO.yaml
    #[usage(long)]
    pub yaml: bool,

    /// Initialize TODO.md
    #[usage(long)]
    pub md: bool,

    /// Initialize both TODO.yaml and TODO.md with dual-write sync
    #[usage(long)]
    pub both: bool,

    /// Project name to embed in header
    #[usage(long)]
    pub project: Option<String>,

    /// Overwrite existing TODO files
    #[usage(short = 'f', long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Positional task description shorthand
    pub description: Option<String>,

    /// Legacy flag task description
    #[usage(long)]
    pub task: Option<String>,

    /// Attach comment to existing task ID
    #[usage(long = "task-id", complete = complete_task_id)]
    pub task_id: Option<String>,

    /// Priority level: low, medium, high
    #[usage(short = 'p', long, complete = complete_priority)]
    pub priority: Option<String>,

    /// Comma-separated tags
    #[usage(short = 't', long)]
    pub tags: Option<String>,

    /// Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)
    #[usage(short = 'd', long)]
    pub due: Option<String>,

    /// Code position URL or file path
    #[usage(long)]
    pub position: Option<String>,

    /// Initial comment to attach to the task
    #[usage(short = 'c', long)]
    pub comment: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,

    /// Task description
    #[usage(short = 'd', long)]
    pub description: Option<String>,

    /// Priority level: low, medium, high
    #[usage(short = 'p', long, complete = complete_priority)]
    pub priority: Option<String>,

    /// Comma-separated tags
    #[usage(short = 't', long, complete = complete_tags)]
    pub tags: Option<String>,

    /// Single tag or comma-separated tag alias
    #[usage(long, complete = complete_tags)]
    pub tag: Option<String>,

    /// Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)
    #[usage(long)]
    pub due: Option<String>,

    /// Code position URL or file path
    #[usage(long)]
    pub position: Option<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Print one plain line per task, for shell completion
    #[usage(long)]
    pub tasks: bool,

    /// Filter by status: todo, in-progress, done, blocked
    #[usage(short = 's', long, complete = complete_status)]
    pub status: Option<String>,

    /// Filter by tag
    #[usage(short = 't', long, complete = complete_tags)]
    pub tag: Option<String>,

    /// Filter by priority: low, medium, high
    #[usage(short = 'p', long, complete = complete_priority)]
    pub priority: Option<String>,

    /// Output as structured JSON array
    #[usage(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DoneArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct StartArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,

    /// Status value: todo, in-progress, done, blocked (or en-cours, bloqued)
    #[usage(long, complete = complete_status)]
    pub set: String,

    /// Reason if blocked
    #[usage(long)]
    pub reason: Option<String>,
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,

    /// Output as JSON
    #[usage(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct MetaArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,

    /// Action: set, get, unset
    pub action: String,

    /// Metadata key
    pub key: String,

    /// Value to set (required for 'set')
    pub value: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// 4-character task ID
    #[usage(complete = complete_task_id)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct CwiArgs {
    /// Target format: yaml, md, both
    #[usage(complete = complete_cwi_format)]
    pub format: Option<String>,
}

#[derive(Subcommands, Debug)]
pub enum Subcommand {
    /// Initialize a new TODO file (TODO.yaml or TODO.md)
    Init(InitArgs),

    /// Add a new task (optionally with initial comment)
    Add(AddArgs),

    /// Update an existing task's attributes
    Update(UpdateArgs),

    /// List tasks
    List(ListArgs),

    /// Mark a task as done
    Done(DoneArgs),

    /// Mark a task as in-progress
    Start(StartArgs),

    /// Change a task's status
    Status(StatusArgs),

    /// View full task details
    Show(ShowArgs),

    /// Get, set, or remove task metadata
    Meta(MetaArgs),

    /// List all tags in use with task counts
    Tags,

    /// Delete a task by ID
    Delete(DeleteArgs),

    /// Check or switch active format mode
    Cwi(CwiArgs),

    /// Generate shell completion scripts
    Completion(Completion),

    /// Export the portable .usage.kdl specification
    Usage,
}
