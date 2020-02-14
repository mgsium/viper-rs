extern crate clap;

use clap::{Arg, App, SubCommand};

use std::fs;
use std::error::Error;
use std::path;

fn main() {

    let matches = App::new("viper")
                    .version("0.1")
                    .author("Musab G. <musabgumaa@gmail.com>")
                    .arg(Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .help("Creates a venv for the project."))
                    .arg(Arg::with_name("dependencies")
                        .short("D")
                        .long("dependencies")
                        .help("Specify Dependencies for the project.")
                        .multiple(true)
                        .takes_value(true))
                    .subcommand(SubCommand::with_name("new")
                        .about("Creates a new project.")
                        .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .help("Specify the name for the project")
                            .required(true)
                            .index(1))
                    )
                    .get_matches();
    
    let ProjectName = matches.subcommand_matches("new").unwrap().value_of("name").unwrap();
    let PathName = format!("./{}", ProjectName);
    println!("{:?}", ProjectName);

    fs::create_dir_all(&PathName);

    let mut filePath = format!("{}/main.py", PathName);
    let path = path::Path::new(&filePath);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
}
