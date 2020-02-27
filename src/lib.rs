#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate dialoguer;

// use regex::Regex;

// ============================================================================================
// File Handling Sub-Module
// ============================================================================================
pub mod fh {
    // Crate Directives
    // ----------------------------------------------------------------------------------------
    use std::fs;
    use std::io::Write;
    use std::error::Error;
    use std::path;
    use regex::Regex;
    // ----------------------------------------------------------------------------------------

    // Public Functions
    // ----------------------------------------------------------------------------------------
    pub fn create_boilerplate_files(path_name: &str) {
        // Creating the Project Directory
        fs::create_dir_all(&path_name).expect("!Error: Could not create Project Directory.");

        // Creating the main.py file, error checking
        let file_path = format!("{}/main.py", path_name);
        let path = path::Path::new(&file_path);
        let display = path.display();

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        file.write(b"print('Hello, World!')").expect("!Error: Unable for write to file main.py");
    }

    pub fn set_requirements(modules: Vec<&str>) -> bool {
        println!("\nCreating Requirements File... ");

        // Checking module format
        for m in modules.iter() {
            if check_module_format(m) {
                println!("{} :  OK", m);
            } else {
                println!("{} : Issue Encountered", m);
                return false;
            }
        }

        return true;
    }
    // ----------------------------------------------------------------------------------------

    // Private Functions
    // ----------------------------------------------------------------------------------------
    fn check_module_format(m: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\b(?:[a-z]{1,})(?:@\d|[^@.])(?:(?:.\d+){0,2}|\w)").unwrap();
        }

        /*
        for capture in RE.captures(m) {
            println!("{:?}", capture);
        }
        */

        return RE.is_match(m);
    }
    // ----------------------------------------------------------------------------------------
}
// ============================================================================================

// ============================================================================================
// Command Line Tools Sub-Module
// ============================================================================================
pub mod cli {
    // Crate Directives
    // ----------------------------------------------------------------------------------------
    use std::process::Command;
    use dialoguer::{theme::ColorfulTheme, Select};
    // ----------------------------------------------------------------------------------------

    // ----------------------------------------------------------------------------------------
    // Public Functions
    // ----------------------------------------------------------------------------------------
    pub fn check_pip_version() {
        println!("\n...Checking pip version\n");
        let status = Command::new("pip").arg("-V").status().expect("\nError: Failed to Execute pip command.\n");

        if status.success() {
            // println!("\n{:?}\n", status);
            println!("Successful\n");

            let options = &[
                "Continue as default",
                "Update to latest pip version",
                "Install custom pip version"
            ];

            let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Pick an Option:")
                            .default(0)
                            .items(&options[..])
                            .interact()
                            .unwrap();

            // println!("{}", selection);
        } else {
            // println!("{:?}", status);
            println!("Pip is not installed");
        }
    }

    // ----------------------------------------------------------------------------------------
}
// ============================================================================================