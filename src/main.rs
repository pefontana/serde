use std::{fs::File, io::Write};

use serde::Serialize;


#[derive(Debug, Serialize)]
enum Move {
  Up,
  Down,
  Right,
  Left,  
} 

fn main() {
    let a = Move::Up;

    let json = serde_json::to_string(&a).unwrap();

    let mut file = File::create("serde.txt").unwrap();
    file.write_all(json.as_bytes()).unwrap();

}
