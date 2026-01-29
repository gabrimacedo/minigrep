use minigrep::config::Config;
use std::{
    env,
    io::{IsTerminal, stdin},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let has_stdin = !stdin().is_terminal();
    let config = Config::build(&args, has_stdin).unwrap_or_else(|e| {
        println!("problem parsing arguments: {e}!");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("application error: {e}");
        process::exit(1);
    };
}
