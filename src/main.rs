use clap::{Arg, Command};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::error::Error;
use glob::glob;
use std::io::{self, Write};

fn make_absolute_path(relative_path: &str) -> PathBuf {
    let mut absolute_path = std::env::current_dir().expect("Failed to get current directory");
    absolute_path.push(relative_path);
    absolute_path
}

fn match_by_regex(
    source: &str,             // 源文件名称
    source_pattern: &str      // 匹配源文件的正则表达式
) -> bool {
    // 编译源文件正则表达式
    let re = Regex::new(source_pattern).unwrap();
    // 使用正则表达式匹配文件名
    let captures = re.captures(source);

    // 如果匹配成功，返回匹配的文件名
    if let Some(_captures) = captures {
        return true;
    } else {
        return false;
    }
}

// 文件或文件夹重命名函数，基于正则表达式
fn rename_by_regex(
    source: &str,             // 源文件名称
    source_pattern: &str,     // 匹配源文件的正则表达式
    target_pattern: &str      // 重命名目标的正则化表达式
) -> (PathBuf, PathBuf) {
    // 编译源文件正则表达式
    let re = Regex::new(source_pattern).unwrap();
    // 使用目标模式替换匹配项生成新文件名
    let new_name = re.replace(source, target_pattern);

    // 进行重命名操作
    let source_path = Path::new(source).to_path_buf();
    let new_path = source_path.with_file_name(new_name.to_string());

    let item = (source_path, new_path);

    item
}

fn apply_rename(
    source: &Path,            // 源文件路径
    target: &Path             // 目标文件路径
) -> Result<(), Box<dyn Error>> {
    // 检查目标文件是否存在
    if target.exists() {
        return Err("Target file already exists".into());
    }

    // 重命名文件
    fs::rename(source, target)?;

    Ok(())
}

// 询问用户是否确认继续操作
fn confirm_rename() -> bool {
    // 提示用户输入 y 或 n
    print!("Do you want to proceed with the renaming of all matched files? [y/N]: ");
    io::stdout().flush().unwrap(); // 确保提示立即显示

    // 读取用户输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // 将输入转为小写并去除空格、换行符
    let input = input.trim().to_lowercase();

    // 检查用户是否输入了 'y' 或 'yes'
    input == "y" || input == "yes"
}

fn main() -> Result<(), Box<dyn Error>> {
    // 使用clap解析命令行参数
    let matches = Command::new("exrn")
        .arg(Arg::new("src")    // 源文件名或通配符
            .required(true)
            .num_args(1..)
            .short('s')
            .long("src")
            .value_name("SOURCE_PATTERN")
            .help("Sets the source file name or pattern (e.g. *.txt)"))
        .arg(Arg::new("rule")   // 两个正则表达式
            .required(true)
            .num_args(2)
            .short('r')
            .long("rule")
            .value_names(&["SOURCE_REGEX", "TARGET_REGEX"]) // 源文件名正则表达式，目标文件名正则表达式
            .help("Sets two regular expressions to match & rename"))
        .arg(Arg::new("yes")    // 是否跳过确认提示
            .required(false)
            .short('y')
            .long("yes")
            .help("Confirms automatically with yes")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // 获取命令行参数
    if let (Some(source_pattern), Some(reg_exp)) = (
        matches.get_many::<String>("src"), 
        matches.get_many::<String>("rule"),
    ) {
        let source_glob: Vec<_> = source_pattern.collect();
        let regs: Vec<_> = reg_exp.collect();
        let source_regex = &regs[0];
        let target_regex = &regs[1];

        // 是否跳过确认提示
        let skip_confirmation = matches.get_flag("yes");

        // 使用glob解析支持通配符的文件路径
        let mut matched_files = vec![];

        println!("Matching files...");
        for source in &source_glob {
            for entry in glob(&source)? {
                match entry {
                    Ok(path) => {
                        let file_name = path.to_string_lossy().to_string();
                        if match_by_regex(&file_name, source_regex) {
                            matched_files.push(file_name);
                        }
                    }
                    Err(e) => eprintln!("Error accessing file: {}", e),
                }
            }
        }

        // 如果有匹配文件
        if !matched_files.is_empty() {
            // 对所有匹配的文件进行重命名
            let mut items = Vec::new();
            for file_name in matched_files {
                let absolute_path = make_absolute_path(&file_name).display().to_string();
                items.push(rename_by_regex(&absolute_path, source_regex, target_regex));
                println!("\t{} -> {}", items.last().unwrap().0.display(), items.last().unwrap().1.display());
            }

            // 如果没有 '-y' 参数，询问用户是否确认操作
            if !skip_confirmation && !confirm_rename() {
                println!("Operation cancelled.");
                return Ok(());
            }

            for (source, target) in items {
                let result = apply_rename(&source, &target);
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error renaming file: {}", e),
                }
            }
        } else {
            println!("No files matched the pattern.");
        }
    } else {
        println!("No regular expression provided");
    }

    Ok(())
}
