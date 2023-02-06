use ansi_term::Colour::Green;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::Command;

pub fn search(path: &OsStr) {
    println!("{}", Green.bold().paint("# Number of SLOC:"));
    search_nb_sloc(path);
    println!("\n");
    println!("{}", Green.bold().paint("# Dependency graph:"));
    search_dependency_graph(path);
    println!("\n");
    println!("{}", Green.bold().paint("# Code coverage:"));
    search_code_coverage(path);
    println!("\n");
}

pub fn fast_search(path: &OsStr) {
    println!("{}", Green.bold().paint("# Number of SLOC:"));
    search_nb_sloc(path);
    println!("\n");
    println!("{}", Green.bold().paint("# Dependency graph:"));
    search_dependency_graph(path);
    println!("\n");
}

fn search_nb_sloc(path: &OsStr) {
    let output = Command::new("tokei")
        //.current_dir(path)
        .args([path])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn search_dependency_graph(path: &OsStr) {
    let output = Command::new("cargo")
        .current_dir(path)
        .args(["tree"])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn search_code_coverage(path: &OsStr) {
    let output = Command::new("cargo")
        .current_dir(path)
        .args(["tarpaulin"])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
