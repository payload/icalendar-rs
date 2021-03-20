use std::iter::{self, Enumerate};

use nom::{
    bitvec::{order::Lsb0, slice::Windows},
    preceded,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;

pub fn alpha_or_space(i: &str) -> IResult<&str, &str> {
    // take_while(|c| ((c as char).is_whitespace() || c == 0x32|| (c as char).is_alphabetic()) && c != 0x97 && c != b'\r')(i)
    take_while(|c: char| (c.is_whitespace() || c.is_alphabetic()))(i)
}

pub fn alpha_or_dash(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| (c == '/' || c == '_' || c == '-' || c.is_alphabetic()))(i)
}

pub fn is_alphanumeric_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphanumeric())
}

pub fn is_alphabetic_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphabetic())
}

pub fn alphanumeric_or_space(i: &str) -> IResult<&str, &str> {
    take_while(is_alphanumeric_or_space)(i)
}

#[test]
#[rustfmt::skip]
fn parse_ical_line() {
    let parsed1 = ical_line("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_line("abcdefg\n----------");
    let parsed3 = ical_line("abcdefg----------");
    let parsed4 = ical_line("abcdefg\n hijklmnopqrstu\n vxyz\n");

    assert_eq!(parsed1, Ok(("", "abcdefg\n hijklmnopqrstu")));
    assert_eq!(parsed2, Ok(("\n----------", "abcdefg")));
    assert_eq!(parsed3, Ok(("", "abcdefg----------")));
    assert_eq!(parsed4, Ok(("\n", "abcdefg\n hijklmnopqrstu\n vxyz")));
}

#[test]
#[rustfmt::skip]
fn parse_ical_alphabetic() {
    let parsed1 = ical_line_alphabetic("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_line_alphabetic("abcdefg\n----------");
    let parsed3 = ical_line_alphabetic("abcdefg----------");
    let parsed4 = ical_line_alphabetic("abcdefg1234567890");

    assert_eq!(parsed1, Ok(("", &"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((
         &"\n----------"[..],
        &"abcdefg"[..],
        )));
    assert_eq!(parsed3, Ok((
        &"----------"[..],
        &"abcdefg"[..],
    )));
    assert_eq!(parsed4, Ok((
        &"1234567890"[..],
        &"abcdefg"[..],
    )));
}

#[test]
#[rustfmt::skip]
fn parse_ical_alphanumeric() {
    let parsed1 = ical_line_alphanumeric("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_line_alphanumeric("abcdefg\n----------");
    let parsed3 = ical_line_alphanumeric("abcdefg----------");
    let parsed4 = ical_line_alphanumeric("abcdefg1234567890");
    let parsed5 = ical_line_alphanumeric("abcdefg\n hijklmnopqrstu\n vwxz");

    assert_eq!(parsed1, Ok(("", &"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((&"\n----------"[..], &"abcdefg"[..])));
    assert_eq!(parsed3, Ok((&"----------"[..], &"abcdefg"[..])));
    assert_eq!(parsed4, Ok((&""[..], &"abcdefg1234567890"[..])));
}

#[test]
fn ical_alphanumeric_equality() {
    let input1 = &"abcdefghijklmnopqrstu"[..];
    let input2 = &"abcd2345678lmnopqrstu"[..];
    let input3 = &"abcd 345678 mnopqrstu"[..];
    let input3 = &"abcd-----------------"[..];
    let input4 = &"abcd 345678 mnopqr\ns"[..];
    let input5 = &"abcd 345678,;|opqrs\n"[..];
    for input in &[input1, input2, input3, input4, input5] {
        assert_eq!(ical_line_alphanumeric(input), alphanumeric_or_space(*input));
    }
}

#[test]
fn ical_alphanumeric_no_equality() {
    let input = &"abcdefg\n 123456789"[..];
    // for input in &[input1] {
    assert_ne!(ical_line_alphanumeric(input), alphanumeric_or_space(input));
    // }
}

pub fn ical_line(input: &str) -> IResult<&str, &str> {
    ical_line_check(input, |_| true)
}
pub fn ical_line_alphabetic(input: &str) -> IResult<&str, &str> {
    ical_line_check(input, |x| (x as char).is_alphabetic())
}

pub fn ical_line_alphanumeric(input: &str) -> IResult<&str, &str> {
    ical_line_check(input, |x| (x as char).is_alphanumeric())
}

pub fn ical_line_check<F>(input: &str, check: F) -> IResult<&str, &str>
where
    F: Fn(u8) -> bool,
{
    for (i, c) in input.as_bytes().windows(2).enumerate() {
        // println!("{:?}", (i, c, input.len()));
        let remainder = &input[i..];
        let output = &input[..i];
        if let Some(&x) = c.get(0) {
            if !(check(x) || (x as char).is_whitespace() || x == b'\n') {
                // println!("check failed {:?}", c);
                return Ok((remainder, output));
            }
        }
        if c.get(0) == Some(&b'\n') && c.get(1) != Some(&b' ') {
            // println!("no space after break {:?}", c);
            let remainder = &input[i..];
            let output = &input[..i];
            return Ok((remainder, output));
        }
    }
    // literally a corner case
    if input.as_bytes().last() == Some(&b'\n') {
        // TODO: cut off `'\r'` as well
        let remainder = &input[input.len() - 1..];
        let output = &input[..input.len() - 1];
        return Ok((remainder, output));
    }
    Ok(("", input))
}

pub fn ical_lines(input: &str) -> impl Iterator<Item = &str> {
    let mut rest = input;
    iter::from_fn(move || match preceded(opt(tag("\n")), ical_line)(rest) {
        Ok((left, "")) => None,
        Ok((left, line)) => {
            rest = left;
            Some(line)
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    })
}

#[derive(Debug)]
pub struct IcalLineReader<'a> {
    input: &'a str,
    index: usize,
    inner: Enumerate<std::slice::Windows<'a, u8>>,
}

impl<'a> IcalLineReader<'a> {
    pub fn new(input: &'a str) -> Self {
        let inner = input.as_bytes().windows(2).enumerate();
        let index = 0;
        Self {
            input,
            index,
            inner,
        }
    }
}

impl<'a> iter::Iterator for IcalLineReader<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let check = |_x| true;
        while let Some((i, window)) = self.inner.next() {
            println!(
                "\nnext {:?}",
                (i, (window[0] as char, window[1] as char), self.input.len())
            );

            let output = &self.input[..i];
            if let Some(&x) = window.get(0) {
                if !(check(x) || (x as char).is_whitespace() || x == b'\n') {
                    println!("check failed {:?}, ({:?})", window, output);
                    return Some(output);
                } else if x == b'\n' {
                    self.index = i;
                } else {
                    println!("continue{:?}, ({:?})", window, output);
                }
            }
            if window.get(0) == Some(&b'\n') && window.get(1) != Some(&b' ') {
                let output = &self.input[self.index..i];
                self.index = i;
                println!("no space after break {:?}", (window, output, self.index));
                return Some(output);
            }
        }
        if self.input.as_bytes().last() == Some(&b'\n') {
            // TODO: cut off `'\r'` as well
            self.index = self.input.len() - 1;
            let output = &self.input[..self.input.len() - 1];
            return Some(output);
        }
        None
    }
}

fn remove_extra_breaks(input: &str) -> String {
    input.replace("\n ", "")
}

#[test]
fn test_line_iterator() {
    let text: String = r#"1 hello world
2 hello
  world
3 hello world
4 hello world
"#
    .into();

    let lines = ical_lines(&text)
        .map(remove_extra_breaks)
        .collect::<Vec<_>>();
    assert_eq!(
        lines,
        vec![
            "1 hello world",
            "2 hello world",
            "3 hello world",
            "4 hello world",
        ]
    )
}

#[test]
fn test_line_iterator2() {
    let text: String = r#"1 hello world
2 hello
  world
3 hello world
4 hello world
"#
    .into();

    let lines = IcalLineReader::new(&text)
        .map(remove_extra_breaks)
        .take(4)
        .collect::<Vec<_>>();
    assert_eq!(
        lines,
        vec![
            "1 hello world",
            "2 hello world",
            "3 hello world",
            "4 hello world",
        ]
    )
}

// fn transformed_alphanumeric_or_space(input: &[u8]) ->  IResult<&[u8], Vec<u8>> {
//     fn convert(i: &[u8]) -> IResult<&[u8], &[u8]>  {
//         alt((
//             map(tag("\\"), |_| "\\".as_bytes()),
//             map(tag("\""), |_| "\"".as_bytes()),
//             map(tag("n"),  |_| "\n".as_bytes()),
//         ))(i)
//     };
//
//     escaped_transform(alphanumeric_or_space, '\\', convert)(input)
// }
