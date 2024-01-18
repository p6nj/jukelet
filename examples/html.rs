use derive_new::new;
use jukelet::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{into, value, verify},
    error::Error,
    multi::many0,
    IResult,
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

#[derive(new)]
struct Env {
    maj: bool,
}

impl Symbol for Token {
    type Env = Env;
    type Output = char;
    fn zap(self, env: Self::Env) -> (Option<Self::Output>, Self::Env) {
        match self {
            Self::Letter(c) => (
                Some(match &env.maj {
                    true => c.to_ascii_uppercase(),
                    false => c.to_ascii_lowercase(),
                }),
                env,
            ),
            Self::MajOn => (None, Env::new(true)),
            Self::MajOff => (None, Env::new(false)),
        }
    }
}

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
    let zapped: String = Symbols::from(tokens("<maj>h</maj>ello <maj>w</maj>orld!").unwrap().1)
        .zap(Vec::new(), Env::new(false));
    println!("{:?}", zapped);
}
