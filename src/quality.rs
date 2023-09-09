use crate::{
    command,
    report::{self, Sloc},
};
use ansi_term::Colour::Green;
use serde_json::{to_string_pretty, Map, Value};
use std::{ffi::OsStr, fs::File, io::prelude::*};

/// Writes JSON content to a file.
///
/// This function takes a `name` and `content` as input and writes the `content` to a JSON file
/// with the provided `name`. The file will be created in the current working directory.
///
/// # Arguments
///
/// * `name` - The name of the JSON file (without the .json extension).
/// * `content` - The JSON content to be written to the file.
///
/// # Errors
///
/// Returns an error if there are any issues with file creation or writing.
///
/// # Example
///
/// ```
/// use rca::quality::write_json_to_file;
///
/// let name = "example";
/// let content = r#"{"key": "value"}"#.to_string();
/// let result = write_json_to_file(name.to_string(), content);
///
/// assert!(result.is_ok());
/// ```
pub fn write_json_to_file(name: String, content: String) -> std::io::Result<()> {
    let mut file = File::create(name + ".json")?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Generates a quality report and writes it to a JSON file.
///
/// This function generates a quality report for the code located at the specified `path`. It includes
/// metrics such as Source Lines of Code (SLOC) and code coverage. The generated report is then written
/// to a JSON file named "quality_report.json" in the current working directory.
///
/// # Arguments
///
/// * `path` - The path to the code repository for which to generate the quality report.
///
/// # Errors
///
/// Returns an error if there are any issues with generating or writing the quality report.
///
/// # Example
///
/// ```
/// use std::ffi::OsStr;
/// use rca::quality::generate_quality_report;
///
/// let path = OsStr::new("./");
/// let result = generate_quality_report(path);
///
/// assert!(result.is_ok());
/// ```
pub fn generate_quality_report(path: &OsStr) -> Result<(), Box<dyn std::error::Error>> {
    let sloc = search_sloc_number_json(path)?;
    let coverage = search_code_coverage_json(path)?;

    let quality_report = report::Quality { sloc, coverage };

    let quality_report = to_string_pretty(&quality_report)?;

    write_json_to_file("quality_report".to_string(), quality_report)?;

    Ok(())
}

/// Searches for various code metrics and information within a given path.
///
/// This function provides an interface to search for code metrics and information within a specified path.
/// It includes searches for Source Lines of Code (SLOC) numbers, dependency graph, and code coverage.
///
/// # Arguments
///
/// * `path` - The path for which to search for code metrics and information.
///
/// # Example
///
/// ```
/// use rca::quality::search;
///
/// let path = std::ffi::OsStr::new("../");
/// search(path);
/// ```
///
pub fn search(path: &OsStr) {
    search_sloc_number(path);
    search_dependency_graph(path);
    search_code_coverage(path);
}

/// Searches for the number of Source Lines of Code (SLOC) within a given path.
///
/// This function uses the `tokei` tool to count the number of Source Lines of Code (SLOC)
/// within the specified path and prints the result.
///
/// # Arguments
///
/// * `path` - The path for which to count the SLOC.
///
/// # Example
///
/// ```
/// use rca::quality::search_sloc_number;
///
/// let path = std::ffi::OsStr::new("../");
/// search_sloc_number(path);
/// ```
pub fn search_sloc_number(path: &OsStr) {
    println!("{}", Green.bold().paint("\n# Number of SLOC:"));
    if command::execute_command_no_path("tokei", &[path.to_str().unwrap()], false).is_err() {
        println!("Error: tokei is not installed.");
    }
}

/// Parses the JSON data to create a Source Lines of Code (SLOC) report.
///
/// This function takes a JSON object, parses it, and constructs a Source Lines of Code (SLOC) report
/// structure from the data. The resulting report contains information about code lines, comments, and
/// blank lines for various programming languages.
///
/// # Arguments
///
/// * `json` - A JSON object containing SLOC data.
///
/// # Returns
///
/// A `report::Sloc` structure representing the SLOC report.
///
fn parse_sloc_json(mut json: Map<String, Value>) -> report::Sloc {
    let mut json_report: Sloc = report::Sloc {
        language_info: Vec::new(),
        code: 0,
        comments: 0,
        inaccurate: false,
    };

    for (key, value) in json.iter_mut() {
        let mut value: Map<String, Value> = value.as_object_mut().unwrap().clone();

        value.remove("reports").unwrap();
        value.remove("children").unwrap();

        if key == &String::from("Total") {
            json_report.code = value["code"].as_u64().unwrap() as usize;
            json_report.comments = value["comments"].as_u64().unwrap() as usize;
            json_report.inaccurate = value["inaccurate"].as_bool().unwrap();
        } else {
            let language_info = report::LanguageInfo {
                language: key.clone(),
                blanks: value["blanks"].as_u64().unwrap() as usize,
                code: value["code"].as_u64().unwrap() as usize,
                comments: value["comments"].as_u64().unwrap() as usize,
            };

            json_report.language_info.push(language_info);
        }
    }

    json_report
}

/// Searches for the number of Source Lines of Code (SLOC) within a given path and returns the result as JSON.
///
/// This function uses the `tokei` tool to count the number of Source Lines of Code (SLOC)
/// within the specified path and returns the result as a JSON representation.
///
/// # Arguments
///
/// * `path` - The path for which to count the SLOC.
///
/// # Returns
///
/// A `Result` containing the SLOC report in JSON format if successful, or an error if the `tokei`
/// command is not installed or encounters other issues.
///
pub fn search_sloc_number_json(path: &OsStr) -> Result<report::Sloc, Box<dyn std::error::Error>> {
    println!("{}", Green.bold().paint("# Generating SLOC Report..."));

    if let Ok(output) = command::execute_command_no_path_return(
        "tokei",
        &[path.to_str().unwrap(), "-o", "json"],
        false,
    ) {
        let json: Map<String, Value> = serde_json::from_str(&output[..]).unwrap();
        let json_report: report::Sloc = parse_sloc_json(json);
        Ok(json_report)
    } else {
        Err("Error: tokei is not installed.".into())
    }
}

/// Searches for the dependency graph of a Rust project within a given path.
///
/// This function uses the `cargo tree` command to generate and display the dependency graph
/// of a Rust project located at the specified path.
///
/// # Arguments
///
/// * `path` - The path where the Rust project is located.
///
/// # Example
///
/// ```
/// use rca::quality::search_dependency_graph;
///
/// let path = std::ffi::OsStr::new("../");
/// search_dependency_graph(path);
/// ```
pub fn search_dependency_graph(path: &OsStr) {
    println!("{}", Green.bold().paint("\n# Dependency graph:"));
    command::execute_command("cargo", path, &["tree"], false);
}

/// Searches for code coverage information within a given path.
///
/// This function uses the `cargo tarpaulin` command to generate and display code coverage information
/// for a Rust project located at the specified path.
///
/// # Arguments
///
/// * `path` - The path where the Rust project is located.
///
/// # Example
///
/// ```
/// use rca::quality::search_code_coverage;
///
/// let path = std::ffi::OsStr::new("../");
/// search_code_coverage(path);
/// ```
pub fn search_code_coverage(path: &OsStr) {
    println!("{}", Green.bold().paint("\n# Code coverage:"));
    command::execute_command("cargo", path, &["tarpaulin"], false);
}

/// Parses the output of the Tarpaulin tool and extracts relevant coverage data.
///
/// This function takes the raw output from the Tarpaulin tool, splits it, and processes it
/// to extract coverage information for each file. It returns a vector of strings, where each
/// string represents coverage data for a file.
///
/// # Arguments
///
/// * `tarpaulin_output` - The raw output from the Tarpaulin tool.
///
/// # Returns
///
/// A vector of strings containing coverage data for each file.
///
pub fn parse_tarpaulin_output(tarpaulin_output: String) -> Vec<String> {
    let mut output = tarpaulin_output.split("||").collect::<Vec<&str>>();
    dbg!(&output);
    output.remove(0);
    output.remove(0);
    output
        .into_iter()
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>()
}

/// Parses the total code coverage data from Tarpaulin's output.
///
/// This function extracts and processes the total code coverage data from Tarpaulin's output.
/// It calculates the coverage percentage, number of covered lines, and total lines of code.
///
/// # Arguments
///
/// * `total_coverage_data` - The total code coverage data as a string.
///
/// # Returns
///
/// A `report::Coverage` structure representing the total code coverage.
///
pub fn parse_total_coverage_data(total_coverage_data: String) -> report::Coverage {
    let splitted_total = total_coverage_data
        .split(",")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();
    let coverage_percentage = splitted_total[0].split("%").collect::<Vec<&str>>()[0]
        .parse::<f64>()
        .unwrap();
    let num_lines = splitted_total[1].split(" ").collect::<Vec<&str>>()[0]
        .split("/")
        .collect::<Vec<&str>>();
    let num_covered_lines = num_lines[0].parse::<u32>().unwrap();
    let total_lines = num_lines[1].parse::<u32>().unwrap();

    report::Coverage {
        file_coverage: Vec::new(),
        total_coverage_percentage: coverage_percentage,
        num_covered_lines,
        total_lines,
    }
}

/// Parses the uncovered lines for each file from Tarpaulin's output.
///
/// This function extracts and processes the uncovered lines data for each file from Tarpaulin's output.
/// It returns a vector of `report::FileCoverage` structures, each representing a file and its uncovered lines.
///
/// # Arguments
///
/// * `files` - A vector of strings containing uncovered lines data for each file.
///
/// # Returns
///
/// A vector of `report::FileCoverage` structures representing each file's uncovered lines.
///
pub fn parse_each_file_uncovered_lines(files: Vec<String>) -> Vec<report::FileCoverage> {
    let mut files_uncovered_lines = Vec::new();

    for uncovered_data in files.iter() {
        let splitted_uncovered = uncovered_data.split(":").collect::<Vec<&str>>();

        let filename = splitted_uncovered[0];

        let uncovered_lines = splitted_uncovered[1]
            .split(",")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let uncovered_lines = uncovered_lines
            .into_iter()
            .map(|x| {
                if x.contains("-") {
                    let splitted = x.split("-").collect::<Vec<&str>>();
                    let start = splitted[0].parse::<u32>().unwrap();
                    let end = splitted[1].parse::<u32>().unwrap();
                    (start..=end).collect::<Vec<u32>>()
                } else {
                    vec![x.parse::<u32>().unwrap()]
                }
            })
            .flatten()
            .collect::<Vec<u32>>();

        files_uncovered_lines.push(report::FileCoverage {
            name: filename.to_string(),
            uncovered_lines,
        });
    }

    files_uncovered_lines
}

/// Searches for code coverage information using the Tarpaulin tool within a given path.
///
/// This function runs the `cargo tarpaulin` command with the specified path to generate
/// and display code coverage information for a Rust project. It returns a `Result` containing
/// the code coverage report if successful or an error if Tarpaulin is not installed or if
/// there are other issues.
///
/// # Arguments
///
/// * `path` - The path where the Rust project is located.
///
/// # Returns
///
/// A `Result` containing a `report::Coverage` structure representing code coverage data
/// if successful, or an error if Tarpaulin is not installed or if the command encounters issues.
///
pub fn search_code_coverage_json(
    path: &OsStr,
) -> Result<report::Coverage, Box<dyn std::error::Error>> {
    println!(
        "{}",
        Green.bold().paint("# Generating Code Coverage Report...")
    );

    if let Ok(tarpaulin_output) =
        command::execute_command_return("cargo", path, &["tarpaulin"], false)
    {
        let parsed_output = parse_tarpaulin_output(tarpaulin_output);

        let coverege_index = parsed_output
            .iter()
            .position(|x| x.contains("Total Lines"))
            .unwrap();

        let uncovered = Vec::from(&parsed_output[..coverege_index]);
        let total = parsed_output[parsed_output.len() - 1].to_string();

        let mut code_coverage = parse_total_coverage_data(total);

        code_coverage.file_coverage = parse_each_file_uncovered_lines(uncovered);

        Ok(code_coverage)
    } else {
        Err("Error: tarpaulin is not installed.".into())
    }
}
