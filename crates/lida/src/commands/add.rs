//! `lida add`: create a task, or attach a comment to one with `--task-id`.

use super::annotation;
use super::prelude::*;
use super::{not_found, parse_priority_arg, parse_tags};
use lida_core::date::Date;
use lida_core::id::generate_id;
use lida_core::model::{Comment, Task};
use lida_core::store::StorageFormat;
use std::collections::{BTreeMap, HashSet};

/// `add`'s options; `description` is the positional text or, failing that, `--task`.
#[derive(Debug)]
pub struct Args {
    pub description: Option<String>,
    pub task_id: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<String>,
    pub due: Option<String>,
    pub position: Option<String>,
    pub comment: Option<String>,
    pub annotate: Option<String>,
}

pub fn run(cwd: &Path, args: Args, today: Date, out: &mut impl Write) -> CmdResult {
    if let Some(ref text) = args.annotate {
        single_line("annotation", text)?;
    }
    if args.task_id.is_some() && args.annotate.is_some() {
        return Err("--annotate cannot be used with --task-id".into());
    }
    if let Some(target_id) = args.task_id {
        let text = args
            .comment
            .or(args.description)
            .ok_or("comment text is required when specifying --task-id")?;
        single_line("comment", &text)?;
        let mut store = Store::load(cwd)?;
        return add_comment(&mut store, &target_id, text, out);
    }

    let desc = args
        .description
        .ok_or("task description is required (e.g. `lida add \"Task description\"`)")?;
    single_line("description", &desc)?;
    if let Some(ref c) = args.comment {
        single_line("comment", c)?;
    }
    validate_task_fields(args.position.as_deref(), args.due.as_deref(), args.tags.as_deref())?;

    let mut store = Store::load(cwd)?;
    let existing_ids: HashSet<String> = store.doc().tasks.iter().map(|t| t.id.clone()).collect();
    let new_id = generate_id(&existing_ids);

    // The refusal names the new ID, so it runs after generation and before any write.
    if args.annotate.is_some() && store.format() == StorageFormat::MdOnly {
        return Err(annotation::md_refusal(&new_id).into());
    }

    let mut new_task = Task::new(&new_id, &desc);
    if let Some(p) = args.priority {
        new_task.priority = parse_priority_arg(&p)?;
    }
    if let Some(t) = args.tags {
        new_task.tags = parse_tags(&t);
    }
    new_task.due = args.due;
    new_task.position = args.position;
    if let Some(c) = args.comment {
        new_task.comments.push(attach_comment(&mut store, &new_id, c));
    }
    if let Some(ref text) = args.annotate {
        annotation::append(&mut new_task, text, today)?;
    }

    store.add_task(new_task);
    store.save()?;
    let suffix = if args.annotate.is_some() {
        ", with annotation"
    } else {
        ""
    };
    writeln!(out, "{} Added task #{new_id} \"{desc}\"{suffix}", "✓".bright_green())?;
    Ok(())
}

fn add_comment(store: &mut Store, target_id: &str, text: String, out: &mut impl Write) -> CmdResult {
    if store.find_task_mut(target_id).is_none() {
        return Err(not_found(target_id).into());
    }
    let comment_id = attach_comment(store, target_id, text);
    if let Some(task) = store.find_task_mut(target_id) {
        task.comments.push(comment_id);
    }
    store.save()?;
    writeln!(out, "{} Added comment to task #{target_id}", "✓".bright_green())?;
    Ok(())
}

/// Adds a comment for `task_id` to the document and returns its new ID.
fn attach_comment(store: &mut Store, task_id: &str, text: String) -> String {
    let existing: HashSet<String> = store.doc().comments.iter().map(|c| c.id.clone()).collect();
    let comment_id = generate_id(&existing);
    store.doc_mut().comments.push(Comment {
        id: comment_id.clone(),
        text,
        task_id: task_id.to_string(),
        actors: Vec::new(),
        extra: BTreeMap::new(),
    });
    comment_id
}
