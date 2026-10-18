use std::{env, process::ExitCode};

use blobstore::{
    cat_file::pretty_print,
    error::{BlobstoreError::InvalidCommand, Result, USAGE},
    hash_object::hash_object,
    init::init,
    ls_tree::ls_tree,
};

fn run(args: &[String]) -> Result<()> {
    match args {
        [cmd] if cmd == "init" => init(),
        [cmd, flag, hash] if cmd == "cat-file" && flag == "-p" => pretty_print(hash),
        [cmd, flag, file] if cmd == "hash-object" && flag == "-w" => hash_object(file),
        [cmd, flag, hash] if cmd == "ls-tree" && flag == "--name-only" => ls_tree(hash),
        [] => {
            println!("{USAGE}");
            Ok(())
        }
        _ => Err(InvalidCommand(args.join(" "))),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
