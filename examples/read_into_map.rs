use fast_input::{FastInput, Str};
use std::collections::HashMap;
use std::io::Write;

fn main() {
    println!("Enter String number tuples (hello 2)");
    let inp = FastInput::new();
    let mut people = HashMap::new();
    while inp.has_next_line() {
        let (name, age) = inp.next_tuple::<Str, u32>();
        *people.entry(*name).or_default() = age;
    }
    println!("{:?}", people);
}
