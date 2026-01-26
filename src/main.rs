use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{self, IsTerminal, Read, stdin},
    path::{Path, PathBuf},
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
        println!("couldn't open {}: {}", display, why);
    };
}

fn read_from_dir(s: &mut String) -> io::Result<()> {
    let dir = env::current_dir()?;
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_front(dir);

    // visit a node
    while let Some(path) = queue.pop_front() {
        visit_node(&mut queue, path, s)?;
    }

    Ok(())
}

fn visit_node(queue: &mut VecDeque<PathBuf>, current: PathBuf, s: &mut String) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            queue.push_back(path);
            continue;
        }
        let path = path.to_str().unwrap();
        read_from_file(path, s);
    }
    Ok(())
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
                let _ = read_from_dir(&mut s);
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
