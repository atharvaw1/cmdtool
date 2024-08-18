use std::env;
use std::process;
use cmdtool::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file name {}", config.file_path);
    if let Err(e) = cmdtool::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
