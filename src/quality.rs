use ansi_term::Colour::Green;
use std::io::{self, Write};
use std::process::Command;

pub fn search(path: &str) {
    println!("{}", Green.bold().paint("# Number of SLOC:"));
    search_nb_sloc(path);
    println!("\n")
}

fn search_nb_sloc(path: &str) {
    let output = Command::new("tokei")
        //.current_dir(path)
        .args([path])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
