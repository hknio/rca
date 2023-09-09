use crate::command;
use ansi_term::Colour::Green;
use std::ffi::OsStr;

/// Represents different kinds of code issues that can be searched for.
///
/// This enum defines various kinds of code issues that can be searched for within a codebase,
/// including formatting issues, compilation warnings and errors, outdated and vulnerable dependencies,
/// integer arithmetic, and error handling practices.
pub enum IssueKind {
    Formatting,
    CompilationWarning,
    CompilationError,
    OutdatedDependency,
    VulnerableDependency,
    UnboundedDataStructure,
    OutOfBound,
    ErrorHandling,
}

/// Searches for multiple code issues within a given path.
///
/// This function provides an interface to search for multiple code issues within a specified path.
/// It includes searches for compilation errors and warnings, formatting issues, outdated dependencies,
/// vulnerable dependencies, integer arithmetic issues, and error handling practices.
///
/// # Arguments
///
/// * `path` - The path for which to search for code issues.
///
/// # Example
///
/// ```
/// use rca::issues::search;
///
/// let path = std::ffi::OsStr::new("../");
/// search(path);
/// ```
pub fn search(path: &OsStr) {
    find_compilation_errors(path);
    find_formatting_issues(path);
    find_outdated_dependencies(path);
    find_vulnerable_dependencies(path);
    find_integer_arithmetics(path);
    find_unwrap_expect(path);
}

/// Searches for compilation errors and warnings within a given path.
///
/// This function uses the `cargo check` command to check for compilation errors and warnings
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check compilation errors and warnings.
///
/// # Example
///
/// ```
/// use rca::issues::find_compilation_errors;
///
/// let path = std::ffi::OsStr::new("../");
/// find_compilation_errors(path);
/// ```
pub fn find_compilation_errors(path: &OsStr) {
    println!(
        "{}",
        Green.bold().paint("\n# Compilation Errors & Warnings")
    );
    command::execute_command("cargo", path, &["check"], false);
}

/// Searches for formatting issues within a given path.
///
/// This function uses the `cargo fmt --check` command to check for formatting issues
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check formatting issues.
///
/// # Example
///
/// ```
/// use rca::issues::find_formatting_issues;
///
/// let path = std::ffi::OsStr::new("../");
/// find_formatting_issues(path);
/// ```
pub fn find_formatting_issues(path: &OsStr) {
    println!("\n{}", Green.bold().paint("\n# Formatting Issues"));
    command::execute_command("cargo", path, &["fmt", "--check"], false);
}

/// Searches for outdated dependencies within a given path.
///
/// This function uses the `cargo outdated` command to check for outdated dependencies
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check outdated dependencies.
///
/// # Example
///
/// ```
/// use rca::issues::find_outdated_dependencies;
///
/// let path = std::ffi::OsStr::new("../");
/// find_outdated_dependencies(path);
/// ```
pub fn find_outdated_dependencies(path: &OsStr) {
    println!("\n{}", Green.bold().paint("\n# Outdated Dependencies"));
    command::execute_command("cargo", path, &["outdated"], false);
}

/// Searches for vulnerable dependencies within a given path.
///
/// This function uses the `cargo audit` command to check for vulnerable dependencies
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check vulnerable dependencies.
///
/// # Example
///
/// ```
/// use rca::issues::find_vulnerable_dependencies;
///
/// let path = std::ffi::OsStr::new("../");
/// find_vulnerable_dependencies(path);
/// ```
pub fn find_vulnerable_dependencies(path: &OsStr) {
    println!("\n{}", Green.bold().paint("\n# Vulnerable Dependencies"));
    command::execute_command("cargo", path, &["audit"], false);
}

/// Searches for integer arithmetic issues within a given path.
///
/// This function uses the `cargo clippy` command to check for integer arithmetic issues
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check integer arithmetic issues.
///
/// # Example
///
/// ```
/// use rca::issues::find_integer_arithmetics;
///
/// let path = std::ffi::OsStr::new("../");
/// find_integer_arithmetics(path);
/// ```
pub fn find_integer_arithmetics(path: &OsStr) {
    println!("\n{}", Green.bold().paint("\n# Integer Arithmetics"));
    command::execute_command(
        "cargo",
        path,
        &[
            "clippy",
            "--",
            "-A",
            "clippy::all",
            "-D",
            "clippy::arithmetic_side_effects",
        ],
        true,
    );
}

/// Searches for error handling practices and unwrapping within a given path.
///
/// This function uses the `cargo clippy` command to check for error handling practices
/// and unwrapping of results within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to check error handling practices and unwrapping.
///
/// # Example
///
/// ```
/// use rca::issues::find_unwrap_expect;
///
/// let path = std::ffi::OsStr::new("../");
/// find_unwrap_expect(path);
/// ```
pub fn find_unwrap_expect(path: &OsStr) {
    println!(
        "\n{}",
        Green.bold().paint("\n# Error Handling & Unwrapping")
    );
    command::execute_command(
        "cargo",
        path,
        &[
            "clippy",
            "--",
            "-A",
            "clippy::all",
            "-D",
            "clippy::unwrap_used",
            "-D",
            "clippy::expect_used",
        ],
        true,
    );
}
