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

    match find_hits(&s, config.pattern) {
        None => println!("No matchs found!"),
        Some(hits) => print_hits(&hits, config.pattern),
    }

    Ok(())
}

fn print_hits(hits: &[&str], pattern: &str) {
    let red_start = "\x1b[31m";
    let green_start = "\x1b[32m";
    let color_end = "\x1b[0m";
    let mut i = 1;

    for s in hits {
        let colorized = s.replace(pattern, format!("{red_start}{pattern}{color_end}").as_str());
        println!("{green_start}{i}{color_end}. {colorized}");
        i += 1;
    }
}

fn find_hits<'a>(s: &'a str, pattern: &str) -> Option<Vec<&'a str>> {
    if pattern.is_empty() || s.is_empty() {
        return None;
    }

    let mut hits = Vec::new();

    for line in s.lines() {
        if line.contains(pattern) {
            hits.push(line);
        }
    }

    if hits.is_empty() {
        return None;
    }
    Some(hits)
}

#[cfg(test)]
mod test {
    use crate::find_hits;

    #[test]
    fn finds_pattern() {
        // arrange
        let s = "we have this text with taste of cocoa";
        let pattern = "cocoa";
        // act
        let hits = find_hits(s, pattern);
        // assert
        assert!(hits.is_some());
        assert!(hits.as_ref().unwrap().len() == 1);
        assert!(hits.unwrap().iter().any(|s| s.contains("cocoa")));
    }

    #[test]
    fn does_not_find_pattern() {
        // arrange
        let s = "we have this text with taste of cocoa";
        let pattern = "sky";
        // act
        let hits = find_hits(s, pattern);
        // assert
        assert!(hits.is_none());
    }

    #[test]
    fn does_not_match_empty() {
        // arrange
        let s = "we have this text with taste of cocoa";
        let pattern = "";
        // act
        let hits = find_hits(s, pattern);
        // assert
        assert!(hits.is_none());
    }
}
