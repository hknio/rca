use std::{
    ffi::OsStr,
    io::{self, Write},
    process::Command,
};

/// Executes a command with specified arguments in the given directory.
///
/// This function executes a command with the provided name and arguments in the specified directory (`path`).
///
/// # Arguments
///
/// * `name` - The name of the command to execute.
/// * `path` - The directory in which to execute the command.
/// * `args` - A slice of string arguments to pass to the command.
/// * `print_status` - A boolean flag indicating whether to print the command's exit status and output.
///
/// # Example
///
/// ```rust
/// use rca::command::execute_command;
/// use std::ffi::OsStr;
///
/// let command_name = "cargo";
/// let command_args = &["build"];
/// let directory = OsStr::new("./");
///
/// execute_command(command_name, directory, command_args, true);
/// ```
pub fn execute_command(name: &str, path: &OsStr, args: &[&str], print_status: bool) {
    let output = Command::new(name)
        .current_dir(path)
        .args(args)
        .output()
        .expect("failed to execute process");

    match print_status {
        true => println!("Status: {}", output.status),
        false => (),
    }

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

/// Executes a command with specified arguments without specifying a directory.
///
/// This function executes a command with the provided name and arguments in the current working directory.
///
/// # Arguments
///
/// * `name` - The name of the command to execute.
/// * `args` - A slice of string arguments to pass to the command.
/// * `print_status` - A boolean flag indicating whether to print the command's exit status and output.
///
/// # Returns
///
/// `Ok(())` if the command execution is successful, or `Err(io::Error)` if there is an error during execution.
///
/// # Example
///
/// ```rust
/// use rca::command::execute_command_no_path;
///
/// let command_name = "echo";
/// let command_args = &["hello world"];
///
/// match execute_command_no_path(command_name, command_args, true) {
///     Ok(()) => println!("Command executed successfully."),
///     Err(error) => eprintln!("Error executing command: {}", error),
/// }
/// ```
pub fn execute_command_no_path(
    name: &str,
    args: &[&str],
    print_status: bool,
) -> Result<(), io::Error> {
    let output: Result<std::process::Output, io::Error> = Command::new(name).args(args).output();

    match output {
        Err(error) => return Err(error),
        Ok(output) => {
            match print_status {
                true => println!("Status: {}", output.status),
                false => (),
            }

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();

            Ok(())
        }
    }
}

/// Executes a command with specified arguments in a given directory and returns the command's output.
///
/// This function takes the name of the command, the directory path, a list of arguments,
/// and a boolean flag to determine whether to print the command's exit status.
///
/// # Arguments
///
/// * `name` - The name of the command to execute.
/// * `path` - The directory path in which to execute the command.
/// * `args` - A slice of string arguments to pass to the command.
/// * `print_status` - A boolean flag indicating whether to print the command's exit status.
///
/// # Returns
///
/// Returns a `Result` containing either the command's output as a `String` on success or an `io::Error` on failure.
///
/// # Example
///
/// ```
/// use std::ffi::OsStr;
/// use std::io;
/// use rca::command::execute_command_return;
///
/// let command_name = "ls";
/// let path = OsStr::new(".");
/// let args = &[];
/// let print_status = true;
///
/// match execute_command_return(command_name, path, args, print_status) {
///     Ok(output) => {
///         println!("Command Output: {}", output);
///     }
///     Err(error) => {
///         eprintln!("Command Error: {}", error);
///     }
/// }
/// ```
///
pub fn execute_command_return(
    name: &str,
    path: &OsStr,
    args: &[&str],
    print_status: bool,
) -> Result<String, io::Error> {
    let output = Command::new(name).current_dir(path).args(args).output();

    match output {
        Err(error) => return Err(error),
        Ok(output) => {
            match print_status {
                true => println!("Status: {}", output.status),
                false => (),
            }

            return Ok(output.stdout.iter().map(|&i| i as char).collect::<String>());
        }
    }
}

/// Executes a command with specified arguments and returns the command's output.
///
/// This function takes the name of the command, a list of arguments,
/// and a boolean flag to determine whether to print the command's exit status.
///
/// # Arguments
///
/// * `name` - The name of the command to execute.
/// * `args` - A slice of string arguments to pass to the command.
/// * `print_status` - A boolean flag indicating whether to print the command's exit status.
///
/// # Returns
///
/// Returns a `Result` containing either the command's output as a `String` on success or an `io::Error` on failure.
///
/// # Example
///
/// ```
/// use std::io;
/// use rca::command::execute_command_no_path_return;
///
/// let command_name = "echo";
/// let args = &["Hello World"];
/// let print_status = true;
///
/// match execute_command_no_path_return(command_name, args, print_status) {
///     Ok(output) => {
///         println!("Command Output: {}", output);
///     }
///     Err(error) => {
///         eprintln!("Command Error: {}", error);
///     }
/// }
/// ```
///
pub fn execute_command_no_path_return(
    name: &str,
    args: &[&str],
    print_status: bool,
) -> Result<String, io::Error> {
    let output: Result<std::process::Output, io::Error> = Command::new(name).args(args).output();

    match output {
        Err(error) => return Err(error),
        Ok(output) => {
            match print_status {
                true => println!("Status: {}", output.status),
                false => (),
            }

            return Ok(output.stdout.iter().map(|&i| i as char).collect::<String>());
        }
    }
}
