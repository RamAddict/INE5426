
# SAPO's A Parser Option
## Compilador CC-2021-1

## File structure:
![image](https://user-images.githubusercontent.com/32513434/125174087-42174800-e199-11eb-997b-4f174e8a6ca7.png)

* the `docs/` folder contains examples that we used during development.
* the `examples/` folder cotains the required 100 lines examples in x++ language
* the `relatorio/` folder contains the required report
* the `src/` folder contains the code and grammar underpinning this work

## To build the project you should run install (if you don't have rust), then make build

## If you don't have rust, run:
    make install

## Building:
    make build

### USAGE:
    make run ARGS="<SUBCOMMAND>"
### This example runs lexer on examples/T1/T1E1.lcc and shows symbols table:
    make run ARGS="lex examples/T1/T1E1.lcc -s"

### This example runs syntax on examples/T2/T2E3.ccc:
    make run ARGS="syntax ./examples/T2/T2E3.ccc"

### This example runs semantic on examples/T2/T3E4.ccc:
    make run ARGS="semantic ./examples/T2/T3E4.ccc"

### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

### SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    lex     Do a lexical analysis using the CC20211 lang
    syntax  Do a syntax analysis using the CC20211 lang
    semantic  Do a semantic analysis using the CC20211 lang

## While using the `lex` subcommand you may also use:
### FLAGS:
    -h, --help       Prints help information
    -s, --symbols    Display the symbols table
    -t, --tokens     Display the tokens list
    -V, --version    Prints version information

### OPTIONS:
    -i, --info <show-info>    Prints parsing information [default: true]


## While using the `syntax` subcommand you may also use:
### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -i, --info <show-info>    Prints parsing information [default: true]

## While using the `semantic` subcommand you may also use:
### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -i, --info <show-info>    Prints parsing information [default: true]
