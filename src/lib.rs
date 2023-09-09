/// Provides functionality for executing shell commands.
pub mod cmd;
pub mod command;

/// Manages Rust toolchain dependencies and system binaries.
pub mod dependencies;

/// Handles the downloading of Git repositories.
pub mod download;

/// Identifies and reports issues in Rust code.
pub mod issues;

/// Evaluates and reports code quality metrics.
pub mod quality;

/// Structs for reports based on code analysis results.
pub mod report;

/// Defines the target path for code analysis.
pub mod target;
