#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate clap;
extern crate indicatif; // Progress Bar Crate
extern crate json;
#[macro_use] extern crate prettytable;

mod control;

// Crate Directives
// ----------------------------------------------------------------------------------------
use clap::{Arg, App, SubCommand, AppSettings};
use indicatif::ProgressBar;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs;
use std::error::Error;
use std::path;
use std::any::type_name;
use control::tabling;
// ----------------------------------------------------------------------------------------


/*
use std::fs;
use std::error::Error;
use std::path;
*/

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // Defining command, subcommands and options
    let matches = App::new("viper")
                    .version("0.3.0")
                    .author("Musab G. <musabgumaa@gmail.com>")
                    .setting(AppSettings::ArgRequiredElseHelp)
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
                        .arg(Arg::with_name("pack")
                            .short("p")
                            .long("pack")
                            .help("Create a template using the project settings. Saved in the project root directory.")
                            .takes_value(true)
                        )
                    )
                    .subcommand(SubCommand::with_name("template")
                        .about("Creates a project template.")
                        .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .help("Specify the name for the location.")
                            .takes_value(true)
                            .required(true)
                            .index(1)
                        )
                        .arg(Arg::with_name("location")
                            .short("l")
                            .long("location")
                            .help("Specify where to save the new template. Default: ")
                            .takes_value(true)
                            .required(false)
                            .index(2)
                        )
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
                    .subcommand(SubCommand::with_name("build")
                        .about("Builds a project from a json template.")
                        .arg(Arg::with_name("path")
                            .short("p")
                            .long("path")
                            .help("Path of the template from which to build the project.")
                            .required(true)
                            .takes_value(true)
                            .index(1)
                        )
                        .arg(Arg::with_name("name")
                            .short("n")
                            .long("name")
                            .help("Specify the name for the project (including the full or relative path).")
                            .required(true)
                            .index(2)
                        )
                    )
                    .subcommand(SubCommand::with_name("remove")
                        .about("Remove a template/project.")
                        .subcommand(SubCommand::with_name("project")
                            .about("Remove a project.")
                            .arg(Arg::with_name("index")
                                .short("i")
                                .long("index")
                                .help("specify the index of the project you want to remove (view with viper list -p)")
                                .required(true)
                                .index(1)
                            )
                        )
                        .subcommand(SubCommand::with_name("template")
                            .about("Remove a template.")
                            .arg(Arg::with_name("index")
                                .short("i")
                                .long("index")
                                .help("specify the index of the template you want to remove (view with viper list -t)")
                                .required(true)
                                .index(1)
                            )
                        )
                    )
                    .subcommand(SubCommand::with_name("list")
                        .about("Lists locally saved projects and templates.")
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Show verbose output.")
                        )
                        .arg(Arg::with_name("templates")
                            .short("t")
                            .long("templates")
                            .help("Filters list to show only templates.")
                        )
                        .arg(Arg::with_name("projects")
                            .short("p")
                            .long("projects")
                            .help("Filters list to show only projects.")
                        )
                    )
                    .subcommand(SubCommand::with_name("update")
                        .about("Updates template & project details in .record.json")
                        .arg(Arg::with_name("path")
                            .short("p")
                            .long("path")
                            .help("specify project path")
                        )
                    )
                    .get_matches();

    // Initializing the Progress Bar
    let _bar = ProgressBar::new(100);

    if let Some(matches) = matches.subcommand_matches("new") {
        // Parsing the project name
        let project_name = matches.value_of("name").unwrap();
        let path_name = format!("{}", project_name);
        println!("Creating Project... {:?}", project_name);

        // Creating Project Directory & main.py;
        let mut venv: bool = false;
        if matches.is_present("env") {
            venv = true;
        }

        viper_utils::fh::create_boilerplate_files(&path_name, venv);

        if venv {
            // Checking pip version
            viper_utils::cli::check_pip_version();

            // Creating requirements.txt
            let requirements_file = viper_utils::fh::create_requirements_file(&path_name);

            // Parsing Module Arguments
            let mut modules = Vec::new();

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
                let _imported_dependencies = fs::read_to_string(filename).expect("!Error: unable to read dependencies file.");
            }

            // Set Requirements
            viper_utils::fh::set_requirements(modules, &requirements_file);

        } else {
            if let Some(matches) = matches.subcommand_matches("new") {
                if matches.is_present("module")
                || matches.is_present("freeze")
                || matches.is_present("freeze3")
                || matches.is_present("importd") {
                    println!("\n!Cannot add dependencies: venv not specified")
                }
            }
        }
        // Install git via the cli
        viper_utils::cli::install_git(&path_name);

        // Add to .record.json
        println!("Saving project.");
        tabling::add_project(&path_name, &project_name, "");

    } else if let Some(matches) = matches.subcommand_matches("template") {
        // Parsing the template name
        let template_name = matches.value_of("name").unwrap();

        let mut template = json::object!{
            "name": template_name,
            "language": "python",
            "config": {
                modules: [],
            }
        };

        // Env Option
        let venv: bool = matches.is_present("env");
        if venv {
            template["config"]["env"] = json::JsonValue::Boolean(true);
        } else {
            template["config"]["env"] = json::JsonValue::Boolean(false);
        }

        if venv {
            // Parsing Module Arguments
            if matches.is_present("module") {
                template["config"]["modules"] = json::JsonValue::new_array();
                println!("\nExternal Modules Specified: ");
                for m in matches.values_of("module").unwrap() {
                        println!("{:?}", m);
                        template["config"]["modules"].push(m);
                }
            } else {
                template["config"]["modules"] = json::JsonValue::new_array();
            }

            // Freeze Option
            if matches.is_present("freeze") {
                template["config"]["freeze"] = json::JsonValue::Boolean(true);
                template["config"]["freeze3"] = json::JsonValue::Boolean(false);
            } else if matches.is_present("freeze3"){
                template["config"]["freeze3"] = json::JsonValue::Boolean(true);
                template["config"]["freeze"] = json::JsonValue::Boolean(false);
            } else {
                template["config"]["freeze3"] = json::JsonValue::Boolean(false);
                template["config"]["freeze"] = json::JsonValue::Boolean(false);
            }

            // Importd Option
            if matches.is_present("importd") {
                let filename = matches.value_of("importd").unwrap();
                template["config"]["importd"] = json::JsonValue::String(json::stringify(filename.to_string()));
            }
        }

        print!("{:#}", template.pretty(4));

        let pre_path: String;

        if let Some(v) = matches.value_of("location") {
            pre_path = v.to_string();
        } else {
            pre_path = "".to_string();
        }
        
        let file_path = format!("{}./{}.json", pre_path, "template");
        let path = path::Path::new(&file_path);
        let display = path.display();

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        file.write(template.dump().as_bytes()).expect("!Could not write to file.");

        //println!("{:?}", template);
        println!("{}", tabling::add_template(template, &file_path, template_name));

    } else if let Some(matches) = matches.subcommand_matches("build") {
        let mut template: json::JsonValue = json::object!{};

        match matches.value_of("path").unwrap().parse::<i32>() {
            Ok(index) => {
                match dirs::home_dir() {
                    Some(path) => {
                        let mut sep_string = "\\";
                        if cfg!(unix) {
                            sep_string = "/";
                        }

                        // Open & Read File
                        let contents = fs::read_to_string(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).expect("\n!Error: Could not read from file.");
                        let mut templates = json::parse(&contents).unwrap(); 
                        template = templates["templates"].array_remove(index as usize);
                        // template = templates["templates"][matches.value_of("path").unwrap()];
                    },
                    _ => println!("\n!Error: Could not find home folder"),
                }
            },
            Err(e) =>  {
                // Reading from the file
                let path = matches.value_of("path").unwrap();
                let mut file = fs::File::open(&path).unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();

                // parsing json
                template = json::parse(&data).unwrap();
            }
        }
        
        
        let name = matches.value_of("name").unwrap();

        // User indicator
        println!("Creating Project... {}", matches.value_of("name").unwrap());
        print!("\nTemplate File:\n{:#}", template.pretty(4));

        // Parsing options
        let mut venv: bool = false;
        if Some(true) == template["config"]["env"].as_bool() {
            venv = true;
        }

        viper_utils::fh::create_boilerplate_files(&name, venv);

        if venv {
            // Checking pip version
            viper_utils::cli::check_pip_version();

            // Creating requirements.txt
            let requirements_file = viper_utils::fh::create_requirements_file(&name);

            // Parsing Module 
            let mut modules = Vec::new();

            if template["config"]["modules"].len() > 0 {
                let json_modules = &template["config"]["modules"];
                println!("\n\nExternal Modules Specified: ");

                for i in 0..json_modules.len() {
                        println!("{} : {:?}", i, json_modules[i].as_str().unwrap());
                        modules.push(json_modules[i].as_str().unwrap());
                }
            }

            // Parsing freeze/freeze3 option
            if template["config"]["freeze"].as_bool().unwrap() {
                viper_utils::fh::freeze(2, &requirements_file);
            } else if template["config"]["freeze3"].as_bool().unwrap() {
                viper_utils::fh::freeze(3, &requirements_file);
            }

            // Parsing importd option
            if template["config"]["importd"].len() > 0 {
                // Parsing filename
                let filename = template["config"]["importd"].as_str().unwrap();
                // Opening and Reading from file
                let _imported_dependencies = fs::read_to_string(filename).expect("!Error: unable to read dependencies file.");
            }

            // Set Requirements
            viper_utils::fh::set_requirements(modules, &requirements_file);
        } else {
            /*
            let config = &template["config"];

            match config["modules"].len() > 0
            || config["freeze"].as_bool().unwrap()
            || config["freeze3"].as_bool().unwrap()
            || config["importd"].as_bool().unwrap() {
                true => println!("\n!Cannot add dependencies: venv not specified"),
                _ => (),
            }
            */
        }
        // Install git via the cli
        viper_utils::cli::install_git(&name);
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let mut verbose: bool = false;
        if matches.is_present("verbose") {
            verbose = true;
        }

        if matches.is_present("templates") {
            tabling::list_templates(verbose);
        } else if matches.is_present("projects") {
            tabling::list_projects(verbose);
        } else {
            tabling::list_templates(verbose);
            tabling::list_projects(verbose);
        }
        
    } else if let Some(matches) = matches.subcommand_matches("remove") {

        if let Some(matches) = matches.subcommand_matches("template") {
            match matches.value_of("index").unwrap().parse::<i32>() {
                Ok(index) => {
                    println!("Removing template at index {}", index);
                    control::tabling::remove_template(index);
                },
                _ => println!("Invalid Index")
            }
        } else if let Some(matches) = matches.subcommand_matches("project") {
            match matches.value_of("index").unwrap().parse::<i32>() {
                Ok(index) => {
                    println!("Removing project at index {}", index);
                    control::tabling::remove_project(index);
                },
                _ => println!("Invalid Index")
            }
        }
        
    } else if let Some(matches) = matches.subcommand_matches("update") {
        let mut sep_string = "\\";
        if cfg!(unix) {
            sep_string = "/";
        }

        if path::Path::new(&format!("{}{}", matches.value_of("path").unwrap(), sep_string)).exists() {
            control::tabling::update(&path::Path::new(&format!("{}{}", matches.value_of("path").unwrap(), sep_string)));
        }
    }
}
