#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;

pub fn alpha_or_space<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    // take_while(|c| ((c as char).is_whitespace() || c == 0x32|| (c as char).is_alphabetic()) && c != 0x97 && c != b'\r')(i)
    take_while(|c| ((c as char).is_whitespace() || (c as char).is_alphabetic()))(i)
}

pub fn is_alphanumeric_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphanumeric())
}

pub fn is_alphabetic_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphabetic())
}

pub fn alphanumeric_or_space<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_while(move |c: u8| is_alphanumeric_or_space(c as char))(i)
}

#[test]
#[rustfmt::skip]
fn parse_ical_lines() {
    let parsed1 = ical_lines(b"abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines(b"abcdefg\n----------");
    let parsed3 = ical_lines(b"abcdefg----------");

    assert_eq!(parsed1, Ok((&[][..], &b"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((&b"\n----------"[..], &b"abcdefg"[..])));
    assert_eq!(parsed3, Ok((&b""[..], &b"abcdefg----------"[..])));
}

#[test]
#[rustfmt::skip]
fn parse_ical_alphabetic() {
    let parsed1 = ical_lines_alphabetic(b"abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines_alphabetic(b"abcdefg\n----------");
    let parsed3 = ical_lines_alphabetic(b"abcdefg----------");
    let parsed4 = ical_lines_alphabetic(b"abcdefg1234567890");

    assert_eq!(parsed1, Ok((&[][..], &b"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((
         &b"\n----------"[..],
        &b"abcdefg"[..],
        )));
    assert_eq!(parsed3, Ok((
        &b"----------"[..],
        &b"abcdefg"[..],
    )));
    assert_eq!(parsed4, Ok((
        &b"1234567890"[..],
        &b"abcdefg"[..],
    )));
}

#[test]
#[rustfmt::skip]
fn parse_ical_alphanumeric() {
    let parsed1 = ical_lines_alphanumeric(b"abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines_alphanumeric(b"abcdefg\n----------");
    let parsed3 = ical_lines_alphanumeric(b"abcdefg----------");
    let parsed4 = ical_lines_alphanumeric(b"abcdefg1234567890");

    assert_eq!(parsed1, Ok((&[][..], &b"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((&b"\n----------"[..], &b"abcdefg"[..])));
    assert_eq!(parsed3, Ok((&b"----------"[..], &b"abcdefg"[..])));
    assert_eq!(parsed4, Ok((&b""[..], &b"abcdefg1234567890"[..])));
}

#[test]
fn ical_alphanumeric_equality() {
    let input1 = &b"abcdefghijklmnopqrstu"[..];
    let input2 = &b"abcd2345678lmnopqrstu"[..];
    let input3 = &b"abcd 345678 mnopqrstu"[..];
    let input3 = &b"abcd-----------------"[..];
    let input4 = &b"abcd 345678 mnopqr\ns"[..];
    let input5 = &b"abcd 345678 mnopqrs\n"[..];
    for input in &[input1, input2, input3, input4, input5] {
        assert_eq!(
            ical_lines_alphanumeric(input),
            alphanumeric_or_space(*input)
        );
    }
}

#[test]
fn ical_alphanumeric_no_equality() {
    let input1 = &b"abcdefg\n 123456789"[..];
    for input in &[input1] {
        assert_ne!(
            ical_lines_alphanumeric(input),
            alphanumeric_or_space(*input)
        );
    }
}

pub fn ical_lines<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    ical_lines_check(input, |_| true)
}
pub fn ical_lines_alphabetic<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    ical_lines_check(input, |x| (x as char).is_alphabetic())
}
pub fn ical_lines_alphanumeric<'a>(input: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    ical_lines_check(input, |x| (x as char).is_alphanumeric())
}
pub fn ical_lines_check<'a, F>(input: &'a [u8], check: F) -> IResult<&'a [u8], &'a [u8]>
where
    F: Fn(u8) -> bool,
{
    for (i, c) in input.windows(2).enumerate() {
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
    if input.last() == Some(&b'\n') {
        let remainder = &input[input.len() - 1..];
        let output = &input[..input.len() - 1];
        return Ok((remainder, output));
    }
    Ok((b"", input))
}

// fn transformed_alphanumeric_or_space(input: &[u8]) ->  IResult<&[u8], Vec<u8>> {
//     fn convert(i: &[u8]) -> IResult<&[u8], &[u8]>  {
//         alt((
//             map(tag(b"\\"), |_| "\\".as_bytes()),
//             map(tag(b"\""), |_| "\"".as_bytes()),
//             map(tag(b"n"),  |_| "\n".as_bytes()),
//         ))(i)
//     };
//
//     escaped_transform(alphanumeric_or_space, '\\', convert)(input)
// }
