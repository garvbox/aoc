use itertools::Itertools;
use miette::{miette, Result};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut files: Vec<Option<u32>> = vec![];

    for (file_index, (file_size, free_space)) in input.chars().tuple_windows().enumerate() {
        let file_size: u32 = file_size
            .to_digit(10)
            .ok_or(miette!("Invalid file size, got {:?}", file_size))?;
        let free_space: u32 = free_space
            .to_digit(10)
            .ok_or(miette!("Invalid free space, got {:?}", file_size))?;

        tracing::trace!("File: {file_size}, Space: {free_space}");
        files.extend([Some(file_index as u32)].repeat(file_size as usize));
        files.extend([None].repeat(free_space as usize));
    }

    // Construct formatted output for printing
    let disk_layout: String = files
        .iter()
        .map(|el| match el {
            Some(item) => "|".to_owned() + &item.to_string(),
            None => "|.".to_string(),
        })
        .collect();
    tracing::info!("Disk Map:\n{}", disk_layout);
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
