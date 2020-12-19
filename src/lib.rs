use std::cell::Cell;
use std::fmt::Display;
use std::io::prelude::*;
use std::io::stdin;
use std::ops::Deref;
use std::str::{from_utf8_unchecked, FromStr};

#[cfg(test)]
mod tests;

/// Simplifies reading and parsing of known input in a speedy fashion.
///
/// Reads all data on standard in into a byte buffer. Provides
/// methods to simplify reading and parsing of lines, specifically
/// aimed to aid in competetive programming where the input is
/// known and correct. Most functions panic if the input is not correct
/// and on the specified format. **Note: FastInput assumes *nix line-endings (`\n`)**.
///
/// FastInput uses interior mutability to allow for zero-copy reading and referencing
/// of string input.
///
/// # Examples
///
/// Creating a new `FastInput` and reading two lines:
///
/// ```no_run
/// // Input:
/// // Hello!
/// // 12 2000
/// use fast_input::FastInput;
///
/// let input = FastInput::new();
/// // Must make into String as next_line returns a slice to the internal buffer
/// // and the second input line advances the internal buffer.
/// let first_line = input.next_line().to_owned();
/// let (a, b): (u32, u32) = input.next_tuple();
///
/// println!("First line was: {}, a + b = {}", first_line, a + b);
/// ```
///
/// Mixing `&str` with parseables:
///
/// To facilitate simple, zero-copy reading of string slices, FastInput
/// defines the `Str` type. The following example reads (name, age) (&str, u8) pairs
/// and stores them in a HashMap.
/// ```no_run
/// use fast_input::{FastInput, Str};
/// use std::collections::HashMap;
///
/// // Input:
/// // Sven 12
/// // Lorna 22
/// let input = FastInput::new();
/// let mut map = HashMap::new();
/// let (sven, sven_age) = input.next_tuple::<Str, u8>();
/// let (lorna, lorna_age) = input.next_tuple::<Str, u8>();
///
/// // Deref the Str to a &str
/// map.insert(*sven, sven_age);
/// map.insert(*lorna, lorna_age);
/// assert_eq!(map["Sven"], 12);
/// ```
pub struct FastInput {
    data: Vec<u8>,
    pos: Cell<usize>,
}

const BUFFER_SIZE: usize = 8196;

#[allow(dead_code)]
impl FastInput {
    /// Creates a new FastInput.
    ///
    /// Upon creation the entire contents of standard input will be read.
    /// The function will block until EOF is reached. If you are using a
    /// terminal you can send EOF using `CTRL + D`. The initial buffer size
    /// is 8196 bytes.
    pub fn new() -> Self {
        FastInput {
            data: FastInput::read_to_end(stdin().lock(), BUFFER_SIZE),
            pos: Cell::new(0),
        }
    }

    /// Creates a new FastInput with a specified buffer size.
    ///
    /// For more information, see [`new`].
    pub fn with_buffer_size(buffer_size: usize) -> Self {
        FastInput {
            data: FastInput::read_to_end(stdin().lock(), buffer_size),
            pos: Cell::new(0),
        }
    }

    /// Creates a new FastInput with a given input that implements
    /// Read
    ///
    /// # Examples
    ///
    /// Creating a FastInput over a byte slice:
    /// ```
    /// use fast_input::FastInput;
    /// use std::io::Read;
    ///
    /// let data = "1 2\n3 4".as_bytes();
    ///
    /// let input = FastInput::with_reader(data);
    ///
    /// let (one, two) = input.next_tuple::<u32, u32>();
    /// let (three, four) = input.next_tuple::<u32, u32>();
    ///
    /// assert_eq!((1, 2), (one, two));
    /// assert_eq!((3, 4), (three, four));
    /// assert_eq!(false, input.has_next_line());
    /// ```
    /// For more information, see [`new`].
    pub fn with_reader<T: Read>(input: T) -> Self {
        FastInput {
            data: FastInput::read_to_end(input, BUFFER_SIZE),
            pos: Cell::new(0),
        }
    }

    /// Reads the next line and returns it.
    ///
    /// # Panics
    ///
    /// The function panics if there is no more data in the buffer.
    /// If you are unsure if there is a next line, see [`has_next_line`].
    pub fn next_line(&self) -> &str {
        if let Some(nline) = self.next_newline() {
            unsafe {
                let pos = self.pos.get();
                let s = from_utf8_unchecked(&self.data[pos..nline]);
                self.pos.set(nline + 1);
                s
            }
        } else {
            unsafe {
                let s = from_utf8_unchecked(&self.data[self.pos.get()..]);
                self.pos.set(self.data.len());
                s
            }
        }
    }

    /// Reads a single value and parses it.
    ///
    /// # Examples
    ///
    /// Reading an integer:
    /// ```no_run
    /// //Input:
    /// //123
    /// use fast_input::FastInput;
    ///
    /// let input = FastInput::new();
    /// let number: i32 = input.next();
    /// println!("{}", number);
    /// ```
    pub fn next<'a, T: FastParse<'a>>(&'a self) -> T {
        let mut it = self.next_as_iter();
        it.next().unwrap()
    }

    /// Reads two elements separated by a space, and returns them parsed as a tuple.
    ///
    /// # Examples
    ///
    /// Reading an `i32` and a `f64`:
    /// ```no_run
    /// use fast_input::FastInput;
    ///
    /// let input = FastInput::new();
    /// let (age, length): (i32, f64) = input.next_tuple();
    /// println!("{} {}", age, length);
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_tuple<'a, T1: FastParse<'a>, T2: FastParse<'a>>(&'a self) -> (T1, T2) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
        )
    }

    /// Reads three elements separated by a space, and returns them as a triple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_triple<'a, T1: FastParse<'a>, T2: FastParse<'a>, T3: FastParse<'a>>(
        &'a self,
    ) -> (T1, T2, T3) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
        )
    }

    /// Reads four elements separated by a space, and returns them as a quad-tuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_quad<
        'a,
        T1: FastParse<'a>,
        T2: FastParse<'a>,
        T3: FastParse<'a>,
        T4: FastParse<'a>,
    >(
        &'a self,
    ) -> (T1, T2, T3, T4) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
            T4::fparse(it.next().unwrap()),
        )
    }

    /// Reads five elements separated by a space, and returns them as a quintuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_quintuple<
        'a,
        T1: FastParse<'a>,
        T2: FastParse<'a>,
        T3: FastParse<'a>,
        T4: FastParse<'a>,
        T5: FastParse<'a>,
    >(
        &'a self,
    ) -> (T1, T2, T3, T4, T5) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
            T4::fparse(it.next().unwrap()),
            T5::fparse(it.next().unwrap()),
        )
    }

    /// Reads the next line and returns an iterator over the elements of the line.
    ///
    /// # Examples
    ///
    /// Collecting a line into a [`Vec`] of integers.
    /// ```no_run
    /// use fast_input::FastInput;
    ///
    /// let input = FastInput::new();
    /// let numbers: Vec<u32> = input.next_as_iter().collect();
    /// println!("Last line contained {} numbers!", numbers.len());
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_as_iter<'a, T: FastParse<'a>>(&'a self) -> impl Iterator<Item = T> + '_ {
        self.next_line().trim().split(' ').map(|x| T::fparse(x))
    }

    /// Reads the next line and returns an iterator over the elements (no parsing).
    ///
    /// # Examples
    ///
    /// Reading a sentence and printing the individual words:
    /// ```no_run
    /// use fast_input::FastInput;
    ///
    /// let input = FastInput::new();
    /// let words = input.next_split();
    /// for (i, word) in words.enumerate() {
    ///     println!("Word {} was: {}", i, word);
    /// }
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_split<'a>(&'a self) -> impl Iterator<Item = &'a str> + '_ {
        self.next_line().trim().split(' ')
    }

    /// Checks if there is more data available in the buffer.
    ///
    /// # Examples
    ///
    /// Reading until EOF:
    /// ```no_run
    /// use fast_input::FastInput;
    ///
    /// let input = FastInput::new();
    /// while input.has_next_line() {
    ///     println!("{}", input.next_line());
    /// }
    /// ```
    pub fn has_next_line(&self) -> bool {
        self.pos.get() != self.data.len()
    }

    /// Returns the next line as a str tuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    #[deprecated(
        since = "0.1.1",
        note = "Use `next_tuple` with the `Str` type instead."
    )]
    pub fn next_str_tuple(&self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }

    /// Returns the next line as a str triple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    #[deprecated(
        since = "0.1.1",
        note = "Use `next_triple` with the `Str` type instead."
    )]
    pub fn next_str_triple(&self) -> (&str, &str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        )
    }

    fn read_to_end<T: Read>(mut input: T, buffer_size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(buffer_size);
        input.read_to_end(&mut data).unwrap();
        data
    }

    fn next_newline(&self) -> Option<usize> {
        let mut i = self.pos.get();
        while i < self.data.len() && self.data[i] != b'\n' {
            i += 1;
        }
        if i < self.data.len() && self.data[i] == b'\n' {
            Some(i)
        } else {
            None
        }
    }

    pub fn lines<'a>(&'a self) -> impl Iterator<Item = &str> + 'a {
        (0..).take_while(move |_| self.has_next_line())
            .map(move |_| self.next_line())
    }
}

impl Default for FastInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for parsing.
/// Mainly used to avoid repeating type constraints.
pub trait FastParse<'a> {
    /// Parses a type from a string slice
    fn fparse(s: &'a str) -> Self;
}

impl<'a, T: FromStr> FastParse<'a> for T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn fparse(s: &'a str) -> Self {
        s.parse().unwrap()
    }
}

/// Allows reading of string slices (`&str`).
/// The standard library does not provide a `FromStr` implementation
/// for `&str`. The `Str` type newtypes `&str` and implements `FastParse`
/// and `Deref<Target = &str>`.
///
/// # Examples
///
/// Reading (name, age, city) triples using `Str` and `FastInput`:
/// ```rust
/// use fast_input::{FastInput, Str};
/// let data = "Jakub 26 Mora".as_bytes();
/// let input = FastInput::with_reader(data);
/// let (name, age, city) = input.next_triple::<Str, u8, Str>();
/// // Str implements Display
/// println!("The person is called {}, is {} years old and lives in {}", name, age, city);
///
/// //To use any functions related to `&str`, dereference the `Str` into a `&str`
/// let name: &str = *name;
///
/// ```
pub struct Str<'a>(&'a str);

impl<'a> FastParse<'a> for Str<'a> {
    fn fparse(s: &'a str) -> Self {
        Str::<'a>(s)
    }
}

impl<'a> Deref for Str<'a> {
    type Target = &'a str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Str<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}
