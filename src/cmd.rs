use std::io;
use std::path::PathBuf;
use std::process::{self, Command};

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Rustup,
    Cargo,
    System,
}

pub trait Cmd {
    fn bin(&self) -> &str;
    //fn target(&self) -> &Path;
    fn cmd_type(&self) -> Type;
    fn cmd_line(&self) -> &str;
    //fn install(&self) -> bool;
    //fn output(&self) -> String;

    fn execute(&self) -> io::Result<String> {
        match self.cmd_type() {
            Type::Rustup | Type::Cargo => {}
            Type::System => {}
            _ => {
                eprintln!("Unknown command type");
                process::exit(-1);
            }
        }
    }
}

// TODO: struct or trait need to choose
pub trait RustupCmd: Cmd {}

// TODO: struct or trait need to choose
//pub trait CargoCmd: Cmd {
//}

pub struct CargoCmd {
    name: String,
    args: Vec<String>,
}

impl CargoCmd {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }
}

impl Cmd for CargoCmd {
    fn bin(&self) -> &str {
        &name
    }

    fn cmd(&self) -> &str {
        &format!("cargo {} {:?}", self.name, self.args)
    }

    fn execute(&self) -> io::Result<String> {
        let output = Command::new("cargo")
            .args(&self.args)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        let s = String::from_utf8_lossy(&output.stdout);
        Ok(s.to_string())
    }
}

// TODO: struct or trait need to choose
pub struct BinCmd {}
