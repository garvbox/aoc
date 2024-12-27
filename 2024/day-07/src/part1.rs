use nom::{
    bytes::complete::tag, character::complete, multi::separated_list0, sequence::separated_pair,
    IResult,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let result: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_discard, res) = parse(line).unwrap();
            tracing::info!("Line Parse result: {:?}, from input {:?}", res, line);
            1 // FIXME: Return something meaningful here...
        })
        .sum();
    tracing::info!("Result: {:?}", result);
    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Equation> {
    let (_remaining, pair) = separated_pair(
        complete::u32,
        tag(": "),
        separated_list0(complete::space1, complete::u32),
    )(input)?;
    tracing::info!("Parse Result: {:?}", pair);
    Ok((
        input,
        Equation {
            test_value: pair.0,
            numbers: pair.1,
        },
    ))
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Equation {
    test_value: u32,
    numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
