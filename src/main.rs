extern crate clap;
extern crate indicatif; // Progress Bar Crate

use clap::{Arg, App, SubCommand};
use indicatif::ProgressBar;

/*
use std::fs;
use std::error::Error;
use std::path;
*/

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

    // Initializing the Progress Bar
    let _bar = ProgressBar::new(100);

    // Parsing the project name
    let project_name = matches.subcommand_matches("new").unwrap().value_of("name").unwrap();
    let path_name = format!("./{}", project_name);
    println!("Creating Project... {:?}", project_name);

    // Creating Project Directory & main.py;
    viper_utils::fh::create_boilerplate_files(&path_name);
    // Creating requirements.txt
    let requirements_file = viper_utils::fh::create_requirements_file(&path_name);

    // Parsing Module Arguments
    let mut modules = Vec::new();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("module") {
            println!("\nExternal Modules Specified: ");
            for m in matches.values_of("module").unwrap() {
                    println!("{:?}", m);
                    modules.push(m);
            }
        }

        if matches.is_present("freeze") {
            viper_utils::fh::freeze(2, &requirements_file);
        } else if matches.is_present("freeze3") {
            viper_utils::fh::freeze(3, &requirements_file);
        }
    }


    // Extension
    viper_utils::fh::set_requirements(modules, &requirements_file);
    viper_utils::cli::check_pip_version();

}
