use itertools::Itertools;
use miette::{miette, Result};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let input_line = input.chars().filter(|ch| *ch != '\n');
    let mut files: Vec<Option<u32>> = vec![];

    for (file_index, mut chunk) in (&input_line.chunks(2)).into_iter().enumerate() {
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

    let mut compacted_files: Vec<u32> = vec![];
    let mut index = 0;
    let mut num_popped = 0;
    let disk_size = files.iter().filter(|el| el.is_some()).count();

    while index < disk_size {
        tracing::trace!("Working on index: {:?}, num moved {:?}", index, num_popped);
        match files[index] {
            Some(block) => compacted_files.push(block),
            None => {
                if let Some(block_to_move) = loop {
                    let candidate = files.pop().expect("Ran out of file blocks");
                    num_popped += 1;
                    tracing::trace!("Getting next block: {:?}", candidate);
                    match candidate {
                        Some(block) => {
                            tracing::trace!("Found move candidate: {:?}", block);
                            break Some(block);
                        }
                        None => {
                            tracing::trace!("Found no move candidate");
                        }
                    }
                } {
                    tracing::trace!("Moving Block: {block_to_move}");
                    compacted_files.push(block_to_move);
                }
            }
        }
        index += 1;
    }

    let disk_layout_compacted: String = compacted_files
        .iter()
        .map(|el| "|".to_owned() + &el.to_string())
        .collect();
    tracing::info!("Compacted Disk Map:\n{}", disk_layout_compacted);

    let checksum: u64 = compacted_files
        .iter()
        .enumerate()
        .map(|(index, file_index)| index as u64 * *file_index as u64)
        .sum();
    Ok(checksum.to_string())
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
