use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

type NumberPair = (u32, u32);

pub fn process(input: &str) -> miette::Result<String> {
    let (remaining, parsed_pairs) = parse(input).unwrap();
    tracing::trace!("Parsed: {:?}, Remaining: {:?}", parsed_pairs, remaining);

    let res: u32 = parsed_pairs.iter().map(|(a, b)| a * b).sum();
    Ok(res.to_string())
}

fn parsemul(input: &str) -> IResult<&str, NumberPair> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, (pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<NumberPair>> {
    many1(many_till(anychar, parsemul).map(|(_discard, res)| res))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
