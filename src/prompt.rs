use std::env;
use colored::Colorize;
use std::fs;
use std::path::Path;

fn get_git_branch() -> Option<String> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        return None;
    }
    let head_path = git_dir.join("HEAD");
    if let Ok(head_content) = fs::read_to_string(head_path) {
        let branch_ref = head_content.trim_start_matches("ref: ").trim();
        let branch_name = branch_ref.strip_prefix("refs/heads/").unwrap_or(branch_ref);
        return Some(branch_name.to_owned());
    }

    None
}

pub fn display_prompt() {
    let current_dir = env::current_dir().unwrap();
    let home_dir = env::var("HOME").unwrap();
    let current_dir_str = current_dir.to_string_lossy().replace(&home_dir, "~");
    let colored_prompt = current_dir_str.blue();
    let mut colored_branch = String::new();
    if let Some(branch) = get_git_branch() {
        colored_branch = branch.red().italic().to_string();
    }
    println!("{} {} >", colored_prompt, colored_branch);
}