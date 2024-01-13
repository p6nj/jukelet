use std::str::Chars;

use jukelet::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{anychar, char, one_of},
    combinator::{into, map_res, not, value, verify},
    error::Error,
    multi::many0,
    sequence::delimited,
    Err, IResult, InputIter, Parser,
};

#[derive(Clone)]
enum Token {
    Letter(char),
    MajOn,
    MajOff,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        Token::Letter(value)
    }
}

#[derive(Default)]
struct Env {
    maj: bool,
}

impl Symbol for Token {
    type Env = Env;
    type Output = char;
    fn zap(self, env: Self::Env) -> (Self::Output, Self::Env) {
        match self {
            Self::Letter(c) => (
                match &env.maj {
                    true => c.to_ascii_uppercase(),
                    false => c.to_ascii_lowercase(),
                },
                env,
            ),
            Self::MajOn => todo!(),
            Self::MajOff => todo!(),
        }
    }
}

type ParseResult<'a> = IResult<&'a str, Token>;

fn letter<'a>(input: &'a str) -> IResult<&'a str, Token> {
    into(verify(anychar::<&'a str, Error<&'a str>>, |s: &char| {
        s != &'<'
    }))(input)
}

fn majon<'a>(input: &'a str) -> IResult<&'a str, Token> {
    value(Token::MajOn, tag("<maj>"))(input)
}

fn majoff<'a>(input: &'a str) -> IResult<&'a str, Token> {
    value(Token::MajOff, tag("</maj>"))(input)
}

fn tokens<'a>(input: &'a str) -> IResult<&'a str, Vec<Token>> {
    many0(alt((letter, majon, majoff)))(input)
}

fn main() {
    let zappable: Symbols<Token, <Vec<Token> as IntoIterator>::IntoIter, Vec<Token>> =
        tokens("<maj>h</maj>ello <maj>w</maj>orld!")
            .unwrap()
            .1
            .into();
    let zapped = zappable.zap::<Chars, String>();
    println!("{zapped}");
}
