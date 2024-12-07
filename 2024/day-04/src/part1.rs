use std::{collections::HashMap, usize};

use tracing::trace;

type Point = (i32, i32);

const OFFSETS: [[Point; 3]; 8] = [
    [(1, 0), (2, 0), (3, 0)],
    [(-1, 0), (-2, 0), (-3, 0)],
    [(0, 1), (0, 2), (0, 3)],
    [(0, -1), (0, -2), (0, -3)],
    [(1, 1), (2, 2), (3, 3)],
    [(-1, -1), (-2, -2), (-3, -3)],
    [(1, -1), (2, -2), (3, -3)],
    [(-1, 1), (-2, 2), (-3, 3)],
];

pub fn process(input: &str) -> miette::Result<String> {
    let matrix_map: HashMap<Point, char> = HashMap::from_iter(
        input
            .lines()
            .skip_while(|line| line.is_empty())
            .enumerate()
            .flat_map(|(row_index, line)| {
                line.chars().enumerate().map(move |(column_index, letter)| {
                    (
                        (column_index.clone() as i32, row_index.clone() as i32),
                        letter.clone(),
                    )
                })
            })
            .collect::<Vec<(Point, char)>>(),
    );

    let other_letters: Vec<char> = "MAS".chars().collect();
    let total_hits: usize = matrix_map
        .iter()
        .filter(|(_position, value)| **value == 'X')
        .map(|(position, _value)| {
            trace!("Found {} at position {:?}", _value, position,);
            OFFSETS
                .iter()
                .map(|offset_points| {
                    offset_points
                        .iter()
                        .map(|offset| {
                            matrix_map.get(&(position.0 + offset.0, position.1 + offset.1))
                        })
                        .enumerate()
                        .all(|(index, value)| other_letters.get(index) == value)
                })
                .filter(|b| *b)
                .count()
        })
        .sum();

    Ok(total_hits.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
