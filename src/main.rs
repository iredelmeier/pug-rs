use pug_tmp_workaround::parse;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let html = parse(buffer).unwrap();
    println!("{}", html);
}
