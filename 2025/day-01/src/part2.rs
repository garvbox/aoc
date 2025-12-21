use crate::part1::{Rotation, rotations};
use nom::Parser;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut counter = 0;
    let mut pos: i32 = 50;

    let (_, rotations) = rotations.parse(input).unwrap();

    for rotation in rotations {
        let previous_pos = pos;
        match rotation {
            Rotation::Left(count) => pos = (pos - count).rem_euclid(100),
            Rotation::Right(count) => pos = (pos + count).rem_euclid(100),
        }

        if pos == 0 {
            counter += 1;
        }
    }

    Ok(counter.to_string())
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
