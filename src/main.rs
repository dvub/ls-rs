use std::{fs::read_dir, path::Path};

use clap::Parser;
use colored::{ColoredString, Colorize};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to list files under
    path: String,

    /// Print only directories without printing any contained files
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
    // base case checks pretty much
    // if there's a max depth and we're above it, stop,
    // if the path doesn't exist, stop here
    // if the path is a file and we marked dirs_only then stop
    if max_depth.map_or(false, |d| depth >= d)
        || (!path.is_dir() && dirs_only)
        || path.file_name().is_none()
    {
        return;
    }
    // this would go above but we should print a helpful message instead of simply returning
    if !path.exists() {
        println!("{}: directory doesn't exist. Exiting...", "Error".red());
        return;
    }
    // here we'll actually print out the file name
    print_file(path, depth);

    // todo.. check if path is a directory before read_dir() ...

    if let Ok(result) = read_dir(path) {
        result.for_each(|dir| {
            if let Ok(entry) = dir {
                // recursion
                ls(&entry.path(), dirs_only, depth + 1, max_depth);
            }
        });
    }
}

fn print_file(path: &Path, num_tabs: usize) {
    // unwrapping a lot because we've already made all of these checks in the ls function
    // so this should be ok
    // as for to_str().unwrap(), idk.
    let file_name = path.file_name().unwrap();

    if file_name.to_str().is_none() {
        return;
    }
    let mut file: ColoredString = file_name.to_str().unwrap().into();

    if file.starts_with('.') {
        return;
    }

    // generate the number of indents/tabs before printing
    let mut tabs: String = String::new();
    for _ in 0..num_tabs {
        tabs.push('\t');
    }

    // color based on file/dir
    // todo: improve this ...?
    if path.is_dir() {
        file = file.red().bold();
    } else if path.is_file() {
        file = file.blue();
    }
    // print
    println!("{}{}", tabs, file);
    // possibly use these symbols or something idk
    // ∟⊢
}
