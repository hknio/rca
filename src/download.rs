use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn git_download(git_url: &str) -> PathBuf {
    let output = Command::new("git")
        .args(["clone", git_url])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    let tmp = git_url
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .take(1)
        .collect::<String>();
    let mut path = env::current_dir().unwrap();
    path.push(tmp);
    path
}
