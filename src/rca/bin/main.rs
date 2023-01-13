use rca::issues;
use rca::quality;
use std::env;
use std::error::Error;
//use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    /*
    if let Err(e) = dependencies::update_and_install_dependencies() {
        for error in e {
            eprintln!("{}", error)
        }
        process::exit(-1);
    }
    */

    let path = env::args().skip(1).take(1).collect::<String>();
    println!("Target: {}", path);

    quality::search(&path);
    issues::search(&path);
    Ok(())
}
