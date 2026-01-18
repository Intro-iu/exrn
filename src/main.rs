mod cli;
mod file_utils;
mod planner;
mod tui;

use clap::Parser;
use cli::Cli;
use colored::*;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write, IsTerminal};

fn confirm_action(prompt: &str) -> Result<bool, Box<dyn Error>> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Parse Regex
    let source_regex = Regex::new(&args.rule[0])?;
    let target_pattern = &args.rule[1];

    // Find Matches
    let matched_files = planner::find_matches(&args.sources, &source_regex)?;

    if matched_files.is_empty() {
        println!("{}", "No files matched the criteria".yellow());
        return Ok(());
    }

    // Generate Execution Plan (Topologically Sorted)
    let execution_plan = planner::generate_plan(matched_files, &source_regex, target_pattern)?;

    // Display Plan (Sorted for display if requested)
    let mut display_plan = execution_plan.clone();
    if args.sort {
        display_plan.sort_by(|a, b| a.source.cmp(&b.source));
    }

    // INTERACTIVE MODE CHECK
    // If not auto-confirm (-y), not dry-run, and is a TTY: use TUI
    if !args.yes && !args.dry_run && std::io::stdout().is_terminal() {
        // TUI Mode
        match tui::run_tui(display_plan)? {
            Some(selected_actions) => {
                if selected_actions.is_empty() {
                    println!("{}", "No files selected for renaming.".yellow());
                    return Ok(());
                }
                
                // Re-order selected actions according to the safe topological execution_plan
                // 1. Create a set of selected sources for fast lookup
                let selected_sources: HashSet<_> = selected_actions.iter().map(|a| &a.source).collect();
                
                // 2. Filter the original execution_plan
                let final_plan: Vec<_> = execution_plan
                    .into_iter()
                    .filter(|a| selected_sources.contains(&a.source))
                    .collect();

                execute_renames(final_plan);
            }
            None => {
                println!("{}", "Operation cancelled".yellow());
            }
        }
    } else {
        // Headless Mode (Standard Output)
        if std::io::stdout().is_terminal() {
            tui::print_preview(display_plan)?;
        } else {
            println!("Planned changes ({} files):", display_plan.len());
            for (i, action) in display_plan.iter().enumerate() {
                let src_name = action.source.file_name().unwrap_or_default().to_string_lossy();
                let dst_name = action.target.file_name().unwrap_or_default().to_string_lossy();
                
                println!(
                    "[{:>2}] {} {} {}",
                    (i + 1).to_string().dimmed(),
                    src_name.red(),
                    "=>".dimmed(),
                    dst_name.green()
                );
            }
        }

        if args.dry_run {
            println!("{}", "Dry run complete. No files were modified.".yellow());
            return Ok(());
        }

        // User Confirmation (Headless)
        if !args.yes && !confirm_action(&"Confirm renaming?".bold().to_string())? {
            println!("{}", "Operation cancelled".yellow());
            return Ok(());
        }

        // Execute SAFE plan (not display plan)
        execute_renames(execution_plan);
    }

    Ok(())
}

fn execute_renames(actions: Vec<planner::RenameAction>) {
    let mut success = 0;
    let total = actions.len();
    
    for action in &actions {
        match file_utils::safe_rename(&action.source, &action.target) {
            Ok(_) => success += 1,
            Err(e) => eprintln!("{} {}: {}", "Error renaming".red().bold(), action.source.display(), e),
        }
    }

    if success > 0 {
        println!(
            "{} {}/{} files",
            "Successfully renamed".green().bold(),
            success,
            total
        );
    } else {
         println!("{}", "No files renamed.".yellow());
    }
}


