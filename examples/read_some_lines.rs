use fast_input::FastInput;
use std::io::Write;

fn main() {
    solve();
}

fn solve<'a>() {
    print!("Enter some text, end with EOF (Ctrl + D): ");
    std::io::stdout().flush().expect("Failed to flush stdout..");

    let inp = FastInput::new();
    let line = inp.next_line();
    println!("The first line was: {}", line);
    
    let rest: Vec<_> = inp.lines().collect();
    println!("There where {} more lines after the first one", rest.len());
}
