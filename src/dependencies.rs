use crate::command::execute_command_no_path;
use ansi_term::Colour::{Green, Red, Yellow};

/// List of Rustup components to install.
pub const RUSTUP_COMPONENT_LIST: &[&str] = &["cargo-clippy", "rustfmt"];

/// List of system binaries to check.
pub const SYSTEM_BINARY_LIST: &[&str] = &["git"];

/// List of Cargo subcommands to install.
pub const CARGO_SUBCOMMAND_LIST: &[&str] = &[
    "cargo-outdated",
    "cargo-audit",
    "cargo-tarpaulin",
    "cargo-crev",
    "cargo-install-update",
    "cargo-expand",
    "cargo-modules",
    // "cargo-nextest",
    "tokei",
];

/// Represents the kinds of dependencies.
#[derive(Debug, Clone, Copy)]
pub enum Kind {
    /// Rustup component.
    Rustup,
    /// Cargo subcommand.
    Cargo,
    /// System binary.
    System,
}

/// Represents errors related to dependency management.
#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    /// Error updating Rustup and Cargo.
    #[error("Update Error")]
    UpdateFailed,

    /// Error installing Rustup components.
    #[error("Rustup Component {:?} Installation Failed", .0)]
    ComponentInstallFailed(Vec<String>),

    /// Error installing Cargo subcommands.
    #[error("Cargo Subcommand {:?} Installation Failed", .0)]
    SubcommandsInstallFailed(Vec<String>),

    /// System binary not installed or not in PATH.
    #[error("System Binary {:?} Are Not Installed or Not In $PATH", .0)]
    SystemBinariesNotInstalled(Vec<String>),
}

/// Installs and updates Rust toolchain dependencies.
///
/// This function updates Rustup and Cargo, installs Rustup components, Cargo subcommands,
/// and checks for the presence of required system binaries.
///
/// # Returns
///
/// - `Ok(())` if no errors are encountered.
/// - `Err(Vec<DependencyError>)` if any errors are encountered during dependency installation or checks.
///
/// # Example
///
/// ```
/// use rca::dependencies::{update_and_install_dependencies, DependencyError};
///
/// match update_and_install_dependencies() {
///     Ok(()) => println!("All dependencies installed and up-to-date."),
///     Err(errors) => {
///         for error in errors {
///             match error {
///                 DependencyError::UpdateFailed => println!("Failed to update Rustup and Cargo."),
///                 DependencyError::ComponentInstallFailed(components) => {
///                     println!("Failed to install Rustup components: {:?}", components);
///                 }
///                 DependencyError::SubcommandsInstallFailed(subcommands) => {
///                     println!("Failed to install Cargo subcommands: {:?}", subcommands);
///                 }
///                 DependencyError::SystemBinariesNotInstalled(binaries) => {
///                     println!("Required system binaries not found: {:?}", binaries);
///                 }
///             }
///         }
///     }
/// }
/// ```
pub fn update_and_install_dependencies() -> Result<(), Vec<DependencyError>> {
    let mut dependency_error: Vec<DependencyError> = Vec::new();

    println!(
        "{}",
        Yellow.italic().paint("[1/5] Updating Rustup and Cargo...")
    );
    if let Err(error) = update() {
        dependency_error.push(error);
    }

    println!(
        "{}",
        Yellow
            .italic()
            .paint("[2/5] Installing Rustup components...")
    );
    if let Err(error) = install_rustup_components() {
        dependency_error.push(error);
    }

    println!(
        "{}",
        Yellow
            .italic()
            .paint("[3/5] Installing Cargo subcommands...")
    );
    if let Err(error) = install_cargo_subcommands() {
        dependency_error.push(error);
    }

    println!(
        "{}",
        Yellow.italic().paint("[4/5] Checking system binaries...")
    );
    if let Err(error) = check_system_binaries() {
        dependency_error.push(error);
    }

    println!("{}", Yellow.italic().paint("[5/5] Check errors..."));
    match dependency_error.is_empty() {
        true => {
            println!(
                "{}",
                Green.italic().bold().paint("-> No errors encountered.\n")
            );
            Ok(())
        }
        false => {
            println!(
                "{}",
                Red.italic()
                    .bold()
                    .paint(format!("-> Faced {} errors.\n", dependency_error.len()))
            );
            Err(dependency_error)
        }
    }
}

/// Checks if a binary is installed and in PATH.
///
/// # Arguments
///
/// * `name` - The name of the binary to check.
///
/// # Returns
///
/// `true` if the binary is installed and in PATH, `false` otherwise.
fn is_installed(name: &str) -> bool {
    which::which(name).is_ok()
}

/// Updates Rustup and Cargo.
///
/// # Returns
///
/// `Ok(())` if the update is successful, `Err(DependencyError)` if the update fails.
pub fn update() -> Result<(), DependencyError> {
    let update_1 = execute_command_no_path("rustup", &["self", "update"], false);
    let update_2 = execute_command_no_path("rustup", &["update"], false);

    match update_1.is_err() || update_2.is_err() {
        true => Err(DependencyError::UpdateFailed),
        false => Ok(()),
    }
}

/// Installs required Rustup components.
///
/// # Returns
///
/// `Ok(())` if installation is successful, `Err(DependencyError)` if installation fails.
pub fn install_rustup_components() -> Result<(), DependencyError> {
    let mut install_failed: Vec<String> = Vec::new();

    for component in RUSTUP_COMPONENT_LIST {
        if !is_installed(component) {
            let args: Vec<&str> = if component == &"rustfmt" {
                vec!["component", "add", component, "--toolchain", "stable"]
            } else {
                vec![
                    "component",
                    "add",
                    component.split('-').collect::<Vec<&str>>()[1],
                ]
            };

            if execute_command_no_path("rustup", &args, false).is_err() {
                install_failed.push(component.to_string());
            }
        }
    }

    match install_failed.is_empty() {
        true => Ok(()),
        false => Err(DependencyError::ComponentInstallFailed(install_failed)),
    }
}

/// Installs required Cargo subcommands.
///
/// # Returns
///
/// `Ok(())` if installation is successful, `Err(DependencyError)` if installation fails.
fn install_cargo_subcommands() -> Result<(), DependencyError> {
    let mut install_failed: Vec<String> = Vec::new();

    for subcommand in CARGO_SUBCOMMAND_LIST {
        if !is_installed(subcommand) {
            let args: Vec<&str> = if subcommand == &"cargo-outdated" {
                vec!["install", "--locked", subcommand]
            } else if subcommand == &"cargo-install-update" {
                vec!["install", "cargo-update"]
            } else if subcommand == &"cargo-modules" {
                vec!["install", subcommand, "--version", "0.5.14"]
            } else {
                vec!["install", subcommand]
            };

            if execute_command_no_path("cargo", &args, false).is_err() {
                install_failed.push(subcommand.to_string());
            }
        }
    }

    match install_failed.is_empty() {
        true => Ok(()),
        false => Err(DependencyError::SubcommandsInstallFailed(install_failed)),
    }
}

/// Checks required system binaries are installed.
///
/// # Returns
///
/// `Ok(())` if all required system binaries are found, `Err(DependencyError)` if any are missing.
fn check_system_binaries() -> Result<(), DependencyError> {
    let mut not_installed: Vec<String> = Vec::new();

    for binary in SYSTEM_BINARY_LIST {
        if !is_installed(binary) {
            not_installed.push(binary.to_string());
        }
    }

    match not_installed.is_empty() {
        true => Ok(()),
        false => Err(DependencyError::SystemBinariesNotInstalled(not_installed)),
    }
}
