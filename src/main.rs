use std::{iter::zip, path::PathBuf};

use clap::Parser;

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

const TRUNK: &str = "│  ";
const BRANCH: &str = "├──";
const FINAL_BRANCH: &str = "└──";

fn print_tree(path: PathBuf, depth: u32) {
    println!("{}", path.display());
    print_tree_recursive(path, depth, 0);
}

fn print_tree_recursive(path: PathBuf, max_depth: u32, cur_depth: u32) {
    if cur_depth >= max_depth {
        return;
    }

    let entries: Vec<_> = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return,
    }
    .collect();

    let count = entries.len();

    zip(entries.iter(), 0..count).for_each(|(entry, i)| match entry {
        Ok(entry) => {
            if !entry.metadata().unwrap().is_dir() {
                return;
            }

            let is_last = i == count - 1;
            let mut prefix = if cur_depth > 0 {
                TRUNK.repeat(cur_depth as usize)
            } else {
                String::new()
            };
            prefix += if is_last { FINAL_BRANCH } else { BRANCH };
            println!("{}{}", prefix, entry.file_name().to_string_lossy());
            print_tree_recursive(path.join(entry.file_name()), max_depth, cur_depth + 1)
        }
        Err(_) => return,
    });
}
