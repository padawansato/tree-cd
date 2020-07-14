use std::error::Error;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

// mod dirsign {
//     pub const HORZ: char = '─';
//     pub const CROSS: char = '├';
//     pub const VERT: char = '│';
//     pub const LAST_FILE: char = '└';
//     pub const BLANK: char = '\u{00A0}';
// }
static LAST_FILE: &str = "└───";
static CROSS: &str = "├───";
static BLANK: &str = "\u{00A0}\u{00A0}\u{00A0}\u{00A0}";
static VERT: &str = "│   ";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "DIRECTORY", default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "L", default_value = "2")]
    level: usize,

    #[structopt(short = "a")]
    all: bool,
}

fn run(dir: &Path, level: usize, all: bool) -> Result<(), Box<Error>> {
    let default_depth = 0;
    let default_prefix = String::from("");
    visit_dirs(&dir, default_depth, level, default_prefix, all);
    Ok(())
}

fn visit_dirs(dir: &Path, depth: usize, level: usize, prefix: String, all: bool) {
    // if depth == level {
    //     Ok();
    // }

    if dir.is_dir() {
        let entry_set = fs::read_dir(&dir).unwrap(); // contains DirEntry
        let mut entries = entry_set
            .filter_map(|v| match v.ok() {
                Some(v) => {
                    if all {
                        return Some(v);
                    } else {
                        if v.file_name().to_str()?.starts_with(".") {
                            return None;
                        } else {
                            Some(v)
                        }
                    }
                }
                None => None,
            })
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));
        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            if index == entries.len() - 1 {
                println!("{}{}{:?}", prefix, &LAST_FILE, filename);
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix = prefix.clone() + &BLANK;
                    visit_dirs(&path, depth, level, prefix, all);
                }
            } else {
                println!("{}{}{:?}", prefix, &CROSS, filename);
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.clone() + &VERT;
                    visit_dirs(&path, depth, level, prefix_new, all);
                }
            }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("opt = {:?}", opt);
    run(&opt.path, opt.level, opt.all);
}
