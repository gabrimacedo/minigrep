use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{self, IsTerminal, Read, stdin},
    path::PathBuf,
};

fn read_from_file(path: PathBuf, s: &mut String) -> io::Result<()> {
    let mut file = File::open(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("could not open file {}: {}", path.display(), e),
        )
    })?;

    file.read_to_string(s).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("could not read file {} to string: {}", path.display(), e),
        )
    })?;

    Ok(())
}

fn read_from_dir(s: &mut String, path: PathBuf) -> io::Result<()> {
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_front(path);

    // visit a node
    while let Some(path) = queue.pop_front() {
        match visit_node(&mut queue, path, s) {
            Err(e) if e.kind() == io::ErrorKind::InvalidData => continue,
            Err(e) => return Err(e),
            Ok(_) => {}
        }
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
        read_from_file(path, s)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // get pattern TODO: handle none
    let pattern = args.get(1).unwrap();
    // string to hold the reads
    let mut s = String::new();

    match args.get(2) {
        // if we get arg, check if dir or file
        Some(path) => {
            let path = PathBuf::from(path);
            if path.is_dir() {
                if let Err(why) = read_from_dir(&mut s, path) {
                    println!("could not read from directory: {}", why)
                };
            } else {
                let _ = read_from_file(path, &mut s);
            }
        }
        // if no arg, check stdin
        None => {
            if stdin().is_terminal() {
                let current = env::current_dir().unwrap();
                let _ = read_from_dir(&mut s, current);
            }
            // else run on current dir
            else {
                let sin = stdin().read_to_string(&mut s).unwrap();
                println!("copied {} bytes", sin);
            }
        }
    }

    find_hits(&s, pattern);
}

fn find_hits(s: &str, pattern: &str) {
    let red_start = "\x1b[31m";
    let red_end = "\x1b[0m";

    for line in s.lines() {
        if line.contains(pattern) {
            let colored = line.replace(pattern, format!("{red_start}{pattern}{red_end}").as_str());
            let trimmed = colored.trim();
            println!("\t{trimmed}");
        }
    }
}
