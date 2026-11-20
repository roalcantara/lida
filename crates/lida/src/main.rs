//! lida: Dual-format project task engine CLI (YAML · MD).

#![forbid(unsafe_code)]

mod cli;

use cli::{LidaCli, Subcommand};
use lida_core::id::generate_id;
use lida_core::model::{Comment, Priority, Status, Task};
use lida_core::store::{StorageFormat, Store};
use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = LidaCli::parse();

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}: {err}", "error".bright_red().bold());
            ExitCode::FAILURE
        }
    }
}

fn run(cli: LidaCli) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;

    let cmd = match cli.command {
        Some(cmd) => cmd,
        None => Subcommand::List {
            tasks: false,
            status: None,
            tag: None,
            priority: None,
            json: false,
        },
    };

    match cmd {
        Subcommand::Init { yaml, md, both, project } => {
            let format = if both {
                StorageFormat::Both
            } else if md {
                StorageFormat::MdOnly
            } else if yaml {
                StorageFormat::YamlOnly
            } else {
                StorageFormat::YamlOnly
            };
            Store::init(&cwd, format, project)?;
            println!("{} Initialized project tasks in {:?}", "✓".bright_green(), cwd);
        }

        Subcommand::Add {
            description,
            task,
            priority,
            tags,
            due,
            position,
            comment,
        } => {
            let desc = description
                .or(task)
                .ok_or_else(|| "task description is required (e.g. `lida add \"Task description\"`)")?;

            let mut store = Store::load(&cwd)?;
            let existing_ids: HashSet<String> = store.doc().tasks.iter().map(|t| t.id.clone()).collect();
            let new_id = generate_id(&existing_ids);

            let mut new_task = Task::new(&new_id, &desc);
            if let Some(p) = priority {
                new_task.priority = match p.to_lowercase().as_str() {
                    "high" => Some(Priority::High),
                    "medium" => Some(Priority::Medium),
                    "low" => Some(Priority::Low),
                    _ => None,
                };
            }
            if let Some(t) = tags {
                new_task.tags = t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            if let Some(d) = due {
                new_task.due = Some(d);
            }
            if let Some(pos) = position {
                new_task.position = Some(pos);
            }

            if let Some(c) = comment {
                let existing_comment_ids: HashSet<String> = store.doc().comments.iter().map(|c| c.id.clone()).collect();
                let comment_id = generate_id(&existing_comment_ids);
                store.doc_mut().comments.push(Comment {
                    id: comment_id.clone(),
                    text: c,
                    task_id: new_id.clone(),
                    actors: Vec::new(),
                });
                new_task.comments.push(comment_id);
            }

            store.add_task(new_task);
            store.save()?;

            println!(
                "{} Added task #{new_id} \"{desc}\"",
                "✓".bright_green(),
            );
        }

        Subcommand::List { tasks, status, tag, priority, json } => {
            let store = Store::load(&cwd)?;
            let mut items: Vec<&Task> = store.doc().tasks.iter().collect();

            if let Some(s) = status {
                items.retain(|t| t.status.as_str().eq_ignore_ascii_case(&s));
            }
            if let Some(tg) = tag {
                items.retain(|t| t.tags.iter().any(|t_tag| t_tag.eq_ignore_ascii_case(&tg)));
            }
            if let Some(p) = priority {
                items.retain(|t| t.priority.map_or(false, |p_val| p_val.as_str().eq_ignore_ascii_case(&p)));
            }

            if json {
                println!("{}", serde_json::to_string_pretty(&items)?);
                return Ok(());
            }

            if tasks {
                // Plain format for completion compatibility: [Status] [Priority] <id> — <description>
                for t in items {
                    let st = t.status.as_str();
                    let prio = t.priority.map_or("".to_string(), |p| format!("[{}] ", p.as_str()));
                    println!("[{st}] {prio}{} — {}", t.id, t.description);
                }
                return Ok(());
            }

            // Tabular human-friendly output
            if items.is_empty() {
                println!("{}", "No tasks found.".dimmed());
                return Ok(());
            }

            println!(
                "{:<6} {:<12} {:<10} {:<30} {}",
                "ID".bold(),
                "STATUS".bold(),
                "PRIORITY".bold(),
                "TITLE".bold(),
                "TAGS".bold()
            );
            println!("{}", "─".repeat(70).dimmed());

            for t in items {
                let status_display = match t.status {
                    Status::Todo => "Todo".dimmed().to_string(),
                    Status::InProgress => "InProgress".bright_yellow().to_string(),
                    Status::Done => "Done".bright_green().to_string(),
                    Status::Blocked => "Blocked".bright_red().to_string(),
                };

                let prio_display = t.priority.map_or("-".dimmed().to_string(), |p| match p {
                    Priority::High => "High".bright_red().to_string(),
                    Priority::Medium => "Medium".bright_yellow().to_string(),
                    Priority::Low => "Low".dimmed().to_string(),
                });

                let tags_display = if t.tags.is_empty() {
                    "-".dimmed().to_string()
                } else {
                    t.tags.join(", ").dimmed().to_string()
                };

                println!(
                    "#{:<5} {:<12} {:<10} {:<30} {}",
                    t.id.bold(),
                    status_display,
                    prio_display,
                    t.description,
                    tags_display
                );
            }
        }

        Subcommand::Done { id } => {
            let mut store = Store::load(&cwd)?;
            let task = store.find_task_mut(&id).ok_or_else(|| format!("task #{id} not found"))?;
            task.status = Status::Done;
            store.save()?;
            println!("{} Task #{} marked as done", "✓".bright_green(), id.bold());
        }

        Subcommand::Start { id } => {
            let mut store = Store::load(&cwd)?;
            let task = store.find_task_mut(&id).ok_or_else(|| format!("task #{id} not found"))?;
            task.status = Status::InProgress;
            store.save()?;
            println!("{} Task #{} marked as in-progress", "✓".bright_green(), id.bold());
        }

        Subcommand::Status { id, set, reason } => {
            let mut store = Store::load(&cwd)?;
            let status: Status = serde_json::from_value(serde_json::Value::String(set))?;
            let task = store.find_task_mut(&id).ok_or_else(|| format!("task #{id} not found"))?;
            task.status = status;
            if status == Status::Blocked {
                task.blocked_reason = reason;
            }
            let status_str = task.status.as_str();
            store.save()?;
            println!("{} Task #{} status set to {}", "✓".bright_green(), id.bold(), status_str);
        }

        Subcommand::Show { id, json } => {
            let store = Store::load(&cwd)?;
            let task = store.doc().tasks.iter().find(|t| t.id == id).ok_or_else(|| format!("task #{id} not found"))?;

            if json {
                println!("{}", serde_json::to_string_pretty(task)?);
            } else {
                println!("Task #{}: {}", task.id.bold(), task.description);
                println!("  Status: {}", task.status.as_str());
                if let Some(p) = task.priority {
                    println!("  Priority: {}", p.as_str());
                }
                if !task.tags.is_empty() {
                    println!("  Tags: {}", task.tags.join(", "));
                }
                if let Some(pos) = &task.position {
                    println!("  Position: {pos}");
                }
                if !task.meta.is_empty() {
                    println!("  Metadata:");
                    for (k, v) in &task.meta {
                        println!("    {k}: {v}");
                    }
                }
            }
        }

        Subcommand::Meta { id, action, key, value } => {
            let mut store = Store::load(&cwd)?;
            let task = store.find_task_mut(&id).ok_or_else(|| format!("task #{id} not found"))?;

            match action.to_lowercase().as_str() {
                "set" => {
                    let val_str = value.ok_or_else(|| "value required for `meta set <key> <value>`")?;
                    let val = serde_json::from_str(&val_str).unwrap_or_else(|_| serde_json::Value::String(val_str));
                    task.meta.insert(key.clone(), val);
                    store.save()?;
                    println!("{} Set metadata {} for task #{}", "✓".bright_green(), key.bold(), id.bold());
                }
                "get" => {
                    if let Some(v) = task.meta.get(&key) {
                        if let serde_json::Value::String(s) = v {
                            println!("{s}");
                        } else {
                            println!("{v}");
                        }
                    } else {
                        return Err(format!("metadata key '{key}' not set on task #{id}").into());
                    }
                }
                "unset" | "rm" => {
                    task.meta.remove(&key);
                    store.save()?;
                    println!("{} Removed metadata {} from task #{}", "✓".bright_green(), key.bold(), id.bold());
                }
                _ => return Err(format!("unknown meta action '{action}'. Use set, get, or unset").into()),
            }
        }

        Subcommand::Tags => {
            let store = Store::load(&cwd)?;
            let mut counts: HashMap<String, usize> = HashMap::new();
            for t in &store.doc().tasks {
                for tag in &t.tags {
                    *counts.entry(tag.clone()).or_insert(0) += 1;
                }
            }
            let mut sorted: Vec<_> = counts.into_iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (tag, count) in sorted {
                println!("  {:<15} {:>4}", tag, count);
            }
        }

        Subcommand::Delete { id } => {
            let mut store = Store::load(&cwd)?;
            if store.remove_task(&id) {
                store.save()?;
                println!("{} Deleted task #{}", "✓".bright_green(), id.bold());
            } else {
                return Err(format!("task #{id} not found").into());
            }
        }

        Subcommand::Cwi { format } => {
            let mut store = Store::load(&cwd)?;
            if let Some(fmt_str) = format {
                let fmt = match fmt_str.to_lowercase().as_str() {
                    "both" => StorageFormat::Both,
                    "md" => StorageFormat::MdOnly,
                    "yaml" => StorageFormat::YamlOnly,
                    _ => return Err("format must be 'yaml', 'md', or 'both'".into()),
                };
                store.set_format(fmt);
                store.save()?;
                println!("{} Switched working format to {:?}", "✓".bright_green(), fmt);
            } else {
                println!("Current working format: {:?}", store.format());
            }
        }

        Subcommand::Usage => {
            println!("# lida.usage.kdl");
            println!("name \"lida\"");
            println!("bin \"lida\"");
            println!("version \"0.1.0\"");
            println!("about \"Dual-format project task engine (YAML · MD)\"");
        }
    }

    Ok(())
}
