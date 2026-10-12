use std::{env, process::ExitCode};

use blobstore::{cat_file::pretty_print, init::init};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [command] if command == "init" => match init() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Command execution failed: {error}");
                ExitCode::FAILURE
            }
        },
        [command, flag, hash] if command == "cat-file" && flag == "-p" => {
            match pretty_print(hash) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Command execution failed: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        [] => {
            println!("usage: \n\tbs init\n\tbs cat-file\n");
            ExitCode::SUCCESS
        }
        invalid => {
            eprintln!("Invalid command: {}", invalid.join(" "));
            ExitCode::FAILURE
        }
    }
}
