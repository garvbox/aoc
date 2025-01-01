use std::collections::HashMap;

use glam::IVec2;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut char_map = HashMap::new();
    for (row, line) in input
        .lines()
        .rev() // NOTE: Input is processed row by row in reverse so that X-Y coords make sense
        .skip_while(|line| line.is_empty())
        .enumerate()
    {
        for antenna in parse_row(row, line) {
            char_map
                .entry(antenna.character)
                .or_insert(Vec::new())
                .push(antenna.position);
        }
    }

    let max_x = input
        .lines()
        .skip_while(|line| line.is_empty())
        .next()
        .unwrap()
        .len() as i32;
    let max_y = input.lines().skip_while(|line| line.is_empty()).count() as i32;
    let bounds = IVec2::new(max_x, max_y);

    // TODO: Collect all possible pairs of antennae (iterools.combinations?) and work out antinodes
    // for each in a HashSet

    Ok("".to_owned())
}

fn in_bounds(position: &IVec2, bound: &IVec2) -> bool {
    position.cmple(*bound).all() && position.cmpge(IVec2::new(0, 0)).all()
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Antenna {
    character: char,
    position: IVec2,
}

#[tracing::instrument]
fn parse_row(row: usize, input: &str) -> Vec<Antenna> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = IVec2::from((column as i32, row as i32));
            let res = match ch {
                '.' => None,
                _other => {
                    tracing::trace!("Found: {:?} at {:?}", &_other, &pos);
                    Some(Antenna {
                        character: _other,
                        position: pos,
                    })
                }
            };
            res
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!("14", process(input)?);
        Ok(())
    }

    #[test]
    fn test_in_bounds_true() -> miette::Result<()> {
        let bounds = IVec2::new(2, 2);
        for position in [(0, 0), (0, 1), (2, 2)] {
            tracing::trace!("Checking position: {:?} in bounds {:?}", position, bounds);
            assert!(in_bounds(&IVec2::new(position.0, position.1), &bounds));
        }
        Ok(())
    }

    #[test]
    fn test_in_bounds_false() -> miette::Result<()> {
        let bounds = IVec2::new(2, 2);
        for position in [(-1, 0), (0, -1), (3, 2), (2, 3), (3, 3)] {
            tracing::trace!(
                "Checking position: {:?} not in bounds {:?}",
                position,
                bounds
            );
            assert!(!in_bounds(&IVec2::new(position.0, position.1), &bounds));
        }
        Ok(())
    }
}
