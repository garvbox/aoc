use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub fn process(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let mut acc = 0;
    let mut is_next_disabled = false;

    for instruction in instructions {
        match instruction {
            Instruction::Do => {
                tracing::trace!("Processing Instruction::Do - disabled was: {is_next_disabled}");
                is_next_disabled = false
            }
            Instruction::Dont => {
                tracing::trace!("Processing Instruction::Dont - disabled was: {is_next_disabled}");
                is_next_disabled = true
            }
            Instruction::Mul(a, b) => {
                tracing::trace!("Processing Mult {a}*{b} - disabled:{is_next_disabled}");
                if !is_next_disabled {
                    tracing::trace!("Added {a}*{b}");
                    acc += a * b;
                }
            }
        }
    }
    Ok(acc.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction_or_mul).map(|(_remaining, res)| res))(input)
}

fn parse_instruction_or_mul(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parsemul,
    ))(input)
}

fn parsemul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
