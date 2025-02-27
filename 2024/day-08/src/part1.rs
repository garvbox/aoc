use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

    let max_x = input.lines().find(|line| !line.is_empty()).unwrap().len() as i32 - 1;
    let max_y = input.lines().skip_while(|line| line.is_empty()).count() as i32 - 1;
    let bounds = IVec2::new(max_x, max_y);
    tracing::debug!("Detected Bounds: {:?}", &bounds);

    let mut antinodes: HashSet<IVec2> = HashSet::new();
    for (character, antennae) in char_map.iter() {
        for combo in antennae.iter().combinations(2) {
            tracing::trace!("Antenna combination: {:?} for {:?}", &combo, &character);
            for antinode in [
                get_antinode(combo[0], combo[1]),
                get_antinode(combo[1], combo[0]),
            ] {
                if in_bounds(&antinode, &bounds) {
                    tracing::debug!("Antinode Found: {:?}", antinode);
                    antinodes.insert(antinode);
                } else {
                    tracing::trace!("Antinode Out of Bounds: {:?}", antinode);
                }
            }
        }
    }

    tracing::info!("Antinodes Found: {:?}", &antinodes);
    Ok(antinodes.len().to_string())
}

pub(crate) fn in_bounds(position: &IVec2, bound: &IVec2) -> bool {
    position.cmple(*bound).all() && position.cmpge(IVec2::new(0, 0)).all()
}

fn get_antinode(lhs: &IVec2, rhs: &IVec2) -> IVec2 {
    IVec2::new(rhs.x + (rhs.x - lhs.x), rhs.y + (rhs.y - lhs.y))
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub(crate) struct Antenna {
    pub character: char,
    pub position: IVec2,
}

#[tracing::instrument]
pub(crate) fn parse_row(row: usize, input: &str) -> Vec<Antenna> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = IVec2::from((column as i32, row as i32));
            match ch {
                '.' => None,
                _other => {
                    tracing::trace!("Found: {:?} at {:?}", &_other, &pos);
                    Some(Antenna {
                        character: _other,
                        position: pos,
                    })
                }
            }
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
