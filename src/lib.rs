use crate::{
    config::{Config, Source},
    readers::{read_from_dir, read_from_file},
};
use std::{
    error::Error,
    io::{Read, stdin},
};

pub mod config;
mod readers;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // string to hold the reads
    let mut s = String::new();

    dbg!(&config);

    match config.source {
        Source::File(path) => read_from_file(path, &mut s)
            .map_err(|why| format!("could not read from directory: {why}"))?,
        Source::Directory(path) => {
            read_from_dir(path, &mut s).map_err(|why| format!("could not read from file: {why}"))?
        }
        Source::Stdin => stdin()
            .read_to_string(&mut s)
            .map(|_| ())
            .map_err(|why| format!("could not read stdin to file: {why}"))?,
    };

    find_hits(&s, config.pattern);
    Ok(())
}

fn find_hits(s: &str, pattern: &str) {
    let red_start = "\x1b[31m";
    let green_start = "\x1b[32m";
    let color_end = "\x1b[0m";
    let mut i = 1;

    for line in s.lines() {
        if line.contains(pattern) {
            let colored =
                line.replace(pattern, format!("{red_start}{pattern}{color_end}").as_str());
            let trimmed = colored.trim();
            println!("{green_start}{i}{color_end}. {trimmed}");
            i += 1;
        }
    }
}
