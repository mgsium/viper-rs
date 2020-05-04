# viper-rs
[![Build status](https://ci.appveyor.com/api/projects/status/l8a7wvywmu4nsxac?svg=true)](https://ci.appveyor.com/project/mgsium/viper)
![Rust](https://github.com/mgsium/viper-rs/workflows/Rust/badge.svg)

A Rust command-line tool to simplify the creation and setup of python projects.

*Viper* allows you to create a new python virtual environment and add initial dependencies with a single command. Dependencies can be imported from a text file, specified inline, or ignored entirely if you forego the virtual environment.

## Main Features
- *Easy Project Creation* - the `viper new` subcommand allows for the creation of a project folder, venv and requirements file in one simple command.
- *Templating* - Config options can be handled in terms of templates containing config details in json format. Templates can be added, deleted, and used to build a project in a standard format. By default templates are stored in the current directory, but an alternative location can be specified. 
- *Tabling* - viper indexes each python project you create, allowing to to create, delete, move and copy them with ease.

## Installation
```
cargo install viper
```

## Examples

- Create a basic new project
``` 
viper new "./TestProject""
```

- Create a template, specifying a venv (-e), freeze modules (-f) and add matplotlib to requirements.txt (-m="matplotlib")
```
viper template "./TestProjectTemplate" -e -f -m="matplotlib"
```

- Build template (equivalent to the first command)
```
viper build "TestProjectTemplate.json" "TestProject"
```

- List Created templates
```
viper list
```

- Delete Template at index 0 (find index using viper list)
``` 
viper remove 0
```

## Usage
```
viper 0.3.0
Musab G. <musabgumaa@gmail.com>

USAGE:
    viper.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build       Builds a project from a json template.
    help        Prints this message or the help of the given subcommand(s)
    list        Lists locally saved projects and templates.
    new         Creates a new project.
    remove      Remove a template/project.
    template    Creates a project template.
    update      Updates template & project details in .record.json
```

## Versions
**0.3.0** Tabling Features for Projects Added

**0.2.1** Minor Changes

**0.2.0** Template creation improved; deletion and build from template added.

**0.1.24** Ability to build from template (viper build subcommand)

**0.1.23** Template Creation (viper template subcommand)

**0.1.22**  Error handling improved

**0.1.2**  Dependency Management, venv now optional.

**0.1.0**  Initial commit

## License
Viper is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
