use ansi_term::Colour::Green;
use std::io::{self, Write};
use std::process::Command;

pub enum IssueKind {
    Formatting,
    CompilationWarning,
    CompilationError,
    OutdatedDependency,
    VulnerableDependency,
    IntegerArithmetic,
    UnboundedDataStructure, // free writing in memory
}

pub fn search(path: &str) {
    //_ = env::set_current_dir(path);
    println!(
        "{}",
        Green.bold().paint("# Compilation errors and warnings")
    );
    find_compilation_errors(path);
    println!("\n");

    println!("{}", Green.bold().paint("# Formatting issues"));
    find_formatting_issues(path);
    println!("\n");

    println!("{}", Green.bold().paint("# Outdated dependencies"));
    find_outdated_dependencies(path);
    println!("\n");

    println!("{}", Green.bold().paint("# Vulnerable dependencies"));
    find_vulnerable_dependencies(path);
    println!("\n");

    println!("{}", Green.bold().paint("# Integer arithmetics"));
    find_integer_arithmetics(path);
    println!("\n");
    /*
    println!("# Result/Option handlings and crashes");
    println!("# Unbounded datastructures");
    find_unbounded_datastructures(path);
    */
}

fn find_compilation_errors(path: &str) {
    let output = Command::new("cargo")
        .current_dir(path)
        .args(["check"])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn find_formatting_issues(path: &str) {
    let output = Command::new("cargo")
        .current_dir(path)
        .arg("fmt")
        .arg("--check")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn find_outdated_dependencies(path: &str) {
    let output = Command::new("cargo")
        .current_dir(path)
        .arg("outdated")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn find_vulnerable_dependencies(path: &str) {
    let output = Command::new("cargo")
        .current_dir(path)
        .arg("audit")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn find_integer_arithmetics(path: &str) {
    let output = Command::new("cargo")
        .current_dir(path)
        .args([
            "clippy",
            "--",
            "-A",
            "clippy::all",
            "-D",
            "clippy::integer_arithmetic",
        ])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

// fn find_unbounded_datastructures(path: &str) {}
