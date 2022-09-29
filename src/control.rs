#![allow(non_snake_case)]

extern crate dirs;
extern crate json;

pub mod tabling {

    use prettytable::{Cell, Row, Table};
    use std::path::Path;
    use std::{fs, io::*};

    pub fn add_template(
        template: json::JsonValue,
        template_path: &str,
        template_name: &str,
    ) -> String {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                if !Path::new(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .exists()
                {
                    println!("\n\nCreating .record.json file... ");
                    // println!("{}\\viper", path.to_str().unwrap());
                    // println!("{}\\.viper\\record.json", path.to_str().unwrap());
                    fs::create_dir_all(&format!("{}\\.viper", path.to_str().unwrap()))
                        .expect("!Error: Could not create .viper directory");
                    fs::File::create(&format!("{}\\.viper\\.record.json", path.to_str().unwrap()))
                        .expect("\n!Error: Could not create .record.json file.");
                }

                // Open & Read File
                let contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from file.");

                let mut record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                } else {
                    record = json::object! {
                        "templates": [],
                        "projects": []
                    };
                }

                // Add template to json
                record["templates"].push(template);

                // Save new Json to File
                fs::write(
                    &format!(
                        "{}{}.viper{}.record.json",
                        path.to_str().unwrap(),
                        sep_string,
                        sep_string
                    ),
                    record.dump().as_bytes(),
                )
                .expect("\n!Error: Could not write to file.");

                return format!(
                    "\nSaved to {}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                );
            }
            _ => return String::from("\n!Error: Could not find home folder"),
        }
    }

    pub fn add_project(project_path: &str, project_name: &str, template_name: &str) -> String {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                if !Path::new(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .exists()
                {
                    println!("\n\nCreating .record.json file... ");
                    // println!("{}\\viper", path.to_str().unwrap());
                    // println!("{}\\.viper\\record.json", path.to_str().unwrap());
                    fs::create_dir_all(&format!("{}\\.viper", path.to_str().unwrap()))
                        .expect("!Error: Could not create .viper directory");
                    fs::File::create(&format!("{}\\.viper\\.record.json", path.to_str().unwrap()))
                        .expect("\n!Error: Could not create .record.json file.");
                }

                // Open & Read File
                let mut contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from file.");

                let mut record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                } else {
                    record = json::object! {
                        "templates": [],
                        "projects": []
                    };
                }

                let project = json::object! {
                    "path": json::JsonValue::String(String::from(project_path)),
                    "name": json::JsonValue::String(String::from(project_name)),
                    "template": json::JsonValue::String(String::from(template_name))
                };

                // print!("{:#}", project.pretty(4));

                // Add project to json
                record["projects"]
                    .push(project)
                    .expect("Could not save project.");

                // Save new Json to File
                fs::write(
                    &format!(
                        "{}{}.viper{}.record.json",
                        path.to_str().unwrap(),
                        sep_string,
                        sep_string
                    ),
                    record.dump().as_bytes(),
                )
                .expect("\n!Error: Could not write to file.");

                return format!(
                    "\nSaved to {}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                );
            }
            _ => return String::from("\n!Error: Could not find home folder"),
        }
    }

    pub fn list_templates(verbose: bool) {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                println!("\nListing Templates...");

                // Initialize table
                let mut table = Table::new();
                if verbose {
                    table.add_row(row!["#", "Name", "Template Object"]);
                } else {
                    table.add_row(row!["#", "Name"]);
                }

                // Open & Read File
                let contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from file.");

                let record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();

                    if record["templates"].len() > 0 {
                        for t in 0..record["templates"].len() {
                            if !verbose {
                                // println!("{}:    {}", t, record["templates"][t]["name"]);
                                table.add_row(row![t, record["templates"][t]["name"]]);
                            } else {
                                // print!("{:#}\n\n", record["templates"][t].pretty(4));
                                table.add_row(row![
                                    t,
                                    record["templates"][t]["name"],
                                    record["templates"][t].pretty(4)
                                ]);
                            }
                        }
                        table.printstd();
                    } else {
                        println!("(No Templates saved.)");
                    }
                } else {
                    println!("\nNo templates saved.")
                }
            }
            _ => println!("\n!Error: Could not find home folder"),
        }
    }

    pub fn list_projects(verbose: bool) {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                println!("\nListing Projects...");

                // Initialize table
                let mut table = Table::new();
                table.add_row(row!["#", "Name", "Directory", "Template"]);

                // Open & Read File
                let contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from file.");

                let record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    let path = std::env::current_dir().unwrap();

                    if record["projects"].len() > 0 {
                        for t in 0..record["projects"].len() {
                            if !verbose {
                                // println!("{}:    {}", t, record["templates"][t]["name"]);
                                table.add_row(row![
                                    t,
                                    record["projects"][t]["name"],
                                    path.to_str().unwrap()
                                ]);
                            } else {
                                // print!("{:#}\n\n", record["templates"][t].pretty(4));
                                table.add_row(row![
                                    t,
                                    record["projects"][t]["name"],
                                    path.to_str().unwrap(),
                                    record["projects"][t]["template"]
                                ]);
                            }
                        }
                        table.printstd();
                    } else {
                        println!("(No Projects saved.)");
                    }
                } else {
                    println!("\n(No Projects saved.)")
                }
            }
            _ => println!("\n!Error: Could not find home folder"),
        }
    }

    pub fn remove_template(index: i32) {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                // Open & Read File
                let contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from .record.json");

                let mut record: json::JsonValue;
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    let t = record["templates"].array_remove(index as usize);
                    print!("\nRemoved template {}...\n{:#}\n", index, t);
                    fs::write(
                        &format!(
                            "{}{}.viper{}.record.json",
                            path.to_str().unwrap(),
                            sep_string,
                            sep_string
                        ),
                        record.dump(),
                    )
                    .expect("\n!Error: Could not write to .record.json");
                }
            }
            _ => println!("!Error: Could not find home folder."),
        }
    }

    pub fn remove_project(index: i32) {
        // Open and read File
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                // Open & Read File
                let contents = fs::read_to_string(&format!(
                    "{}{}.viper{}.record.json",
                    path.to_str().unwrap(),
                    sep_string,
                    sep_string
                ))
                .expect("\n!Error: Could not read from .record.json");

                let mut record: json::JsonValue;
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    let t = record["projects"].array_remove(index as usize);
                    print!("\nRemoved project {}...\n{:#}\n", index, t);
                    fs::write(
                        &format!(
                            "{}{}.viper{}.record.json",
                            path.to_str().unwrap(),
                            sep_string,
                            sep_string
                        ),
                        record.dump(),
                    )
                    .expect("\n!Error: Could not write to .record.json");
                }

                let mut choice = String::from("");

                while choice.trim() != "y"
                    && choice.trim() != "Y"
                    && choice.trim() != "n"
                    && choice.trim() != "N"
                {
                    print!("Would you like to delete the project folder? (y/N) : ");
                    let _ = std::io::stdout().flush();
                    std::io::stdin()
                        .read_line(&mut choice)
                        .expect("\nError: unable to read input.");
                    println!("{}", choice);
                }
            }
            _ => println!("!Error: Could not find home folder."),
        }
    }

    pub fn update(path: &Path) {
        //
    }
}
