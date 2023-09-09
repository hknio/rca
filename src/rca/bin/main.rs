use ansi_term::Colour::{Green, Yellow};
use rca::{dependencies, download, issues, quality, target};
use std::{env, error::Error, io::Write};

/// Downloads a Git repository specified as a command-line argument.
///
/// This function downloads a Git repository from the provided URL and returns the path to the downloaded repository.
///
/// # Returns
///
/// A `std::path::PathBuf` representing the path to the downloaded repository.
fn download_repository() -> std::path::PathBuf {
    let target: String = env::args().skip(1).take(1).collect::<String>();
    let target_path: target::TargetPath = target::TargetPath::new(target.clone()).unwrap();
    if target_path.is_local() {
        match target_path {
            target::TargetPath::Path(path) => path,
            _ => panic!("Fatal Error: TargetPath is not a local path"),
        }
    } else {
        download::download_from_git(&target[..])
    }
}

/// Downloads and installs dependencies if requested by the user.
///
/// This function asks the user if they want to install dependencies, and if the response is 'y', it installs Rust toolchain dependencies.
///
/// # Returns
///
/// `Ok(())` if the installation is successful, or an error if there is an issue with user input or dependency installation.
fn download_dependencies() -> Result<(), Box<dyn Error>> {
    print!("\nDo you want to install dependencies? (y/n): ");
    std::io::stdout().flush()?;

    loop {
        let mut option: String = String::new();
        let _ = std::io::stdin().read_line(&mut option)?;

        match option.trim() {
            "y" => {
                println!("{}", Green.bold().paint("Installing Dependencies..."));
                dependencies::update_and_install_dependencies().unwrap();
                break;
            }
            "n" => {
                println!("");
                break;
            }
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}

/// Performs actions based on user-selected options.
///
/// This function displays a menu of options for analyzing Rust code and performs the selected action based on the user's input.
///
/// # Arguments
///
/// * `path` - The path to the downloaded repository.
///
/// # Returns
///
/// `Ok(())` if the action is successful, or an error if there is an issue with user input or action execution.
fn do_action(path: &std::ffi::OsStr) -> Result<(), Box<dyn Error>> {
    loop {
        println!("{}", Yellow.bold().paint("\nMenu:"));
        println!("\t1. Find compilation errors and warnings");
        println!("\t2. Find formatting issues");
        println!("\t3. Find outdated dependencies");
        println!("\t4. Find vulnerable dependencies");
        println!("\t5. Find integer arithmetics");
        println!("\t6. Find error handling and unwrapping");
        println!("\t7. Find number of SLOC");
        println!("\t8. Find dependency graph");
        println!("\t9. Find code coverage");
        println!("\t10. Find all issues");
        println!("\t11. Find all quality metrics");
        println!("\t12. Generate Quality Report");
        println!("\t13. Exit");

        print!("\nChoose an option: ");
        std::io::stdout().flush()?;

        let mut option: String = String::new();
        let _ = std::io::stdin().read_line(&mut option)?;

        match option.trim() {
            "1" => issues::find_compilation_errors(&path),
            "2" => issues::find_formatting_issues(&path),
            "3" => issues::find_outdated_dependencies(&path),
            "4" => issues::find_vulnerable_dependencies(&path),
            "5" => issues::find_integer_arithmetics(&path),
            "6" => issues::find_unwrap_expect(&path),
            "7" => quality::search_sloc_number(&path),
            "8" => quality::search_dependency_graph(&path),
            "9" => quality::search_code_coverage(&path),
            "10" => issues::search(&path),
            "11" => quality::search(&path),
            "12" => quality::generate_quality_report(&path)?,
            "13" => break,
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}

/// Main entry point for the Rust Code Analyzer.
///
/// This function serves as the main entry point for the Rust Code Analyzer. It initiates the download of a Git repository, installs dependencies, and provides a menu for the user to choose actions.
///
/// # Returns
///
/// `Ok(())` if the program runs successfully, or an error if there are issues with downloading, dependency installation, or user actions.
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", Green.bold().paint("Welcome to Rust Code Analyzer!\n"));

    let path: std::path::PathBuf = download_repository();
    let path: &std::ffi::OsStr = path.as_os_str();

    download_dependencies()?;

    do_action(path)?;

    Ok(())
}
