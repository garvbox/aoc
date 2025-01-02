use itertools::Itertools;
use miette::miette;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let input_line = input.chars().filter(|ch| *ch != '\n');
    let mut files: Vec<File> = vec![];

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
        files.push(File {
            index: Some(file_index as u32),
            size: file_size,
        });
        files.push(File {
            index: None,
            size: free_space,
        });
    }

    // Construct formatted output for printing
    let disk_layout: String = files
        .iter()
        .map(|file| {
            let char = match file.index {
                Some(index) => index.to_string(),
                None => ".".to_string(),
            };
            char.repeat(file.size as usize)
        })
        .collect();
    tracing::info!("Original Disk Map:\n{}", disk_layout);

    Ok("".to_string())
}

struct File {
    index: Option<u32>, // None here means a Gap
    size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
