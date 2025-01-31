use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use glob::glob;

// 将路径转换为绝对路径并规范化
fn normalize_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(path);
    Ok(if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    })
}

// 使用正则表达式验证文件名匹配
fn match_filename(
    file_path: &Path,
    source_regex: &Regex
) -> Result<bool, Box<dyn Error>> {
    let file_name = file_path.file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy();
    Ok(source_regex.is_match(&file_name))
}

// 执行正则替换生成新文件名
fn generate_new_path(
    file_path: &Path,
    source_regex: &Regex,
    target_pattern: &str
) -> Result<PathBuf, Box<dyn Error>> {
    let file_name = file_path.file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy();
    
    let new_name = source_regex.replace_all(&file_name, target_pattern);
    
    // 校验新文件名有效性
    if new_name.is_empty() {
        return Err("New file name cannot be empty".into());
    }
    if new_name.contains(std::path::MAIN_SEPARATOR) {
        return Err(format!("New name contains path separator: {}", new_name).into());
    }
    
    Ok(file_path.with_file_name(new_name.as_ref()))
}

// 执行重命名操作
fn safe_rename(source: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    if source == target {
        return Err("Source and target are the same".into());
    }
    if target.exists() {
        return Err("Target file already exists".into());
    }
    fs::rename(source, target)?;
    Ok(())
}

// 用户确认提示
fn confirm_action(prompt: &str) -> Result<bool, Box<dyn Error>> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("rex")
        .version("1.0")
        .about("Safe batch file renamer using regular expressions")
        .arg(Arg::new("sources")
            .required(true)
            .short('s')
            .long("sources")
            .num_args(1..)
            .value_name("GLOB_PATTERN")
            .help("File glob patterns to match (e.g. *.txt)"))
        .arg(Arg::new("rule")
            .required(true)
            .short('r')
            .long("rule")
            .num_args(2)
            .value_names(["SOURCE_REGEX", "TARGET_PATTERN"])
            .help("Regex pattern and replacement (e.g. '^(.*)\\.txt$' '$1.md')"))
        .arg(Arg::new("yes")
            .short('y')
            .long("yes")
            .help("Auto-confirm all prompts")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // 解析正则表达式
    let rules = matches.get_many::<String>("rule")
        .ok_or("Missing regex rules")?
        .collect::<Vec<_>>();
    let source_regex = Regex::new(rules[0])?;
    let target_pattern = rules[1];

    // 收集唯一匹配文件
    let mut matched_files = HashSet::new();
    for pattern in matches.get_many::<String>("sources").unwrap() {
        for entry in glob(pattern)? {
            let path = normalize_path(&entry?.to_string_lossy())?;
            if path.is_file() && match_filename(&path, &source_regex)? {
                matched_files.insert(path);
            }
        }
    }

    if matched_files.is_empty() {
        println!("No files matched the criteria");
        return Ok(());
    }

    // 生成重命名计划
    let mut rename_plan = Vec::new();
    for source in &matched_files {
        let target = generate_new_path(source, &source_regex, target_pattern)?;
        if source != &target {
            rename_plan.push((source.clone(), target));
        }
    }

    // 显示变更预览
    println!("Planned changes ({} files):", rename_plan.len());
    for (i, (src, dst)) in rename_plan.iter().enumerate() {
        println!("[{:2}] {} => {}", 
            i+1,
            src.file_name().unwrap().to_string_lossy(),
            dst.file_name().unwrap().to_string_lossy()
        );
    }

    // 用户确认
    let auto_confirm = matches.get_flag("yes");
    if !auto_confirm && !confirm_action("Confirm renaming?")? {
        println!("Operation cancelled");
        return Ok(());
    }

    // 执行重命名
    let mut success = 0;
    for (src, dst) in &rename_plan {
        match safe_rename(src, dst) {
            Ok(_) => success += 1,
            Err(e) => eprintln!("Error renaming {}: {}", src.display(), e),
        }
    }

    println!("Successfully renamed {}/{} files", success, rename_plan.len());
    Ok(())
}

