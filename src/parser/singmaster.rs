use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_digit,
    combinator::{map, map_res, value},
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::prelude::RubikTransformGroup;

pub fn parse<'a>(src: &'a str) -> Result<RubikTransformGroup, Box<dyn Error + 'a>> {
    let input = src.as_ref();
    let (rest, output) = many0(modified_move)(input)?;
    if rest.is_empty() {
        Ok(RubikTransformGroup::from(output.as_slice()))
    } else {
        Err(format!("parse error near: {}", String::from_utf8(rest.to_vec())?).into())
    }
}

#[derive(Debug, Clone, Copy)]
enum Modifier {
    Inverse,
    Repeat(usize),
}

#[derive(Debug, Clone, Copy)]
enum BaseMove {
    F,
    B,
    L,
    R,
    U,
    D,
    M,
    E,
    S,
    X,
    Y,
    Z,
    RR,
    LL,
    UU,
    DD,
    FF,
    BB,
}

#[derive(Debug, Clone)]
enum RubikMove {
    Base(BaseMove),
    Group(Vec<ModifiedMove>),
}
#[derive(Debug, Clone)]

struct ModifiedMove {
    pub rubik_move: RubikMove,
    pub modifiers: Vec<Modifier>,
}

fn modifier(input: &[u8]) -> IResult<&[u8], Modifier> {
    alt((
        value(Modifier::Inverse, tag("'")),
        map_res(take_while(is_digit), |s: &[u8]| {
            String::from_utf8_lossy(s)
                .parse::<usize>()
                .map(Modifier::Repeat)
        }),
    ))(input)
}

fn base_move(input: &[u8]) -> IResult<&[u8], BaseMove> {
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

fn rubik_move(input: &[u8]) -> IResult<&[u8], RubikMove> {
    alt((
        delimited(
            tag("("),
            map(many0(modified_move), RubikMove::Group),
            tag(")"),
        ),
        map(base_move, RubikMove::Base),
    ))(input)
}

fn modified_move(input: &[u8]) -> IResult<&[u8], ModifiedMove> {
    let (input, rubik_move) = rubik_move(input)?;
    let (input, modifiers) = many0(modifier)(input)?;
    Ok((
        input,
        ModifiedMove {
            rubik_move,
            modifiers,
        },
    ))
}

impl From<&[ModifiedMove]> for RubikTransformGroup {
    fn from(val: &[ModifiedMove]) -> Self {
        RubikTransformGroup::Combine(val.iter().map(Into::into).collect())
    }
}

impl From<&ModifiedMove> for RubikTransformGroup {
    fn from(val: &ModifiedMove) -> Self {
        let mut tf: RubikTransformGroup = (&val.rubik_move).into();
        for modifier in &val.modifiers {
            tf = match modifier {
                Modifier::Inverse => tf.inverse(),
                Modifier::Repeat(n) => tf.repeat(*n),
            }
        }
        tf
    }
}

impl From<&RubikMove> for RubikTransformGroup {
    fn from(val: &RubikMove) -> Self {
        match val {
            RubikMove::Base(b) => b.into(),
            RubikMove::Group(g) => RubikTransformGroup::Combine(g.iter().map(Into::into).collect()),
        }
    }
}

impl From<&BaseMove> for RubikTransformGroup {
    fn from(val: &BaseMove) -> Self {
        match val {
            BaseMove::F => RubikTransformGroup::F,
            BaseMove::B => RubikTransformGroup::B,
            BaseMove::L => RubikTransformGroup::L,
            BaseMove::R => RubikTransformGroup::R,
            BaseMove::U => RubikTransformGroup::U,
            BaseMove::D => RubikTransformGroup::D,
            BaseMove::M => RubikTransformGroup::M,
            BaseMove::E => RubikTransformGroup::E,
            BaseMove::S => RubikTransformGroup::S,
            BaseMove::X => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::R,
                RubikTransformGroup::M.inverse(),
                RubikTransformGroup::L.inverse(),
            ]),
            BaseMove::Y => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::U,
                RubikTransformGroup::E.inverse(),
                RubikTransformGroup::D.inverse(),
            ]),
            BaseMove::Z => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::F,
                RubikTransformGroup::S.inverse(),
                RubikTransformGroup::B.inverse(),
            ]),
            BaseMove::RR => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::R,
                RubikTransformGroup::M.inverse(),
            ]),
            BaseMove::LL => {
                RubikTransformGroup::Combine(vec![RubikTransformGroup::L, RubikTransformGroup::M])
            }
            BaseMove::UU => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::U,
                RubikTransformGroup::E.inverse(),
            ]),
            BaseMove::DD => {
                RubikTransformGroup::Combine(vec![RubikTransformGroup::D, RubikTransformGroup::E])
            }
            BaseMove::FF => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::F,
                RubikTransformGroup::S.inverse(),
            ]),
            BaseMove::BB => RubikTransformGroup::Combine(vec![
                RubikTransformGroup::B,
                RubikTransformGroup::S.inverse(),
            ]),
        }
    }
}
