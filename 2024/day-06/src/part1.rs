#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day_06 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}