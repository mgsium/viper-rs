#![allow(non_snake_case)]

extern crate dirs;
extern crate json;

pub mod tabling {

    use std::path::Path;
    use std::fs;


    pub fn add_template(template: json::JsonValue, template_path: &str, template_name: &str) -> String {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                if !Path::new(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).exists() {
                    println!("\n\nCreating .record.json file... ");
                    // println!("{}\\viper", path.to_str().unwrap());
                    // println!("{}\\.viper\\record.json", path.to_str().unwrap());
                    fs::create_dir_all(&format!("{}\\.viper", path.to_str().unwrap())).expect("!Error: Could not create .viper directory");
                    fs::File::create(&format!("{}\\.viper\\.record.json", path.to_str().unwrap())).expect("\n!Error: Could not create .record.json file.");
                }

                // Open & Read File
                let mut contents = fs::read_to_string(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).expect("\n!Error: Could not read from file.");
                
                let mut record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                } else {
                    record = json::object!{
                        "templates": []
                    };
                }

                // Add template to json
                record["templates"].push(template);

                // Save new Json to File
                fs::write(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string), record.dump().as_bytes()).expect("\n!Error: Could not write to file.");

                return format!("\nSaved to {}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string);
            },
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

                // Open & Read File
                let contents = fs::read_to_string(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).expect("\n!Error: Could not read from file.");
                
                let record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    
                    if record["templates"].len() > 0 {
                        for t in 0..record["templates"].len() {
                            println!("{}:    {}", t, record["templates"][t]["name"]);
                            if verbose {
                                print!("{:#}\n\n", record["templates"][t].pretty(4));
                            }
                        }
                    } else {
                        println!("(No Templates saved.)");
                    }

                } else {
                    println!("\nNo templates saved.")
                }
            },
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
                let contents = fs::read_to_string(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).expect("\n!Error: Could not read from .record.json");
                
                let mut record: json::JsonValue;
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    let t = record["templates"].array_remove(index as usize);
                    print!("\nRemoved template {}...\n{:#}\n", index, t);
                    fs::write(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string), record.dump()).expect("\n!Error: Could not write to .record.json");
                }

            },
            _ => println!("!Error: Could not find home folder.")
        }
    }

}