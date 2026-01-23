use std::{env, fs::File, io::Read, path::Path};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // get pattern
    let pattern = args.get(1).unwrap();
    //
    // get path or use current dir
    let path = args.get(2).unwrap();
    let path = Path::new(path);
    let display = path.display();

    // get file handle
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // read file
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(_) => println!("file {} read sucessfully!", display),
    };

    let mut hits = Vec::new();
    // search for pattern
    for line in s.lines() {
        if line.contains(pattern) {
            hits.push(line);
        }
    }

    dbg!(hits);
    // TODO: highlight the word that matched
}
