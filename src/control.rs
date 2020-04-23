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

                return format!("\nSaved to {}", path.to_str().unwrap());
            },
            _ => return String::from("\n!Error: Could not find home folder"),
        }
    }

    pub fn list_templates() {
        match dirs::home_dir() {
            Some(path) => {
                let mut sep_string = "\\";
                if cfg!(unix) {
                    sep_string = "/";
                }

                println!("\nListing Templates...\n");

                // Open & Read File
                let mut contents = fs::read_to_string(&format!("{}{}.viper{}.record.json", path.to_str().unwrap(), sep_string, sep_string)).expect("\n!Error: Could not read from file.");
                
                let mut record: json::JsonValue;

                // Parse Json
                if json::parse(&contents).is_ok() {
                    record = json::parse(&contents).unwrap();
                    for t in 0..record["templates"].len() {
                        println!("{}", record["templates"][t]["name"]);
                    }
                } else {
                    println!("\nNo templates saved.")
                }
            },
            _ => println!("\n!Error: Could not find home folder"),
        }
    }

}