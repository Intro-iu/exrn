mod cli;
mod file_utils;
mod planner;

use clap::Parser;
use cli::Cli;
use colored::*;
use regex::Regex;
use std::error::Error;
use std::io::{self, Write};

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

    println!("Planned changes ({} files):", display_plan.len());
    for (i, action) in display_plan.iter().enumerate() {
        let src_name = action.source.file_name().unwrap().to_string_lossy();
        let dst_name = action.target.file_name().unwrap().to_string_lossy();
        
        println!(
            "[{:>2}] {} {} {}",
            (i + 1).to_string().dimmed(),
            src_name.red(),
            "=>".dimmed(),
            dst_name.green()
        );
    }

    if args.dry_run {
        println!("{}", "Dry run complete. No files were modified.".yellow());
        return Ok(());
    }

    // User Confirmation
    if !args.yes && !confirm_action(&"Confirm renaming?".bold().to_string())? {
        println!("{}", "Operation cancelled".yellow());
        return Ok(());
    }

    // Execute
    let mut success = 0;
    for action in &execution_plan {
        match file_utils::safe_rename(&action.source, &action.target) {
            Ok(_) => success += 1,
            Err(e) => eprintln!("{} {}: {}", "Error renaming".red().bold(), action.source.display(), e),
        }
    }

    println!(
        "{} {}/{} files",
        "Successfully renamed".green().bold(),
        success,
        execution_plan.len()
    );

    Ok(())
}


