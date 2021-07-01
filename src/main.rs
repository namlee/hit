use std::{env, fs, path::Path};

use clap::{App, Arg, SubCommand};

fn main() {
    let command = App::new("Hit")
        .version("1.0")
        .author("Nam Le. <namle.0109@gmail.com>")
        .subcommand(
            SubCommand::with_name("init")
                .about("init project")
                .arg(Arg::with_name("directory").help("Init with exisiting root_pathsitory")),
        )
        .get_matches();

    // init command
    if let Some(match_cmds) = command.subcommand_matches("init") {
        // create path
        let mut git_path = match match_cmds.value_of("directory") {
            // get absolute path
            // TODO: assumpt that the user enter full path
            Some(dir) => Path::new(dir).to_path_buf(),
            // create current_dir -> get absolute path
            None => match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => panic!("Can not create directory, path: {}", e),
            },
        };
        println!(
            "Initialized empty Hit repository in {}",
            git_path.as_path().display()
        );
        // create .git
        git_path.push(".git");
        let path = git_path.as_path();
        // create .objects, .ref dirs
        let dirs = ["objects", "refs"];
        for dir in dirs {
            let full_path = path.join(dir);
            match fs::create_dir_all(full_path) {
                Ok(_) => (),
                Err(e) => panic!("Can not create directory {:?} {}", path.join(dir), e),
            }
        }
    }
}
