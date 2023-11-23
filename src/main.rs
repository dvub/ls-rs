use std::{fs::read_dir, path::Path};

use clap::Parser;
use colored::{ColoredString, Colorize};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to list files under
    path: String,

    /// Print directories and sub-directories without printing any contained files
    #[arg(short, long)]
    dirs: bool,

    #[arg(long)]
    max_depth: Option<usize>,
}
fn main() {
    let args = Args::parse();

    ls(Path::new(&args.path), args.dirs, 0, args.max_depth);
}

fn ls(path: &Path, dirs_only: bool, depth: usize, max_depth: Option<usize>) {
    if !path.exists() {
        return;
    }

    if !path.is_dir() && dirs_only {
        return;
    }

    if path.file_name().is_none() {
        return;
    }

    print_file(path, depth);

    if path.is_dir() {
        let result = read_dir(path).unwrap();
        result.for_each(|dir| {
            let entry = dir.unwrap().path();
            ls(&entry, depth + 1, dirs_only);
        });
    }
}

fn print_file(path: &Path, num_tabs: usize) {
    let mut tabs: String = String::new();
    for _ in 0..num_tabs {
        tabs.push_str("\t");
    }

    let mut file: ColoredString = path.file_name().unwrap().to_str().unwrap().into();
    if path.is_dir() {
        file = file.red().bold();
    } else if path.is_file() {
        file = file.blue();
    }
    println!("{} {}", tabs, file);
    // ∟⊢
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
