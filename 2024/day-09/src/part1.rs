use itertools::Itertools;
use miette::{miette, Result};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let mut files: Vec<Option<u32>> = vec![];
    for (file_index, mut chunk) in (&input.chars().chunks(2)).into_iter().enumerate() {
        let file_size: u32 = chunk
            .next()
            .ok_or(miette!("Invalid Input chunk"))?
            .to_digit(10)
            .ok_or(miette!("Invalid file size"))?;

        let free_space: u32 = chunk
            .next()
            .unwrap_or('0') // Last file in a sequence may or may not have free space after
            .to_digit(10)
            .ok_or(miette!("Invalid free space"))?;

        tracing::trace!(
            "File index: {file_index}, File size: {file_size}, Free space: {free_space}"
        );
        files.extend([Some(file_index as u32)].repeat(file_size as usize));
        files.extend([None].repeat(free_space as usize));
    }

    // Construct formatted output for printing
    let disk_layout: String = files
        .iter()
        .map(|el| match el {
            Some(file) => file.to_string(),
            None => ".".to_string(),
        })
        .collect();
    tracing::info!("Original Disk Map:\n{}", disk_layout);
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
