use std::collections::HashSet;

use glam::UVec2;

use crate::part1;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (bounds, original_guard, obstructions) = part1::parse_input(input);
    let mut visited_positions: HashSet<UVec2> = HashSet::from([original_guard.position]);

    // Collect all guard positions on the original path - similar to part1 but with directions
    // included also
    let mut guard = original_guard.clone();
    while !part1::is_exiting_bounds(&guard, &bounds) {
        let next_position = part1::get_next_position(&guard);

        if obstructions.contains(&next_position) {
            guard.direction = guard.direction.pivot();
        } else {
            guard.position = next_position;
            visited_positions.insert(guard.position);
        }
    }

    // For each visited position by the guard, check if adding an obstruction in the next
    // position would put the guard in a loop. This can be checked by moving the guard in the
    // standard fashion and cheching if it crosses the set
    let results = visited_positions
        .clone()
        .iter()
        .filter(|obstruction_candidate| {
            let mut guard = original_guard.clone();
            tracing::info!(
                "Assessing candidate {:?}, starting pos {:?}",
                obstruction_candidate,
                guard
            );

            // Original guard position is not allowed
            if obstruction_candidate == &&guard.position {
                return false;
            }

            let mut guard_locations: HashSet<part1::GuardLocation> = HashSet::from([guard.clone()]);

            loop {
                // Standard Guard Movement, including new obstruction candidate position
                let next_position = part1::get_next_position(&guard);
                if obstructions.contains(&next_position)
                    || (&&next_position == obstruction_candidate)
                {
                    guard.direction = guard.direction.pivot();
                    continue;
                } else {
                    guard.position = next_position;
                }

                // After moving the guard to the next position, check if we are on a previous
                // position and vector
                if guard_locations.contains(&guard) {
                    tracing::debug!(
                        "Found Loop: {:?}, guard pos {:?}",
                        obstruction_candidate,
                        guard
                    );
                    break true;
                }

                if part1::is_exiting_bounds(&guard, &bounds) {
                    tracing::debug!(
                        "Hit bounds at {:?}, cannot be a Loop: {:?}",
                        guard,
                        obstruction_candidate,
                    );
                    // Exited bounds - cannot be a loop
                    break false;
                }

                guard_locations.insert(guard.clone());
            }
        })
        .count();

    Ok(results.to_string())
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
