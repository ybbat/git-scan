use std::path::PathBuf;

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

    println!(
        "children {:?}",
        args.path.read_dir().unwrap().collect::<Vec<_>>()
    );
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
