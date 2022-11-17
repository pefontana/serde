use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Move {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    // SERDE
    let a = Move::Up;

    let json = serde_json::to_string(&a).unwrap();

    let mut file = File::create("serde.txt").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    file = File::open("serde.txt").unwrap();
    let reader = BufReader::new(file);
    let b: Move = serde_json::from_reader(reader).unwrap();

    fs::remove_file("serde.txt").unwrap();
    assert_eq!(a, b);

    println!("a : {:?}", a);
    println!("b_serde : {:?}", b);

    // RON

    let ron = ron::to_string(&a).unwrap();

    let mut file = File::create("ron.txt").unwrap();
    file.write_all(ron.as_bytes()).unwrap();

    file = File::open("ron.txt").unwrap();
    let reader = BufReader::new(file);
    let b_ron = ron::de::from_reader(reader).unwrap();
    fs::remove_file("ron.txt").unwrap();
    assert_eq!(a, b_ron);

    println!("b_ron : {:?}", b_ron);

    // let ron = ron::to_string(&a).unwrap();
    let ron_buffer: Vec<u8> = ron::ser::to_string(&a).unwrap().into_bytes();
    println!("ron_buffer : {:?}", ron_buffer);

    let c: Move = ron::de::from_bytes(&ron_buffer).unwrap();
    println!("c : {:?}", c);
}
