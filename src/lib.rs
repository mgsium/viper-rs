#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate dialoguer;
extern crate requests;

// use regex::Regex;

// ============================================================================================
// File Handling Sub-Module
// ============================================================================================
pub mod fh {
    // Crate Directives
    // ----------------------------------------------------------------------------------------
    use std::fs;
    use std::io::{Read, Write};
    use std::error::Error;
    use std::path;
    use regex::Regex;

    use std::process::Command;
    // ----------------------------------------------------------------------------------------

    // Public Functions
    // ----------------------------------------------------------------------------------------
    pub fn create_boilerplate_files(path_name: &str, venv: bool) {
        if venv {
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
        } else {
            fs::create_dir_all(path_name).expect(&format!("!Error: Could not create project directory at {}", path_name));
        }

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

        file.write(b"
def main():
    print('Hello, World!')

if __name__ == \"__main__\":
    main()
        ").expect("!Error: Unable for write to file main.py");
    }

    pub fn create_requirements_file(path_name: &str) -> fs::File {
        // Creating the main.py file, error checking
        let file_path = format!("{}/requirements.txt", path_name);
        let path = path::Path::new(&file_path);
        let display = path.display();

        let file = match fs::File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        return file;
    }

    pub fn set_requirements(modules: Vec<&str>, mut file: &fs::File) -> bool {
        println!("\nAdding modules... ");
        println!("{:?}", modules);

        if modules.len() > 0 {
            install_yolk3k();
        }

        // Checking module format
        for m in modules.iter() {
            let (is_valid, version_num) = check_module_format(m);
            if is_valid {
                println!("{} :  OK", m);
                file.write(format!("{}\n", str::replace(&version_num, " ", "==")).as_bytes()).expect("!Error: Could not write module to file.");
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

    /*
    pub fn import_dependencies_from_file(filename: &str) -> Vec<String> {


        return modules;
    }
    */
    // ----------------------------------------------------------------------------------------

    // Private Functions
    // ----------------------------------------------------------------------------------------
    fn check_module_format(m: &str) -> (bool, String) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\b(?:[a-z]{1,})(?:@\d|[^@.])(?:(?:.\d+){0,2}|\w)").unwrap();
        }

        let mut has_version_num: bool = true;
        let mut ver;

        let output = Command::new("yolk")
            .args(&["-V", &m])
            .output()
            .expect("\n!Error: Could not run command.");

        if !m.contains("@") {
            ver = String::from_utf8_lossy(&output.stdout);
            if ver.len() > 0 {
                println!("{:?}", ver);
            } else {
                // println!("Invalid Package")
                has_version_num = false;
            }
        } else {
            ver = String::from_utf8_lossy(b"");
        }

        return (RE.is_match(m) && has_version_num, ver.to_string());
    }

    /*
    fn get_latest_module_version(m: &str) {
        let url = format!("https://pypi.python.org/pypi/{}/json", m);

        let response = requests::get(&url).unwrap();
        let data = response.json().unwrap();

        println!("{:?}", data);
    }
    */

    fn install_yolk3k() {
        println!("\n...Installing yolk3k");

        let status = Command::new("pip").args(&["install", "yolk3k"]).status().expect("\nError: Failed to Execute pip command.\n");
            if status.success(){
                println!("Successful\n");
            } else {
                println!("Error: could not install yolk3k.");
            }
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

    pub fn install_git(project_path: &str) {
        println!("\nInitialize with git?");

        let options = &[
            "Yes",
            "No"
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Pick an Option:")
                        .default(0)
                        .items(&options[..])
                        .interact()
                        .unwrap();

        let mut choice: bool = false;
        match selection {
            0 => choice = true,
            _ => choice = false
        }

        if choice {
            println!("\n...Cheking pip version.");
            let r = check_git_installed();
            // println!("{:?}", r);
            if r {
                println!("...Initializing");
                git_init(project_path);
                println!("...Done!");
            } else {
                println!("\n...Git is not installed.\n");
            }
        }
    }
    // ----------------------------------------------------------------------------------------

    // ----------------------------------------------------------------------------------------
    // Private Functions
    // ----------------------------------------------------------------------------------------
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

    fn check_git_installed() -> bool {
        Command::new("git").arg("--version").output().expect("").status.success()
    }

    fn git_init(path: &str) {
        println!("\n...Initializing Directory");

        Command::new("git")
            .args(&["init", &path])
            .output()
            .expect("\nError: Could not Initialize with Git.\n");
    }
    // ----------------------------------------------------------------------------------------
}
// ============================================================================================
