use std::error::Error;
use std::fs;
use std::io;
use std::io::{stdin, stdout, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

static LAST_FILE: &str = "└───";
static CROSS: &str = "├───";
static BLANK: &str = "\u{00A0}\u{00A0}\u{00A0}\u{00A0}";
static VERT: &str = "│   ";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "DIRECTORY", default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "L", default_value = "3")]
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
    if depth == level {
        return;
    }

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
const MESSAGE: &str = "Tree";

fn main() -> Result<(), io::Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    write!(stdout, "{}Please, Push T!!", cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let opt = Opt::from_args();

    for c in stdin.keys() {
        match c {
            Ok(_) => {
                if let Ok((width, height)) = terminal_size() {
                    let x = 1 as u16;
                    let y = 1;
                    write!(
                        stdout,
                        "{}{}{}{}{}",
                        clear::All,
                        cursor::Goto(x, y),
                        color::Fg(color::Blue),
                        style::Bold,
                        style::Reset,
                    )
                    .unwrap();
                    run(&opt.path, opt.level, opt.all);
                    stdout.flush().unwrap();
                }
            }
            Ok(event::Key::Ctrl('c')) => break,
            _ => {}
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}
