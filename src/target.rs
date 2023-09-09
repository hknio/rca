use regex::Regex;
use std::path::PathBuf;
use thiserror::Error;

/// Custom error type for `TargetPath` operations.
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPathError {
    #[error("Local path `{0}` does not exist.")]
    LocalPathDoesNotExist(String),
}

/// Represents a target path, which can be either a local filesystem path or a remote repository URL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPath {
    /// A local filesystem path.
    Path(PathBuf),
    /// A remote repository URL.
    RemoteRepository(String),
}

impl TargetPath {
    /// Creates a new `TargetPath` instance based on the provided input string.
    ///
    /// If the input string matches the regular expression for a remote repository URL, it creates a `RemoteRepository`.
    /// If the input string represents an existing local filesystem path, it creates a `Path`.
    ///
    /// # Arguments
    ///
    /// * `target_path` - The input string representing a path or a remote repository URL.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetPath)` - If the input string is valid and corresponds to either a local path or a remote repository URL.
    /// * `Err(TargetPathError)` - If the input string is neither a valid local path nor a remote repository URL.
    ///
    /// # Example
    ///
    /// ```
    /// use rca::target::TargetPath;
    ///
    /// match TargetPath::new("./src".to_string()) {
    ///     Ok(target_path) => {
    ///         assert!(target_path.is_local());
    ///     },
    ///     Err(_) => {},
    /// }
    ///
    /// match TargetPath::new("https://github.com/hknio/rca.git".to_string()) {
    ///     Ok(target_path) => {
    ///         assert!(target_path.is_remote());
    ///     },
    ///     Err(_) => {},
    /// }
    /// ```
    pub fn new(target_path: String) -> Result<TargetPath, TargetPathError> {
        let regex: Regex =
            Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(//)?)([\w\.@:/\-~]+)(\.git)(/)?")
                .expect("Fatal Error: Cannot create regular expression");

        if regex.is_match(&target_path) {
            Ok(TargetPath::RemoteRepository(target_path))
        } else {
            let path_buffer: PathBuf = PathBuf::from(target_path.clone());
            if path_buffer.as_path().exists() {
                Ok(TargetPath::Path(path_buffer))
            } else {
                Err(TargetPathError::LocalPathDoesNotExist(target_path))
            }
        }
    }

    /// Checks if the `TargetPath` is a local path.
    ///
    /// # Returns
    ///
    /// * `true` - If the `TargetPath` represents a local filesystem path.
    /// * `false` - If the `TargetPath` represents a remote repository URL.
    pub fn is_local(&self) -> bool {
        match self {
            TargetPath::Path(_) => true,
            TargetPath::RemoteRepository(_) => false,
        }
    }

    /// Checks if the `TargetPath` is a remote repository URL.
    ///
    /// # Returns
    ///
    /// * `true` - If the `TargetPath` represents a remote repository URL.
    /// * `false` - If the `TargetPath` represents a local filesystem path.
    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }
}

/// Represents a target.
pub struct Target {
    /// The path associated with the target.
    pub path: PathBuf,
}

impl Target {
    // Additional methods and functionality can be added here.
}
