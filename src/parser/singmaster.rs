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

use crate::{prelude::RubikTransform, tf};

pub fn parse<'a>(src: &'a str) -> Result<RubikTransform, Box<dyn Error + 'a>> {
    let input = src.as_ref();
    let (rest, output) = many0(modified_move)(input)?;
    if rest.is_empty() {
        Ok(RubikTransform::from(output.as_slice()))
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

impl From<&[ModifiedMove]> for RubikTransform {
    fn from(val: &[ModifiedMove]) -> Self {
        RubikTransform::Combine(val.iter().map(Into::into).collect())
    }
}

impl From<&ModifiedMove> for RubikTransform {
    fn from(val: &ModifiedMove) -> Self {
        let mut tf: RubikTransform = (&val.rubik_move).into();
        let mut inv = false;
        let mut repeat_times = 1;
        for modifier in &val.modifiers {
            match modifier {
                Modifier::Inverse => inv = !inv,
                Modifier::Repeat(n) => repeat_times *= n,
            }
        }
        if repeat_times == 0 {
            return RubikTransform::Combine(vec![]);
        }
        if inv {
            tf = tf.inverse();
        }
        if repeat_times > 1 {
            tf = tf.repeat(repeat_times);
        }
        tf
    }
}

impl From<&RubikMove> for RubikTransform {
    fn from(val: &RubikMove) -> Self {
        match val {
            RubikMove::Base(b) => b.into(),
            RubikMove::Group(g) => RubikTransform::Combine(g.iter().map(Into::into).collect()),
        }
    }
}

impl From<&BaseMove> for RubikTransform {
    fn from(val: &BaseMove) -> Self {
        use crate::transform::*;
        match val {
            BaseMove::F => tf!(F),
            BaseMove::B => tf!(B),
            BaseMove::L => tf!(L),
            BaseMove::R => tf!(R),
            BaseMove::U => tf!(U),
            BaseMove::D => tf!(D),
            BaseMove::M => tf!(M),
            BaseMove::E => tf!(E),
            BaseMove::S => tf!(S),
            BaseMove::X => tf!(R, MI, LI),
            BaseMove::Y => tf!(U, EI, DI),
            BaseMove::Z => tf!(F, SI, BI),
            BaseMove::RR => tf!(R, MI),
            BaseMove::LL => tf!(L, M),
            BaseMove::UU => tf!(U, EI),
            BaseMove::DD => tf!(D, E),
            BaseMove::FF => tf!(F, SI),
            BaseMove::BB => tf!(B, S),
        }
    }
}
