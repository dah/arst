use clap::Parser;
use serde::{Deserialize, Serialize};

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
}
