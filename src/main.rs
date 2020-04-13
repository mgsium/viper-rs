#![allow(dead_code)]

extern crate clap;
extern crate indicatif; // Progress Bar Crate

// Crate Directives
// ----------------------------------------------------------------------------------------
use clap::{Arg, App, SubCommand};
use indicatif::ProgressBar;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs;
// ----------------------------------------------------------------------------------------


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
                        .arg(Arg::with_name("env")
                            .short("e")
                            .long("env")
                            .help("Creates a venv for the project."))
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
                        .arg(Arg::with_name("importd")
                            .short("d")
                            .long("importd")
                            .help("Import dependencies from a file.")
                            .takes_value(true)
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
    let mut venv: bool = false;
    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("env") {
            venv = true;
        }
    }
    viper_utils::fh::create_boilerplate_files(&path_name, venv);

    if (venv) {
        // Checking pip version
        viper_utils::cli::check_pip_version();

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

            if matches.is_present("importd") {
                // Parsing filename
                let filename = matches.value_of("importd").unwrap();

                // Opening and Reading from file
                let data = fs::read_to_string(filename).expect("!Error: unable to read dependencies file.");
            }

            // Extension
            viper_utils::fh::set_requirements(modules, &requirements_file);
        }
    } else {
        if let Some(matches) = matches.subcommand_matches("new") {
            if matches.is_present("module")
              || matches.is_present("freeze")
              || matches.is_present("freeze3")
              || matches.is_present("importd"){
                println!("\n!Cannot add dependencies: venv not specified")
            }
        }
    }

    viper_utils::cli::install_git(&path_name);

}
