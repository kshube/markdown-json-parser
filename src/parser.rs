use crate::Markdown;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{map, not},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    IResult,
};

pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((
        map(parse_header, |e| Markdown::Heading(e)),
        map(parse_content, |e| Markdown::Paragraph(e)),
    )))(i)
}

fn parse_text(i: &str) -> IResult<&str, String> {
    map(
        many1(preceded(
            not(tag("\n")),
            take(1u8),
        )),
        |vec| vec.join(""),
    )(i)
}

fn parse_content(i: &str) -> IResult<&str, Vec<String>> {
    terminated(many0(parse_text), tag("\n"))(i)
}

fn parse_header_tag(i: &str) -> IResult<&str, usize> {
    map(
        terminated(take_while1(|c| c == '#'), tag(" ")),
        |s: &str| s.len(),
    )(i)
}

fn parse_header(i: &str) -> IResult<&str, Vec<String>> {
    preceded(parse_header_tag, parse_content)(i)
}