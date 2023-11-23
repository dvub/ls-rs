use std::{
    env::{self, join_paths},
    fs::read_dir,
    path::{Path, PathBuf},
};

use clap::Parser;
use colored::Colorize;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to list files under
    path: String,

    /// Optionally, list only directories
    #[arg(short = 'd', long)]
    dirs: Option<bool>,

    /// Optionally, list only files
    #[arg(short = 'f', long)]
    files: Option<bool>,
}

fn main() {
    let args = Args::parse();
    println!("listing all directories under {} ...", args.path);
    ls(Path::new(&args.path), 0);
}
fn ls(path: &Path, depth: usize) {
    if !path.is_dir() {
        return;
    }
    let mut tabs: String = String::new();
    for _ in 0..depth {
        tabs.push_str("\t");
    }

    let file_name = path.file_name().unwrap().to_str().unwrap().green().bold();
    println!("{} ∟ {}", tabs, file_name);

    let result = read_dir(path).unwrap();
    result.for_each(|dir| {
        let entry = dir.unwrap().path();
        ls(&entry, depth + 1);
    });
}

#[cfg(test)]
mod tests {
    const PARENT_TEST_DIR: &str = "test-dir";
    use std::{
        env,
        error::Error,
        fs::{create_dir, read_dir, remove_dir_all},
        io,
    };
    // creates a few test directories under the parent test directory
    fn setup() {
        let current_dir = env::current_dir();
        let target_dir = current_dir.unwrap().join(PARENT_TEST_DIR);
        create_dir(&target_dir).unwrap();
        let other_dirs = ["test1", "test2", "testmisc"];
        for dir in other_dirs {
            let path = target_dir.join(dir);
            create_dir(&path).unwrap();
        }
    }
    // deletes everything in the parent test directory, returning a result
    fn teardown() -> Result<(), io::Error> {
        let target = env::current_dir().unwrap().join(PARENT_TEST_DIR);
        remove_dir_all(&target)
    }

    #[test]
    fn test_setup() {
        let target = env::current_dir().unwrap().join(PARENT_TEST_DIR);
        setup();
        assert_eq!(read_dir(&target).unwrap().count(), 3);
    }
    #[test]
    fn test_teardown() {
        let target = env::current_dir().unwrap().join(PARENT_TEST_DIR);
        setup();
        assert!(teardown().is_ok());
        assert!(read_dir(&target).is_err());
    }
}
