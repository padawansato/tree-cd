use std::error::Error;
use std::fs;
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
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "L", default_value = "2")]
    level: usize,
}
fn run(dir: &Path, level: usize) {
    let default_depth = 0;
    let default_prefix = String::from("");
    visit_dirs(&dir, default_depth, level, default_prefix);
}

fn visit_dirs(dir: &Path, depth: usize, level: usize, prefix: String) {
    // if depth == level {
    //     return Ok(());
    // }
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect();
    let index = paths.len();
    paths.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));
    // println!("paths = {:?}", paths);
    // println!("index = {:?}", index);
    if dir.is_dir() {
        for (index, entry) in paths.iter().enumerate() {
            let path = entry.path();
            if index == paths.len() - 1 {
                println!("{}{}{}", prefix, &LAST_FILE, &path.display());
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix = prefix.clone() + &BLANK;
                    visit_dirs(&path, depth, level, prefix);
                }
            } else {
                println!("{}{}{}", prefix, &CROSS, &path.display());
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.clone() + &VERT;
                    visit_dirs(&path, depth, level, prefix_new);
                }
            }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("opt = {:?}", opt);
    run(&opt.path, opt.level)
}
