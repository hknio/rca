use regex::Regex;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPathError {
    #[error("Local path `{0}` does not exist")]
    LocalPathDoesNotExist(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPath {
    Path(PathBuf),
    RemoteRepository(String),
}

impl TargetPath {
    pub fn new(target_path: String) -> Result<TargetPath, TargetPathError> {
        // Must not fail, or a crash is expected.
        let re =
            Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(//)?)([\w\.@:/\-~]+)(\.git)(/)?")
                .expect("Fatal error: Can not create regular expression");

        if re.is_match(&target_path) {
            Ok(TargetPath::RemoteRepository(target_path))
        } else {
            let pb = PathBuf::from(target_path.clone());
            if pb.as_path().exists() {
                Ok(TargetPath::Path(pb))
            } else {
                Err(TargetPathError::LocalPathDoesNotExist(target_path))
            }
        }
    }

    pub fn is_local(&self) -> bool {
        match self {
            TargetPath::Path(_) => true,
            TargetPath::RemoteRepository(_) => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }
}

pub struct Target {
    pub path: PathBuf,
}

impl Target {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_ssh_remote_repo_path() {
        let url = "git@github.com:hknio/rca.git";
        let target = TargetPath::new(url.to_string()).unwrap();
        assert_eq!(target, TargetPath::RemoteRepository(url.to_string()));
    }

    #[test]
    fn should_parse_https_remote_repo_path() {
        let url = "https://github.com/hknio/rca.git";
        let target = TargetPath::new(url.to_string()).unwrap();
        assert_eq!(target, TargetPath::RemoteRepository(url.to_string()));
    }

    #[test]
    fn should_parse_http_remote_repo_path() {
        let url = "http://github.com/hknio/rca.git";
        let target = TargetPath::new(url.to_string()).unwrap();
        assert_eq!(target, TargetPath::RemoteRepository(url.to_string()));
    }

    #[test]
    fn should_parse_local_repo_path() {
        let path = "./";
        let target = TargetPath::new(path.to_string()).unwrap();
        assert_eq!(target, TargetPath::Path(PathBuf::from(path)));
    }

    #[test]
    fn should_error_if_local_path_does_not_exist() {
        let path = "/path/does/not/exist";
        assert_eq!(
            TargetPath::new(path.to_string()),
            Err(TargetPathError::LocalPathDoesNotExist(path.to_string()))
        );
    }
}
