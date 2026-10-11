use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".bs").unwrap();
        fs::create_dir(".bs/objects").unwrap();
        fs::create_dir(".bs/refs").unwrap();
        fs::write(".bs/HEAD", "ref: refs/heads/master\n").unwrap();
        println!("Initialized blobstore directory");
    } else {
        println!("unknown command: {}", args[1]);
    }
}
