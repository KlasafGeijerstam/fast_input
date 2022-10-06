use fast_input::{FastInput, Str, FastParse};
use std::collections::HashMap;

fn main() {
    println!("Enter String number tuples (hello 2)");
    let inp = FastInput::new();
    let mut people = HashMap::new();
    while inp.has_next_line() {
        let (name, age): (Str, u16) = inp.next();
        *people.entry(*name).or_default() = age;
    }
    println!("{:?}", people);
}
