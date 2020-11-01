![Rust](https://github.com/KlasafGeijerstam/fast_input/workflows/Rust/badge.svg?event=push)
# FastInput - Read input, fast!

FastInput is aimed to simplify reading of known input from `stdin`,
mainly in a competetive programming environment. The crate exposes
the `FastInput` struct which contains methods for reading and parsing
line-based input. `FastInput` does no validation of the input and
uses unsafe operations and buffering to achieve great performance.


## Example

The following example creates a new `FastInput` and reads some input:

```rust
use fast_input::{FastInput, Str};

let input = FastInput::new();
let first_line = input.next_line();

// Type arguments can often be omitted as rust is awesome,
// specified here for clarity.
let (a, b): (u32, u32) = input.next_tuple();

println!("First line was: {}, a + b = {}", first_line, a + b);

// Read a line of integers and collect into a `Vec`
let numbers: Vec<u32> = input.next_as_iter().collect();

// `FastInput` contains methods to read up to quintuples
let (a, b, c, d, e) = input.next_quintuple();
let sum: 0i32 + a + b + c + d + e;


// The `Str` type can be used to mix string slices with parsed data
let (name, age) = input.next_tuple::<Str, u8>();
// `Str` can be dereferenced into its contained string slice
let name: &str = *name;

// Read all remaining lines and print them
while input.has_next_line() {
    println!("{}", input.next_line());
}
```
