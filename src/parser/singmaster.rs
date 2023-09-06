use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_till},
    character::is_digit,
    combinator::{map_res, value, map},
    sequence::delimited,
    multi::{many0, many1},
    IResult,
};

use crate::prelude::{RubikLayerTransform, RubikTransformGroup};
#[derive(Debug, Clone, Copy)]
pub enum Modifier {
    Inverse,
    Repeat(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum BaseMove {F,B,L,R,U,D,M,E,S,X,Y,Z,RR,LL,UU,DD,FF,BB}

#[derive(Debug, Clone)]
pub enum RubikMove {
    Base(BaseMove),
    Group(Vec<ModifiedMove>),
}
#[derive(Debug, Clone)]

pub struct ModifiedMove {
    pub rubik_move: RubikMove,
    pub modifiers: Vec<Modifier>,
}

impl From<&[ModifiedMove]> for RubikTransformGroup {
    fn from(val: &[ModifiedMove]) -> Self {
        todo!()
    }
}
pub fn modifier(input: &[u8]) -> IResult<&[u8], Modifier> {
    alt((
        value(Modifier::Inverse, tag("'")),
        map_res(take_while(is_digit), |s: &[u8]| {
            String::from_utf8_lossy(s).parse::<usize>().map(Modifier::Repeat)
        }),
    ))(input)
}

pub fn base_move(input: &[u8]) -> IResult<&[u8], BaseMove> {
    alt((
        value(BaseMove::F, tag("F")),
        value(BaseMove::B, tag("B")),
        value(BaseMove::L, tag("L")),
        value(BaseMove::R, tag("R")),
        value(BaseMove::U, tag("U")),
        value(BaseMove::D, tag("D")),
        value(BaseMove::M, tag("M")),
        value(BaseMove::E, tag("E")),
        value(BaseMove::S, tag("S")),
        value(BaseMove::X, tag("x")),
        value(BaseMove::Y, tag("y")),
        value(BaseMove::Z, tag("z")),
        value(BaseMove::RR, tag("r")),
        value(BaseMove::LL, tag("l")),
        value(BaseMove::UU, tag("u")),
        value(BaseMove::DD, tag("d")),
        value(BaseMove::FF, tag("f")),
        value(BaseMove::BB, tag("b")),
    ))(input)
}


pub fn rubik_move(input: &[u8])-> IResult<&[u8], RubikMove> {
    alt((
        delimited(
            tag("("),
            map(
                many0(modified_move),
                RubikMove::Group,
            ),
            tag(")"),
        ),
        map(base_move, RubikMove::Base),
    ))(input)
}

pub fn modified_move(input: &[u8]) -> IResult<&[u8], ModifiedMove> {
    let (input, rubik_move) = rubik_move(input)?;
    let (input, modifiers) = many0(modifier)(input)?;
    Ok((input, ModifiedMove { rubik_move, modifiers }))
}


pub fn parse<'a>(src: &'a str) -> Result<Vec<ModifiedMove>, Box<dyn Error + 'a>> {
    let input = src.as_ref();
    let (rest, output) = many0(modified_move)(input)?;
    if rest.is_empty() {
        Ok(output)
    } else {
        Err(format!("parse error near: {}", String::from_utf8(rest.to_vec())?).into())
    }
}