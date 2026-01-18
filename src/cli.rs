use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(name = "exrn")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File glob patterns to match (e.g. *.txt)
    #[arg(short, long, required = true, num_args = 1..)]
    pub sources: Vec<String>,

    /// Regex pattern and replacement (e.g. '^(.*)\.txt$' '$1.md')
    #[arg(short, long, num_args = 2, value_names = ["SOURCE_REGEX", "TARGET_PATTERN"])]
    pub rule: Vec<String>,

    /// Auto-confirm all prompts
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub yes: bool,

    /// Dry run: print planned changes and exit without modifying files
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Sort the output by source filename
    #[arg(long, action = ArgAction::SetTrue, default_value = "true")]
    pub sort: bool,
}
