# Viper

A Rust command-line tool to simplify the creation and setup of python projects.

*Viper* allows you to create a new python virtual environment and add initial dependencies with a single command. Dependencies can be imported from a text file, specified inline, or ignored entirely if you forego the virtual environment.

## Main Features
- *Easy Project Creation* - the `viper new` subcommand allows for the creation of a project folder, venv and requirements file in one simple command.
- *Templating* - Config options can be handled in terms of templates containing config details in json format. By default templates are stored in the current directory, but an alternative location can be specified. 
<!--- - *Tabling* - viper indexes each python project you create, allowing to to create, delete, move and copy them with ease. View projects with "viper list"
-->

## Installation
```
cargo install viper
```

## Examples
``` 
// Create a project directory, initialize a virtual environment and add dependencies
viper new "./TestProject" -e -F -m="matplotlib"
```

```
// Create a templete
viper template "./TestProjectTemplate" -e -f -m="matplotlib"
// Build template (equivalent to the first command)
viper build "TestProjectTemplate.json" "TestProject"
```

## Usage
```
viper 0.1
Musab G. <musabgumaa@gmail.com>

USAGE:
    viper.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build       
    help        Prints this message or the help of the given subcommand(s)
    new         Creates a new project.
    template    Creates a project template.

```

## Versions
**0.1.24** Ability to build from template (viper build subcommand)

**0.1.23** Template Creation (viper template subcommand)

**0.1.22**  Error handling improved

**0.1.2**  Dependency Management, venv now optional.

**0.1.0**  Initial commit

## License
Viper is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
