use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

pub enum Rotation {
    Left(i32),
    Right(i32),
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut counter = 0;
    let mut pos: i32 = 50;

    let (_, rotations) = rotations.parse(input).unwrap();

    for rotation in rotations {
        let count = match rotation {
            Rotation::Right(count) => count,
            Rotation::Left(count) => -count,
        };
        pos = (pos + count).rem_euclid(100);

        if pos == 0 {
            counter += 1;
        }
    }

    Ok(counter.to_string())
}

pub fn rotations(input: &str) -> IResult<&str, Vec<Rotation>> {
    separated_list1(line_ending, rotation).parse(input)
}

fn rotation(input: &str) -> IResult<&str, Rotation> {
    let (input, dir) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, num) = complete::i32(input)?;

    let d = match dir {
        "L" => Rotation::Left(num),
        "R" => Rotation::Right(num),
        x => panic!("Invalid: {x}"),
    };
    Ok((input, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
