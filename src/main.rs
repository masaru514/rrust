use std::env;
use std::process;
use rrust::minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application Erorr!: {}", e);
        process::exit(1);
    }
}
