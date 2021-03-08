#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;

pub fn alpha_or_space<'a>(i: &str) -> IResult<&str, &str> {
    // take_while(|c| ((c as char).is_whitespace() || c == 0x32|| (c as char).is_alphabetic()) && c != 0x97 && c != b'\r')(i)
    take_while(|c| ((c as char).is_whitespace() || (c as char).is_alphabetic()))(i)
}

pub fn is_alphanumeric_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphanumeric())
}

pub fn is_alphabetic_or_space(c: char) -> bool {
    c != '\n' && (c.is_whitespace() || c.is_alphabetic())
}

pub fn alphanumeric_or_space<'a>(i: &str) -> IResult<&str, &str> {
    take_while(move |c| is_alphanumeric_or_space(c as char))(i)
}

#[test]
#[rustfmt::skip]
fn parse_ical_lines() {
    let parsed1 = ical_lines("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines("abcdefg\n----------");
    let parsed3 = ical_lines("abcdefg----------");

    assert_eq!(parsed1, Ok(("", &"abcdefg\n hijklmnopqrstu"[..])));
    assert_eq!(parsed2, Ok((&"\n----------"[..], &"abcdefg"[..])));
    assert_eq!(parsed3, Ok((&""[..], &"abcdefg----------"[..])));
}

#[test]
#[rustfmt::skip]
fn parse_ical_alphabetic() {
    let parsed1 = ical_lines_alphabetic("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines_alphabetic("abcdefg\n----------");
    let parsed3 = ical_lines_alphabetic("abcdefg----------");
    let parsed4 = ical_lines_alphabetic("abcdefg1234567890");

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
    let parsed1 = ical_lines_alphanumeric("abcdefg\n hijklmnopqrstu");
    let parsed2 = ical_lines_alphanumeric("abcdefg\n----------");
    let parsed3 = ical_lines_alphanumeric("abcdefg----------");
    let parsed4 = ical_lines_alphanumeric("abcdefg1234567890");

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
        assert_eq!(
            ical_lines_alphanumeric(input),
            alphanumeric_or_space(*input)
        );
    }
}

#[test]
fn ical_alphanumeric_no_equality() {
    let input1 = &"abcdefg\n 123456789"[..];
    for input in &[input1] {
        assert_ne!(
            ical_lines_alphanumeric(input),
            alphanumeric_or_space(*input)
        );
    }
}

pub fn ical_lines<'a>(input: &str) -> IResult<&str, &str> {
    ical_lines_check(input, |_| true)
}
pub fn ical_lines_alphabetic<'a>(input: &str) -> IResult<&str, &str> {
    ical_lines_check(input, |x| (x as char).is_alphabetic())
}
pub fn ical_lines_alphanumeric<'a>(input: &str) -> IResult<&str, &str> {
    ical_lines_check(input, |x| (x as char).is_alphanumeric())
}
pub fn ical_lines_check<'a, F>(input: &str, check: F) -> IResult<&str, &str>
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
        let remainder = &input[input.len() - 1..];
        let output = &input[..input.len() - 1];
        return Ok((remainder, output));
    }
    Ok(("", input))
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
