use std::path::PathBuf;
use url::Url;

pub enum Target {
    Path(PathBuf),
    Repository(Url),
}

impl Target {
    fn new(target_path: String) -> Self {
        match Url::parse(&target_path) {
            Ok(Url) => Target::Repository(Url),
            Err(_) => Target::Path(PathBuf::from(target_path)),
        }
    }
}
