use std::io::prelude::*;
use std::str::{FromStr, from_utf8_unchecked};

/// Simplifies reading and parsing of known input in a speedy fashion.
///
/// Reads all data on standard in into a byte buffer. Provides
/// methods to simplify reading and parsing of lines, specifically
/// aimed to aid in competetive programming where the input is
/// known and correct. Most functions panic if the input is not correct
/// and on the specified format.
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
/// let mut input = FastInput::new();
/// // Must make into String as next_line returns a slice to the internal buffer
/// // and the second input line advances the internal buffer.
/// let first_line = input.next_line().to_owned();
/// let (a, b): (u32, u32) = input.next_tuple();
///
/// println!("First line was: {}, a + b = {}", first_line, a + b);
/// ```
pub struct FastInput {
    data: Vec<u8>,
    pos: usize,
    emptied: bool,
}

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
            data: FastInput::read_to_end(8196),
            pos: 0,
            emptied: false,
        }
    }

    /// Creates a new FastInput with a specified buffer size.
    ///
    /// For more information, see [`new`].
    pub fn with_buffer_size(buffer_size: usize) -> Self {
        FastInput {
            data: FastInput::read_to_end(buffer_size),
            pos: 0,
            emptied: false,
        }
    }

    /// Reads the next line and returns it.
    ///
    /// # Panics
    ///
    /// The function panics if there is no more data in the buffer.
    /// If you are unsure if there is a next line, see [`has_next_line`].
    pub fn next_line(&mut self) -> &str {
        if let Some(nline) = self.next_newline() {
            unsafe {
                let s = from_utf8_unchecked(&self.data[self.pos..nline]);
                self.pos = nline + 1;
                s
            }
        } else {
            unsafe {
                self.emptied = true;
                from_utf8_unchecked(&self.data[self.pos..])
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
    /// let mut input = FastInput::new();
    /// let number: i32 = input.next();
    /// println!("{}", number);
    /// ```
    pub fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
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
    /// let mut input = FastInput::new();
    /// let (age, length): (i32, f64) = input.next_tuple();
    /// println!("{} {}", age, length);
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_tuple<T1: FromStr, T2: FromStr>(&mut self) -> (T1, T2)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
    }

    /// Reads three elements separated by a space, and returns them as a triple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_triple<T1: FromStr, T2: FromStr, T3: FromStr>(&mut self) -> (T1, T2, T3)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (it.next().unwrap().parse().unwrap(),
         it.next().unwrap().parse().unwrap(),
         it.next().unwrap().parse().unwrap())
    }

    /// Reads four elements separated by a space, and returns them as a quad-tuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_quad<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr>(&mut self) -> (T1, T2, T3, T4)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
        <T4 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    /// Reads five elements separated by a space, and returns them as a quintuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_quintuple<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr>(&mut self) -> (T1, T2, T3, T4, T5)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
        <T4 as FromStr>::Err: std::fmt::Debug,
        <T5 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
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
    /// let mut input = FastInput::new();
    /// let numbers: Vec<u32> = input.next_as_iter().collect();
    /// println!("Last line contained {} numbers!", numbers.len());
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_as_iter<T: FromStr>(&mut self) -> impl Iterator<Item = T> + '_
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.next_line()
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
    }

    /// Reads the next line and returns an iterator over the elements (no parsing).
    ///
    /// # Examples
    ///
    /// Reading a sentence and printing the individual words:
    /// ```no_run
    /// use fast_input::FastInput;
    ///
    /// let mut input = FastInput::new();
    /// let words = input.next_split();
    /// for (i, word) in words.enumerate() {
    ///     println!("Word {} was: {}", i, word);
    /// }
    /// ```
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_split(&mut self) -> impl Iterator<Item = &str> + '_ {
        self.next_line()
            .trim()
            .split(' ')
    }

    /// Checks if there is more data available in the buffer.
    ///
    /// # Examples
    ///
    /// Reading until EOF:
    /// ```no_run
    /// use fast_input::FastInput;
    /// 
    /// let mut input = FastInput::new();
    /// while input.has_next_line() {
    ///     println!("{}", input.next_line());
    /// }
    /// ```
    pub fn has_next_line(&self) -> bool {
        self.pos != self.data.len()
    }

    /// Returns the next line as a str tuple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_str_tuple(&mut self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }

    /// Returns the next line as a str triple.
    ///
    /// # Panics
    /// If there is no more data in the buffer. See [`has_next_line`].
    pub fn next_str_triple(&mut self) -> (&str, &str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap(), line.next().unwrap())
    }

    fn read_to_end(buffer_size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(buffer_size);
        std::io::stdin().lock().read_to_end(&mut data).unwrap();
        data
    }

    fn next_newline(&mut self) -> Option<usize> {
        let mut i = self.pos;
        while i < self.data.len() && self.data[i] != '\n' as u8 {
            i += 1;
        }
        if i < self.data.len() && self.data[i] == '\n' as u8 {
            Some(i)
        } else {
            None
        }
    }
}
