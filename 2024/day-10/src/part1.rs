use glam::IVec2;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let points: HashMap<IVec2, u32> = input
        .lines()
        .rev()
        .enumerate()
        .flat_map(move |(row_index, row)| {
            row.chars().enumerate().filter_map(move |(col_index, col)| {
                let height = match col {
                    '.' => return None,
                    height => height,
                };
                Some((
                    IVec2::new(col_index as i32, row_index as i32),
                    height.to_string().parse().unwrap(),
                ))
            })
        })
        .collect();

    let trailheads: Vec<&IVec2> = points
        .iter()
        .filter_map(
            |(point, height)| {
                if *height == 0 {
                    Some(point)
                } else {
                    None
                }
            },
        )
        .collect();

    let mut score: u32 = 0;
    for start_position in trailheads.iter() {
        tracing::debug!("Starting trace of trail: {:?}", start_position);
        let mut queue = VecDeque::<IVec2>::new();
        let mut trail_ends = HashSet::<IVec2>::new();

        queue.push_back(**start_position);

        while let Some(position) = queue.pop_front() {
            let height = points.get(&position).unwrap();

            // Explore all possible directions and add paths
            for dir in DIRECTIONS.iter() {
                let next_position = position + dir;
                let next_height = match points.get(&next_position) {
                    Some(height) => height,
                    None => continue,
                };
                if *height == 8 && *next_height == 9 {
                    if trail_ends.contains(&next_position) {
                        tracing::trace!(
                            "Already seen trail end: {:?} -> {:?}",
                            position,
                            next_position
                        );
                    } else {
                        tracing::trace!("Found Trail end: {:?} -> {:?}", position, next_position);
                        trail_ends.insert(next_position);
                        score += 1;
                    }
                } else if *next_height == height + 1 {
                    tracing::trace!("Tracking path: {:?} -> {:?}", position, next_position);
                    queue.push_front(next_position);
                }
            }
        }
    }

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process_single_trail() -> miette::Result<()> {
        let input = "
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_single_trail_four_path() -> miette::Result<()> {
        let input = "
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_multi_trailhead() -> miette::Result<()> {
        let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
