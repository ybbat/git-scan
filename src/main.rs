use std::{fs::DirEntry, iter::zip, path::PathBuf};

use clap::Parser;
use git2::Repository;
use owo_colors::OwoColorize;

/// Scans a directory tree for git repositories
#[derive(Parser)]
struct Cli {
    /// Directory to start the scan from
    #[arg(value_parser = parse_dir, default_value = ".")]
    path: PathBuf,

    /// How deep to scan
    #[arg(short, long, default_value = "3")]
    depth: u32,
}

fn main() {
    let args = Cli::parse();

    print_tree(args.path, args.depth);
}

fn parse_dir(arg: &str) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(arg);
    match path.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Ok(path.to_path_buf());
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Path is not a directory",
                ));
            }
        }
        Err(e) => return Err(e),
    }
}

const INDENT: &str = "   ";
const TRUNK: &str = "│  ";
const BRANCH: &str = "├──";
const FINAL_BRANCH: &str = "└──";

fn print_tree(path: PathBuf, depth: u32) {
    println!("{}", path.display());
    print_tree_recursive(path, depth, 0, String::from(""));
}

fn print_tree_recursive(path: PathBuf, max_depth: u32, cur_depth: u32, prefix: String) {
    if cur_depth >= max_depth {
        return;
    }

    let entries: Vec<_> = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return,
    }
    .filter(|entry| entry.is_ok() && entry.as_ref().unwrap().metadata().unwrap().is_dir())
    .collect();

    let count = entries.len();

    zip(entries.iter(), 0..count).for_each(|(entry, i)| match entry {
        Ok(entry) => {
            let (marker, next_pre) = if i == count - 1 {
                (FINAL_BRANCH, INDENT)
            } else {
                (BRANCH, TRUNK)
            };

            match Repository::open(entry.path()) {
                Ok(repo) => {
                    println!("{}{} {}", prefix, marker, format_repo(entry, repo));
                }
                Err(_) => {
                    println!("{}{} {}", prefix, marker, format_dir(entry));
                    print_tree_recursive(
                        path.join(entry.file_name()),
                        max_depth,
                        cur_depth + 1,
                        format!("{}{}", prefix, next_pre),
                    )
                }
            }
        }
        Err(_) => return,
    });
}

fn format_dir(dir: &DirEntry) -> String {
    let s = dir.file_name().to_string_lossy().to_string();
    return s.dimmed().to_string();
}

fn format_repo(dir: &DirEntry, repo: Repository) -> String {
    let s = dir.file_name().to_string_lossy().to_string();
    return s.green().to_string();
}
