use rca::download;
use rca::download::git_download;
use rca::issues;
use rca::quality;
use rca::target::TargetPath;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    /*
    if let Err(e) = dependencies::update_and_install_dependencies() {
        for error in e {
            eprintln!("{}", error)
        }
        process::exit(-1);
    }
    */

    let target = env::args().skip(1).take(1).collect::<String>();
    let path = download::git_download(&target);
    println!("Target: {}", path.display());

    quality::fast_search(path.as_os_str());
    issues::search(path.as_os_str());
    Ok(())
}
