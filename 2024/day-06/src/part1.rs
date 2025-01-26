use std::collections::HashSet;

use glam::UVec2;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (bounds, mut guard, obstructions) = parse_input(input);
    let mut tracked_positions: HashSet<UVec2> = HashSet::from([guard.position]);

    while !is_exiting_bounds(&guard, &bounds) {
        let next_position = get_next_position(&guard);
        if obstructions.contains(&next_position) {
            // Make no move if we are to hit an obstruction, do not count positions
            guard.direction = guard.direction.pivot();
            tracing::debug!("Pivoted to {:?}", guard.direction,);
        } else {
            tracing::debug!(
                "Moving Guard at {:?} to {:?}",
                guard.position,
                next_position
            );
            guard.position = next_position;
            tracked_positions.insert(guard.position);
        }
    }
    tracing::debug!("Guard Final Position: {:?}", guard);

    return Ok(tracked_positions.len().to_string());
}

pub fn parse_input(input: &str) -> (UVec2, GuardLocation, HashSet<UVec2>) {
    let entities: Vec<Entity> = input
        .lines()
        .rev() // NOTE: Input is processed row by row in reverse so that X-Y coords make sense
        .skip_while(|line| line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| parse_row(row, line))
        .collect();

    let bounds: UVec2 = match entities.last() {
        Some(Entity::Obstruction(pos) | Entity::Guard(pos) | Entity::None(pos)) => {
            UVec2::new(pos.x, pos.y)
        }
        _ => unreachable!("Something went horribly wrong here..."),
    };

    let mut guard: GuardLocation = GuardLocation::new(0, 0);
    let obstructions: HashSet<UVec2> = entities
        .iter()
        .filter_map(|i| match i {
            Entity::Obstruction(position) => Some(*position),
            Entity::Guard(position) => {
                guard.position = *position;
                None
            }
            Entity::None(_) => None,
        })
        .collect();
    tracing::trace!("Obstructions: {:?}", obstructions);
    tracing::debug!("Guard Initial Position: {:?}", guard);

    (bounds, guard, obstructions)
}

#[tracing::instrument]
pub fn is_exiting_bounds(guard: &GuardLocation, bounds: &UVec2) -> bool {
    tracing::trace!(
        "Position: {:?}, Min element: {:?}",
        guard.position,
        guard.position.min_element()
    );
    match guard.direction {
        GuardDirection::North => guard.position.y >= bounds.y,
        GuardDirection::East => guard.position.x >= bounds.x,
        GuardDirection::South => guard.position.y == 0,
        GuardDirection::West => guard.position.x == 0,
    }
}

pub fn get_next_position(guard: &GuardLocation) -> UVec2 {
    match guard.direction {
        GuardDirection::North => UVec2::new(guard.position.x, guard.position.y + 1),
        GuardDirection::South => UVec2::new(guard.position.x, guard.position.y - 1),
        GuardDirection::East => UVec2::new(guard.position.x + 1, guard.position.y),
        GuardDirection::West => UVec2::new(guard.position.x - 1, guard.position.y),
    }
}

#[tracing::instrument]
fn parse_row(row: usize, input: &str) -> Vec<Entity> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = UVec2::from((column as u32, row as u32));
            let res = match ch {
                '#' => Some(Entity::Obstruction(pos)),
                '^' => Some(Entity::Guard(pos)),
                '.' => Some(Entity::None(pos)),
                _ => unreachable!("Parser missed possible input {:?}", input),
            };
            tracing::trace!("Found: {:?}", &res);
            res
        })
        .collect()
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Entity {
    Obstruction(UVec2),
    Guard(UVec2),
    None(UVec2),
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuardLocation {
    pub position: UVec2,
    pub direction: GuardDirection,
}

impl GuardLocation {
    fn new(x: u32, y: u32) -> GuardLocation {
        GuardLocation {
            position: UVec2::from((x, y)),
            direction: GuardDirection::North,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GuardDirection {
    #[default]
    North,
    South,
    East,
    West,
}

impl GuardDirection {
    #[tracing::instrument]
    pub fn pivot(&self) -> Self {
        use GuardDirection::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
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
    fn test_process_simple_single_exit() -> miette::Result<()> {
        let input = "
....#.....
..........
....^.....
";
        // Should move up 1 step and then 5 to the right then done
        assert_eq!("7", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_empty_row() -> miette::Result<()> {
        let input = "..........";
        let expected: Vec<Entity> = vec![];
        assert_eq!(
            expected,
            parse_row(0, input)
                .into_iter()
                .filter(|entity| { !matches!(entity, Entity::None(_)) })
                .collect::<Vec<Entity>>()
        );
        Ok(())
    }

    #[test]
    fn test_parse_guard_position() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(UVec2::new(4, 0))];
        assert_eq!(
            expected,
            parse_row(0, input)
                .into_iter()
                .filter(|entity| { !matches!(entity, Entity::None(_)) })
                .collect::<Vec<Entity>>()
        );
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_non_zero_row() -> miette::Result<()> {
        let input = "....^.....";
        let expected: Vec<Entity> = vec![Entity::Guard(UVec2::new(4, 7))];
        assert_eq!(
            expected,
            parse_row(7, input)
                .into_iter()
                .filter(|entity| { !matches!(entity, Entity::None(_)) })
                .collect::<Vec<Entity>>()
        );
        Ok(())
    }

    #[test]
    fn test_parse_guard_position_and_obstruction() -> miette::Result<()> {
        let input = "....^...#.";
        let expected: Vec<Entity> = vec![
            Entity::Guard(UVec2::new(4, 0)),
            Entity::Obstruction(UVec2::new(8, 0)),
        ];
        assert_eq!(
            expected,
            parse_row(0, input)
                .into_iter()
                .filter(|entity| { !matches!(entity, Entity::None(_)) })
                .collect::<Vec<Entity>>()
        );
        Ok(())
    }

    #[test]
    fn test_exiting_bounds_returns_true_when_zero_bounds_reached_and_direction_south(
    ) -> miette::Result<()> {
        let bounds = UVec2::new(5, 5);
        let mut guard = GuardLocation::new(1, 0);
        guard.direction = GuardDirection::South;
        assert!(is_exiting_bounds(&guard, &bounds));
        Ok(())
    }

    #[test]
    fn test_exiting_bounds_returns_false_when_zero_bounds_reached_and_direction_not_south(
    ) -> miette::Result<()> {
        let bounds = UVec2::new(5, 5);
        let mut guard = GuardLocation::new(1, 0);
        guard.direction = GuardDirection::West;
        assert!(!is_exiting_bounds(&guard, &bounds));
        Ok(())
    }
}
