use std::fs;
use std::io;
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
static CROSS: &str = "├──";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}
fn run(dir: &Path) {
    visit_dirs(&dir, String::from(""))
}

fn visit_dirs(dir: &Path, prefix: String) {
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
                println!("{}{}{:?}", prefix, &LAST_FILE, &path.display())
            } else {
                println!("{}{}{}", prefix, &CROSS, &path.display())
            }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("opt = {:?}", opt);
    run(&opt.path)
}
