use std::collections::HashMap;

use glam::UVec2;

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

    // TODO: Collect all possible pairs of antennae (iterools.combinations?) and work out antinodes
    // for each in a HashSet

    Ok("".to_owned())
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Antenna {
    character: char,
    position: UVec2,
}

#[tracing::instrument]
fn parse_row(row: usize, input: &str) -> Vec<Antenna> {
    input
        .chars()
        .enumerate()
        .filter_map(|(column, ch)| {
            let pos = UVec2::from((column as u32, row as u32));
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
}
