use std::path::Path;
use std::process::exit;
use walkdir::WalkDir;

use clap::Parser;

/// contents: summarize directory contents (total items / file count / directory count)
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// parent directory path
    #[arg(value_name = "DIR_PATH")]
    parent_directory: String,
}

/// Returns array with file and directory count within provided
/// parent directory with structure [`FILE_COUNT`, `DIR_COUNT`]
fn count_contents(parent_directory: &str) -> [usize; 2] {
    let mut dir_acc: usize = 0;
    let mut file_acc: usize = 0;

    for item in WalkDir::new(parent_directory).min_depth(1) {
        match item {
            Ok(entry) => {
                let path = entry.into_path();
                if path.is_dir() {
                    dir_acc += 1;
                } else if path.is_file() {
                    file_acc += 1;
                }
            }
            Err(_) => continue,
        }
    }
    [file_acc, dir_acc]
}

fn main() {
    let args = Args::parse();
    let parent_directory = &args.parent_directory;

    let dir_path = Path::new(parent_directory);
    if !(dir_path.exists() && dir_path.is_dir()) {
        eprintln!("contents: error: directory not found");
        exit(1);
    }

    let [file_acc, dir_acc] = count_contents(parent_directory);
    let total_acc = file_acc + dir_acc;
    println!("{total_acc} items / {file_acc} files / {dir_acc} dirs");
}
