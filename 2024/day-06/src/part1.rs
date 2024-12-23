use glam::UVec2;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
#[tracing::instrument]
#[derive(Eq, PartialEq, Clone, Debug)]
enum Entity {
    Obstruction(UVec2),
    Guard(GuardPosition),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct GuardPosition {
    position: UVec2,
    direction: GuardDirection,
}

impl GuardPosition {
    fn new(x: u32, y: u32) -> GuardPosition {
        GuardPosition {
            position: UVec2::from((x, y)),
            direction: GuardDirection::North,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
enum GuardDirection {
    #[default]
    North,
    South,
    East,
    West,
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
