use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

/// Creates a `PathBuf` representing a directory path derived from a Git repository URL.
///
/// This function extracts the repository name from the provided Git repository URL,
/// and appends it to the current working directory to create a `PathBuf`.
///
/// # Arguments
///
/// * `git_url` - The URL of the Git repository from which to derive the directory name.
///
/// # Returns
///
/// A `PathBuf` representing the path to the directory created by appending the extracted repository name to the current working directory.
///
/// # Example
///
/// ```
/// use rca::download::create_path_from_repo_name;
/// use std::env;
///
/// let git_url = "https://github.com/hknio/rca.git";
/// let path = create_path_from_repo_name(git_url);
/// assert_eq!(path.to_str().unwrap(), env::current_dir().unwrap().to_str().unwrap().to_string() + "/rca");
/// ```
///
/// # Note
///
/// The function expects a valid Git repository URL in the format "https://github.com/user/repo.git" or similar, where "user" is the username or organization and "repo" is the repository name.
pub fn create_path_from_repo_name(git_url: &str) -> PathBuf {
    let repository_name: String = git_url
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();

    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(repository_name);
    path
}

/// Downloads a Git repository from the provided URL to the current directory.
///
/// This function clones a Git repository from the specified URL to the current working directory
/// if the repository does not already exist there.
///
/// # Arguments
///
/// * `git_url` - The URL of the Git repository to be downloaded.
///
/// # Returns
///
/// A `PathBuf` representing the path to the downloaded or existing repository.
///
/// # Example
///
/// ```
/// use rca::download::download_from_git;
/// use std::env;
///
/// let git_url = "https://github.com/hknio/rca.git";
/// let path = download_from_git(git_url);
/// assert_eq!(path.to_str().unwrap(), env::current_dir().unwrap().to_str().unwrap().to_string() + "/rca");
/// ```
///
/// # Note
///
/// If the repository already exists in the current directory, the function will not perform a download
/// and will instead print "Repository already exists."
pub fn download_from_git(git_url: &str) -> PathBuf {
    let path = create_path_from_repo_name(&git_url);

    if !path.exists() {
        let output: std::process::Output = Command::new("git")
            .args(["clone", git_url])
            .output()
            .expect("Failed to execute process");

        println!("Status: {}", output.status);

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        println!("Repository already exists.");
    }

    path
}
