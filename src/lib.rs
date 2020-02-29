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

    use std::process::Command;
    // ----------------------------------------------------------------------------------------

    // Public Functions
    // ----------------------------------------------------------------------------------------
    pub fn create_boilerplate_files(path_name: &str) {
        // Installing virtualenv
        Command::new("pip")
                .args(&["install", "virtualenv"])
                .output()
                .expect("Could not install virtualenv.");
        
        println!("\n. . .Building Project Directory.");

        // Creating the Project Directory
        Command::new("virtualenv")
                .arg(&path_name)
                .output()
                .expect("Could not create virtual environment.");
            
        println!("\n. . .Done!");

        // fs::create_dir_all(&path_name).expect("!Error: Could not create Project Directory.");
        
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

    pub fn create_requirements_file(path_name: &str) -> fs::File {
        // Creating the main.py file, error checking
        let file_path = format!("{}/requirements.txt", path_name);
        let path = path::Path::new(&file_path);
        let display = path.display();

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        return file;
    }

    pub fn set_requirements(modules: Vec<&str>, mut file: &fs::File) -> bool {
        println!("\nAdding modules... ");

        // Checking module format
        for m in modules.iter() {
            if check_module_format(m) {
                println!("{} :  OK", m);
                let output = str::replace(m, "@", "==");
                file.write(format!("{}\n", output).as_bytes()).expect("!Error: Could not write module to file.");
            } else {
                println!("{} : Issue Encountered", m);
                return false;
            }
        }

        return true;
    }

    pub fn freeze(pip_v: u32, mut file: &fs::File) {
        let mut command = String::new();

        if pip_v == 3 {
            println!("\n. . .freezing installed modules with pip3");
            command = String::from("pip3");
        } else if pip_v == 2 {
            println!("\n. . .freezing installed modules with pip");
            command = String::from("pip");
        } else {
            println!("Error");
        }
        
        // println!("{}", format!("{}/requirements.txt", project_path));

        let output = Command::new(&command)
                .args(&["freeze"])
                .output()
                .expect("\n!Error: Could not create requirements.txt");

        file.write(&output.stdout).expect("\n!Error: Could not write to requirements.txt");
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
    use std::io;
    use std::io::*;
    use std::process::Command;
    use dialoguer::{theme::ColorfulTheme, Select};
    use regex::Regex;
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

            match selection {
                1 => update_pip_version(),
                2 => install_custom_pip_version(),
                _ => println!("\n. . .Using default pip version")

            }

            // println!("{}", selection);
        } else {
            // println!("{:?}", status);
            println!("Pip is not installed");
        }
    }

    fn update_pip_version() {
        println!("\n. . .Updating pip version");

        Command::new("python")
                .args(&["-m","pip","install","--upgrade","pip"])
                .output()
                .expect("Could not update pip.");

        Command::new("pip").arg("-V").status().expect("\nError: Failed to Execute pip command.\n");
        println!("\n. . .Latest pip version installed");
    }

    fn install_custom_pip_version() {
        let mut valid = false;
        let mut version = String::new();
        let ref RE: Regex = Regex::new(r"\b(?:(?:.\d+){0,2}|\w)").unwrap();

        while !valid {
            print!("Enter preferred version number: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut version).expect("\nError: unable to read input.");
            if RE.is_match(&version) {
                valid = true;
            }
        }

        println!("\n. . .Installing pip version {}", version);

        Command::new("pip")
                .args(&["uninstall", "pip"])
                .output()
                .expect("\nCould not uninstall pip");

        Command::new("python")
                .args(&["-m", "pip", "install", "--upgrade", &format!("pip=={}", version)])
                .output()
                .expect(&format!("\nCould not install pip version {}", version));
            
        Command::new("pip").arg("-V").status().expect("\nError: Failed to Execute pip command.\n");
    }
    // ----------------------------------------------------------------------------------------
}
// ============================================================================================