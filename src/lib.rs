#[macro_use] extern crate lazy_static;
extern crate regex;

// use regex::Regex;

// ============================================================================================
// File Handling Sub-Module
// ============================================================================================
pub mod fh {
    // Crate Directives
    // ----------------------------------------------------------------------------------------
    use std::fs;
    use std::error::Error;
    use std::path;
    use regex::Regex;
    // ----------------------------------------------------------------------------------------

    // Public Functions
    // ----------------------------------------------------------------------------------------
    pub fn create_file(file_type:&str) {

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
            static ref RE: Regex = Regex::new(r"([a-z]{1,})(@{0,})([\d]{0,})(.?)([\d]{0,})(.?)([\d]{0,})").unwrap();
        }

        return RE.is_match(m);
    }
    // ----------------------------------------------------------------------------------------
}
// ============================================================================================

