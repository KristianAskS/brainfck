use std::fs;
use std::path::PathBuf;
use std::process::Command;

use clap::App;

mod builder;
mod parser;

fn format_input(input: String) -> String {
    let x = input.replace(" ", "").replace("\n", "");
    x
}

fn main() {
    let app = App::new("Brainfuck transpiler").arg(
        clap::Arg::new("input")
            .help("The brainfuck code to transpile")
            .required(true),
    );

    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();

    let input: String = {
        let srcdir = PathBuf::from(input);
        if srcdir.is_file() {
            let contents = fs::read_to_string(srcdir)
                .unwrap();
            contents
        } else {
            input.to_string()
        }
    };

    let input = format_input(input.to_string());
    let parser = parser::Parser::new(input);
    parser.parse();
    let instructions = parser.get_instructions();
    let builder = builder::Builder::new(instructions);
    builder.builder("transpiled.c");

    let mut compile_command = Command::new("gcc")
        .arg("-O1")
        .arg("transpiled.c")
        .arg("-o")
        .arg("transpiled")
        .spawn()
        .expect("failed to execute process");
    compile_command.wait().expect("failed to wait on child");

    let mut run_command = Command::new("./transpiled")
        .spawn()
        .expect("failed to execute process");
    run_command.wait().expect("failed to wait on child");

    fs::remove_file("transpiled.c").expect("failed to remove file");
    fs::remove_file("transpiled").expect("failed to remove file");
}
