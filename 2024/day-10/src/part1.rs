use glam::IVec3;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let points: Vec<IVec3> = input
        .lines()
        .rev()
        .enumerate()
        .flat_map(move |(row_index, row)| {
            row.chars().enumerate().filter_map(move |(col_index, col)| {
                let height = match col {
                    '.' => return None,
                    height => height,
                };
                Some(IVec3::new(
                    col_index as i32,
                    row_index as i32,
                    height.to_string().parse().unwrap(),
                ))
            })
        })
        .collect();

    tracing::trace!("Parsed Points: {:?}", points);
    Ok("".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
