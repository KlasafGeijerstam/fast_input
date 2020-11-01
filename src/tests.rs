use super::*;

#[test]
fn test_empty() {
    let data = "".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!(false, input.has_next_line());
}

#[test]
fn test_read_line_as_split() {
    let src = "Lorem Ipsum Sit Dolor";
    let data = src.as_bytes();
    let input = FastInput::with_reader(data);
    let read: Vec<_> = input.next_split().collect();
    let truth: Vec<_> = src.split(' ').collect();
    assert_eq!(truth, read);
}

#[test]
fn read_single() {
    let data = "-123".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!(-123, input.next());
}

#[test]
fn read_tuple() {
    let data = "-123 127".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!((-123, 127), input.next_tuple());
}

#[test]
fn read_triple() {
    let data = "-123 127 -127".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!((-123, 127, -127), input.next_triple());
}

#[test]
fn read_quad() {
    let data = "-123 127".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!((-123, 127), input.next_tuple());
}

#[test]
fn read_quintuple() {
    let data = "-123 127 -127 123 127".as_bytes();
    let input = FastInput::with_reader(data);
    assert_eq!((-123, 127, -127, 123, 127), input.next_quintuple());
}

#[test]
fn read_next_line() {
    let src = "A very long line";
    let input = FastInput::with_reader(src.as_bytes());
    assert_eq!(src, input.next_line());
}

#[test]
fn read_next_as_iter() {
    let src = "1 2 3";
    let input = FastInput::with_reader(src.as_bytes());
    let read: Vec<i32> = input.next_as_iter().collect();
    assert_eq!([1, 2, 3], read[..]);
}

#[test]
fn read_some_lines() {
    let src = "1 2 3\n1 2 3\n1 2 3\n1 2 3";
    let input = FastInput::with_reader(src.as_bytes());
    for _ in 0..3 {
        let read: Vec<i32> = input.next_as_iter().collect();
        assert_eq!([1, 2, 3], read[..]);
    }
}
