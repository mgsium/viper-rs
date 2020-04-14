# Viper

A Rust command-line tool to simplify the creation and setup of python projects.

*Viper* allows you to create a new python virtual environment and add initial dependencies with a single command. Dependencies can be imported from a text file, specified inline, or ignored entirely if you forego the virtual environment.

## Main Features
- *Easy Project Creation* - the `viper new` subcommand allows for the creation of a project folder, venv and requirements file in one simple command.
- *Templating* - Config options can be handled in terms of templates containing config details in json format, which can be re-used in the future. By default templates are stored in...
- *Tabling* - viper indexes each python project you create, allowing to to create, delete, move and copy them with ease. View projects with "viper list"

## Installation
```
cargo install viper
```
## Usage *viper*
```
viper 0.1
Musab G. <musabgumaa@gmail.com>

USAGE:
    viper.exe [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    new     Creates a new project.
```
### Usage *viper new*
```
viper.exe-new 
Creates a new project.

USAGE:
    viper.exe new [FLAGS] [OPTIONS] <name>

FLAGS:
    -e, --env        Creates a venv for the project.
    -f, --freeze     Specify installed modules (provided by 'pip freeze') as requirements
    -F, --freeze3    Specify installed modules (provided by 'pip3 freeze') as requirements
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --importd <importd>     Import dependencies from a file.
    -m, --module <module>...    Specify an external package to include in the project in the format
                                <modulename>@<version>.

ARGS:
    <name>    Specify the name for the project
```

## Versions
**0.1.22**  Error handling improved.

**0.1.2**  Dependency Management, venv now optional.

**0.1.0**  Initial commit

## License
Viper is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
