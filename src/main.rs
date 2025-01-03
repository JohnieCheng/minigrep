use minigrep_io_practice::Config;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_io_practice::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
