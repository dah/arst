use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    name: String,
    count: u32,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    let point = Point { x: 1, y: 2 };
    let serialised = serde_json::to_string(&point).unwrap();
    println!("serialised = {}", serialised);

    let deserialised: Point = serde_json::from_str(&serialised).unwrap();
    println!("deserialised = {:?}", deserialised);

    //let poem = fs::read_to_string(Path::new("~/.config/arst/poem.txt")).expect("Couldn't read any file.");

    let mut file = File::open("~/.config/arst/poem.txt").expect("Couldn't Open File");
    let mut poem = String::new();
    file.read_to_string(&mut poem)
        .expect("Unable to read contents");
    println!("Poem: \n\n{}", poem);
}
