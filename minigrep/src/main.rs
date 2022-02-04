use minigrep::*;
use std::env;
use std::error::Error;
use std::process;

fn main() {
    // get elements from an iterator let args: Vec<String> = env::args().collect();
    let bad = |err: Box<dyn Error>, message: &str| {
        eprintln!("{}: {}", message, err);
        process::exit(1);
    };
    let config = Config::new(env::args().skip(1)).unwrap_or_else(|err| bad(err, "Config Error:"));

    if let Err(e) = run(config) {
        bad(e, "Aplication Error");
    }
}
