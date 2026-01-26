use std::{
    env,
    fs::File,
    io::{IsTerminal, Read, stdin},
    path::Path,
};

fn read_from_file(path: &str, s: &mut String) {
    let path = Path::new(path);
    let display = path.display();

    // get file handle
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // read file
    if let Err(why) = file.read_to_string(s) {
        panic!("couldn't open {}: {}", display, why);
    };
}

fn read_from_current_dir(_: &mut String) {
    let dir = env::current_dir().unwrap();
    println!("current dir is: {}", dir.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // get pattern TODO: handle none
    let pattern = args.get(1).unwrap();

    let mut s = String::new();

    // read from file if specified, stdin otherwise, TODO: curr dir otherwsise
    match args.get(2) {
        Some(path) => read_from_file(path, &mut s),
        None => {
            if stdin().is_terminal() {
                read_from_current_dir(&mut s);
            } else {
                let sin = stdin().read_to_string(&mut s).unwrap();
                println!("copied {} bytes", sin);
            }
        }
    }

    let red_start = "\x1b[31m";
    let red_end = "\x1b[0m";
    // search for pattern
    for line in s.lines() {
        if line.contains(pattern) {
            let colored = line.replace(pattern, format!("{red_start}{pattern}{red_end}").as_str());
            let trimmed = colored.trim();
            println!("\t{trimmed}");
        }
    }
}
