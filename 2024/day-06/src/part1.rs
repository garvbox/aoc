use glam::UVec2;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
#[tracing::instrument]
fn parse_row(row: usize, input: &str) -> Vec<Entity> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = (row as u32, column as u32);
            match ch {
                '#' => Some(Entity::Obstruction(UVec2::from(pos))),
                '^' => Some(Entity::Guard(GuardPosition::new(pos.0, pos.1))),
                '.' => None,
                _ => unreachable!("Parser missed possible input {:?}", input),
            }
        })
        .collect()
}

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

    #[test]
    fn test_parse_empty_row() -> miette::Result<()> {
        let input = "..........";
        let expected: Vec<Entity> = vec![];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(GuardPosition::new(0, 4))];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_non_zero_row() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(GuardPosition::new(7, 4))];
        assert_eq!(expected, parse_row(7, input));
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_and_obstruction() -> miette::Result<()> {
        let input = "....^...#.";
        let expected: Vec<Entity> = vec![
            Entity::Guard(GuardPosition::new(0, 4)),
            Entity::Obstruction(UVec2::new(0, 8)),
        ];
        assert_eq!(expected, parse_row(0, input));
        Ok(())
    }
}
