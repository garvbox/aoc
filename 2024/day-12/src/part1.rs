use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};
use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let plants: HashMap<IVec2, char> = input
        .lines()
        .rev()
        .skip_while(|line| line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            tracing::trace!("Line {row} - {line:?}");
            line.chars()
                .enumerate()
                .map(move |(col, ch)| (IVec2::new(col as i32, row as i32), ch))
        })
        .collect();
    tracing::debug!("Plants: {plants:?}");

    let mut graph: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

    for (position, letter) in plants.iter() {
        let node = graph.add_node((position.x, position.y));

        for direction in DIRECTIONS.iter() {
            let next_position = position + direction;
            if plants
                .get(&next_position)
                .is_some_and(|next_letter| letter == next_letter)
            {
                graph.add_edge(node, (next_position.x, next_position.y), ());
                tracing::trace!(
                    "Found graph edge for region '{letter}': {:?}->{:?}",
                    position,
                    next_position
                );
            }
        }
    }

    let condensed = condensation(graph.clone().into_graph::<NodeIndex>(), false);
    let result: usize = condensed
        .node_references()
        .map(|(_index, nodes)| {
            let perimeter: usize = nodes.iter().map(|n| 4 - graph.neighbors(*n).count()).sum();
            perimeter * nodes.len()
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        assert_eq!("140", process(input)?);
        Ok(())
    }
}
