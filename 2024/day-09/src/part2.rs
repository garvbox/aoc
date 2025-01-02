use itertools::Itertools;
use miette::miette;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let input_line = input.chars().filter(|ch| *ch != '\n');
    let mut files: Vec<File> = vec![];
    let mut file_position: usize = 0;

    for (file_id, mut chunk) in (&input_line.chunks(2)).into_iter().enumerate() {
        let file_size = chunk
            .next()
            .ok_or(miette!("Invalid Input chunk"))?
            .to_digit(10)
            .ok_or(miette!("Invalid file size"))?;

        let free_space = chunk
            .next()
            .unwrap_or('0') // Last file in a sequence may or may not have free space after
            .to_digit(10)
            .ok_or(miette!("Invalid free space"))?;

        tracing::trace!("File index: {file_id}, File size: {file_size}, Free space: {free_space}");
        files.push(File {
            id: Some(file_id as u64),
            size: file_size as u64,
            position: file_position,
        });
        file_position += file_size as usize;

        files.push(File {
            id: None,
            size: free_space as u64,
            position: file_position,
        });
        file_position += free_space as usize;
    }
    print_file_map(files);
}

fn print_file_map(files: Vec<File>) {
    let disk_map: String = files
        .iter()
        .map(|file| {
            let char = match file.id {
                Some(index) => index.to_string(),
                None => ".".to_string(),
            };
            char.repeat(file.size as usize)
        })
        .collect();
    tracing::info!("Original Disk Map:\n{}", disk_layout);

    Ok("".to_string())
    println!("Disk Map:\n{}", disk_map);
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct File {
    id: Option<u64>, // None here means a Gap
    size: u64,
    position: usize,
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
