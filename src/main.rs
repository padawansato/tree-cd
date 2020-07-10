use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

// fn visit_dirs(dir: &Path) -> io::Result<()> {
//     let mut paths: Vec<_> = fs::read_dir(dir)?
//         .map(|entry| entry.unwrap().path())
//         .collect();
// }

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
    for entry in opt.path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{}├── {:?}", String::from(""), entry.path());
        }
    }
}
