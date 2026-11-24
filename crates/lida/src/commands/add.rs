//! `lida add`: create a task, or attach a comment to one with `--task-id`.

use super::prelude::*;
use super::{not_found, parse_priority_arg, parse_tags};
use lida_core::id::generate_id;
use lida_core::model::{Comment, Task};
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
}

pub fn run(cwd: &Path, args: Args, out: &mut impl Write) -> CmdResult {
    let mut store = Store::load(cwd)?;
    if let Some(target_id) = args.task_id {
        let text = args
            .comment
            .or(args.description)
            .ok_or("comment text is required when specifying --task-id")?;
        return add_comment(&mut store, &target_id, text, out);
    }

    let desc = args
        .description
        .ok_or("task description is required (e.g. `lida add \"Task description\"`)")?;
    let existing_ids: HashSet<String> = store.doc().tasks.iter().map(|t| t.id.clone()).collect();
    let new_id = generate_id(&existing_ids);

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

    store.add_task(new_task);
    store.save()?;
    writeln!(out, "{} Added task #{new_id} \"{desc}\"", "✓".bright_green())?;
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
