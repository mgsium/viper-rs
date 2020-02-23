extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs;
use std::error::Error;
use std::path;

// mod lib;
// use lib::viper_utils;

fn main() {
    // Defining command, subcommands and options
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
                        .arg(Arg::with_name("module")
                            .short("m")
                            .long("module")
                            .help("Specify an external package to include in the project in the format <modulename>@<version>.")
                            .multiple(true)
                            .takes_value(true)
                        )
                        .arg(Arg::with_name("freeze")
                            .short("f")
                            .long("freeze")
                            .help("Specify installed modules (provided by 'pip freeze') as requirements")
                            .takes_value(false)
                        )
                        .arg(Arg::with_name("freeze3")
                            .short("F")
                            .long("freeze3")
                            .help("Specify installed modules (provided by 'pip3 freeze') as requirements")
                            .takes_value(false)
                        )
                    )
                    .get_matches();

    // Parsing the project name
    let ProjectName = matches.subcommand_matches("new").unwrap().value_of("name").unwrap();
    let PathName = format!("./{}", ProjectName);
    println!("Creating Project... {:?}", ProjectName);

    // Creating the Project Directing
    fs::create_dir_all(&PathName);

    // Creating the main.py file, error checking
    let mut filePath = format!("{}/main.py", PathName);
    let path = path::Path::new(&filePath);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Parsing Module Arguments
    let mut modules = Vec::new();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("module") {
            println!("External Modules Specified: ");
            for m in matches.values_of("module").unwrap() {
                    println!("{:?}", m);
                    modules.push(m);
            }
        }
    }

    // Extension
    viper_utils::fh::set_requirements(modules);

}
