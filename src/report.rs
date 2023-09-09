use serde::{Deserialize, Serialize};

/// A struct representing security-related information.
pub struct Security;

/// Information about a programming language's source code.
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageInfo {
    /// The name of the programming language.
    pub language: String,
    /// The number of blank lines in the source code.
    pub blanks: usize,
    /// The number of lines of code in the source code.
    pub code: usize,
    /// The number of lines containing comments in the source code.
    pub comments: usize,
}

/// Information about source lines of code (SLOC) for multiple programming languages.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sloc {
    /// Information about source code for different programming languages.
    pub language_info: Vec<LanguageInfo>,
    /// The total number of lines of code.
    pub code: usize,
    /// The total number of lines containing comments.
    pub comments: usize,
    /// Indicates if the SLOC data is inaccurate.
    pub inaccurate: bool,
}

/// Information about code coverage for individual source files.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileCoverage {
    /// The name of the source file.
    pub name: String,
    /// The lines in the file that are not covered by tests.
    pub uncovered_lines: Vec<u32>,
}

/// Information about code coverage for a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Coverage {
    /// Code coverage data for individual source files.
    pub file_coverage: Vec<FileCoverage>,
    /// The total code coverage percentage.
    pub total_coverage_percentage: f64,
    /// The total number of covered lines.
    pub num_covered_lines: u32,
    /// The total number of lines in the project.
    pub total_lines: u32,
}

/// Information about remote and local paths.
pub struct Information {
    /// The remote path.
    pub remote: String,
    /// The local path.
    pub local: String,
}

/// Quality-related information, including SLOC and code coverage.
#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    /// Information about source lines of code (SLOC).
    pub sloc: Sloc,
    /// Information about code coverage.
    pub coverage: Coverage,
}

/// A struct representing a comprehensive report.
pub struct Report {
    /// Information about paths (remote and local).
    pub information: Information,
    /// Quality-related information, including SLOC and code coverage.
    pub quality: Quality,
    /// Security-related information.
    pub security: Security,
}
