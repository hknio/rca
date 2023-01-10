use thiserror::Error;
use which::which;

pub const RUSTUP_COMPONENT_LIST: &[&str] = &["cargo-clippy", "rustfmt"];
pub const CARGO_SUBCOMMAND_LIST: &[&str] = &[
    "cargo-outdated",
    "cargo-audit",
    "cargo-tarpaulin",
    "cargo-geiger",
    "cargo-crev",
    "cargo-install-update",
    "cargo-expand",
    "cargo-tree",
    "cargo-modules",
    "cargo-nextest",
];
pub const SYSTEM_BINARY_LIST: &[&str] = &["az", "git"];

#[derive(Debug, Error)]
pub enum DepError {
    #[error("Update error")]
    UpdateFailed,
    #[error("Rustup component {:?} installation failed", .0)]
    ComponentsInstallFailed(Vec<String>),
    #[error("Cargo subcommand {:?} installation failed", .0)]
    SubcommandsInstallFailed(Vec<String>),
    #[error("Binary {:?} is not installed or not in PATH", .0)]
    SystemBinariesNotInstalled(Vec<String>),
}

pub fn update_and_install_dependencies() -> Result<(), Vec<DepError>> {
    let mut dep_errors = Vec::new();

    if let Err(e) = update() {
        dep_errors.push(e);
    }

    if let Err(e) = install_rustup_components() {
        dep_errors.push(e);
    }

    if let Err(e) = install_cargo_subcommands() {
        dep_errors.push(e);
    }

    if let Err(e) = check_system_binaries() {
        dep_errors.push(e);
    }

    if dep_errors.is_empty() {
        Ok(())
    } else {
        Err(dep_errors)
    }
}

fn update() -> Result<(), DepError> {
    // TODO: rustup self update
    // TODO: rustup update
    // TODO: cargo bin update
    Ok(())
}

// TODO: should check toolchain version

fn install_rustup_components() -> Result<(), DepError> {
    let mut install_failed = Vec::new();

    for component in RUSTUP_COMPONENT_LIST {
        if !is_installed(component) {
            install_failed.push(component.to_string())
            // TODO: Install component when cmd.rs done
        }
    }

    if install_failed.is_empty() {
        Ok(())
    } else {
        Err(DepError::ComponentsInstallFailed(install_failed))
    }
}
fn install_cargo_subcommands() -> Result<(), DepError> {
    let mut install_failed = Vec::new();

    for subcommand in CARGO_SUBCOMMAND_LIST {
        if !is_installed(subcommand) {
            // TODO: Install subcommand when cmd.rs done
            install_failed.push(subcommand.to_string())
        }
    }

    if install_failed.is_empty() {
        Ok(())
    } else {
        Err(DepError::SubcommandsInstallFailed(install_failed))
    }
}

fn check_system_binaries() -> Result<(), DepError> {
    let mut uninstall = Vec::new();
    for binary in SYSTEM_BINARY_LIST {
        if !is_installed(binary) {
            uninstall.push(binary.to_string());
        }
    }

    if uninstall.is_empty() {
        Ok(())
    } else {
        Err(DepError::SystemBinariesNotInstalled(uninstall))
    }
}

fn is_installed(name: &str) -> bool {
    which(name).is_ok()
}
