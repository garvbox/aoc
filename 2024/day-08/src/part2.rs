use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::part1::in_bounds;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut char_map = HashMap::new();
    for (row, line) in input
        .lines()
        .rev() // NOTE: Input is processed row by row in reverse so that X-Y coords make sense
        .skip_while(|line| line.is_empty())
        .enumerate()
    {
        for antenna in crate::part1::parse_row(row, line) {
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

    let antinodes: HashSet<IVec2> = char_map
        .iter()
        .flat_map(|(character, antennae)| {
            antennae.iter().combinations(2).flat_map(move |combo| {
                tracing::trace!("Antenna combination: {:?} for {:?}", &combo, &character);
                let mut nodes = vec![*combo[0], *combo[1]];
                nodes.extend(get_resonant_nodes(combo[0], combo[1], &bounds));
                nodes.extend(get_resonant_nodes(combo[1], combo[0], &bounds));
                nodes
            })
        })
        .collect();
    tracing::info!("Antinodes Found: {:?}", &antinodes);

    // NOTE: The below is just gathering output for debugging
    let antenna_positions: HashMap<IVec2, char> = HashMap::from_iter(
        char_map
            .iter()
            .flat_map(|(character, antennae)| antennae.iter().map(|antenna| (*antenna, *character)))
            .collect::<Vec<(IVec2, char)>>(),
    );
    let mut output: String = String::new();
    for row in (0..max_y + 1).rev() {
        tracing::trace!("Row: {:?}", &row);
        for col in 0..max_x + 1 {
            let pos = IVec2::new(col, row);
            if antenna_positions.contains_key(&pos) {
                output.push(*antenna_positions.get(&pos).unwrap())
            } else if antinodes.contains(&pos) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    tracing::debug!("Collected Positions: \n{}", output);

    Ok(antinodes.len().to_string())
}

fn get_resonant_nodes(lhs: &IVec2, rhs: &IVec2, bounds: &IVec2) -> Vec<IVec2> {
    let diff = rhs - lhs;
    tracing::trace!("Antenna diff: {:?}", &diff);
    let mut candidate = rhs + diff;
    let mut antinodes = Vec::new();
    while in_bounds(&candidate, bounds) {
        tracing::trace!("Antinode found: {:?}", &candidate);
        antinodes.push(candidate);
        candidate += diff;
    }
    antinodes
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
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
