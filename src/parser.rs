use super::{Die, RolledDice};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list,
    sequence::{terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, RolledDice> {
    let (input, dice_pairings) =
        separated_list(tuple((space0, tag("+"), space0)), counted_dice)(input)?;

    let mut result = RolledDice::default();
    for (count, die) in dice_pairings {
        result.add_dice(&count, &die);
    }

    Ok((input, result))
}

fn counted_dice(input: &str) -> IResult<&str, (usize, Die)> {
    let (input, count) = map(opt(parse_usize), |v| v.unwrap_or(1))(input)?;
    let (input, die) = alt((
        parse_d4, parse_d6, parse_d8, parse_d10, parse_d12, parse_d20,
    ))(input)?;

    Ok((input, (count, die)))
}

fn parse_d4(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("4")), |_| Die::D4)(input)
}

fn parse_d6(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("6")), |_| Die::D6)(input)
}

fn parse_d8(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("8")), |_| Die::D8)(input)
}

fn parse_d10(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("10")), |_| Die::D10)(input)
}

fn parse_d12(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("12")), |_| Die::D12)(input)
}

fn parse_d20(input: &str) -> IResult<&str, Die> {
    map(terminated(parse_d_prefix, tag("20")), |_| Die::D20)(input)
}

fn parse_d_prefix(input: &str) -> IResult<&str, &str> {
    alt((tag("d"), tag("D")))(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}
