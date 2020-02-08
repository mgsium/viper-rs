extern crate clap;

use clap::{Arg, App};
//use std::env;

fn main() {

    let matches = App::new("viper")
                    .version("0.1")
                    .author("Musab G. <musabgumaa@gmail.com>")
                    .arg(Arg::with_name("venv")
                        .short("v")
                        .long("venv")
                        .help("Creates a venv for the project."))
                    .arg(Arg::with_name("dependencies")
                        .short("D")
                        .long("dependencies")
                        .help("Specify Dependencies for the project.")
                        .multiple(true)
                        .takes_value(true))
                    .get_matches();

    /*
    let output = if let values = matches.values_of("dependencies") {
        for value in values.unwrap() {
            println!("{:?}", value);
        }
    };
    */

    let _dependencies: Vec<&str> = matches.values_of("dependencies").unwrap().collect();
    
}
