use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".",parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
    for entry in opt.path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}
