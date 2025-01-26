use std::collections::HashMap;

use tracing::trace;

type Point = (i32, i32);

const OFFSETS: [[Point; 2]; 4] = [
    [(1, 1), (-1, -1)],
    [(1, -1), (-1, 1)],
    [(-1, -1), (1, 1)],
    [(-1, 1), (1, -1)],
];

pub fn process(input: &str) -> miette::Result<String> {
    let matrix_map: HashMap<Point, char> = HashMap::from_iter(
        input
            .lines()
            .skip_while(|line| line.is_empty())
            .enumerate()
            .flat_map(|(row_index, line)| {
                line.chars().enumerate().map(move |(column_index, letter)| {
                    ((column_index as i32, row_index as i32), letter)
                })
            })
            .collect::<Vec<(Point, char)>>(),
    );

    let other_letters: Vec<char> = "MS".chars().collect();
    let total_hits: usize = matrix_map
        .iter()
        .filter(|(_position, value)| **value == 'A')
        .filter(|(position, _value)| {
            trace!("Found {} at position {:?}", _value, position,);
            OFFSETS
                .iter()
                .map(|point_offsets| {
                    point_offsets
                        .iter()
                        .map(|offset| {
                            matrix_map.get(&(position.0 + offset.0, position.1 + offset.1))
                        })
                        .enumerate()
                        .all(|(index, value)| other_letters.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();

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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
