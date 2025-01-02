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
    print_file_map(&files);

    // TODO: The below loops over files and tries to keep track as we go, but what we should
    // actually be doing is just picking up the file indexes first and repeatedly re-processing the
    // same list of objects for movement as it would make it a lot easier to keep track of things
    // than the nested iterators to find gaps and movement candidates

    let file_ids: Vec<u64> = files.clone().iter().filter_map(|f| f.id).rev().collect();

    for file_id in file_ids {
        files = defrag_move_file(file_id, files);
        print_file_map(&files);
    }
    print_file_map(&files);

    let checksum: u64 = files
        .iter()
        .map(|file| match file.id {
            Some(id) => (file.position..file.position + file.size as usize)
                .map(|pos| pos as u64 * id)
                .sum(),
            None => 0,
        })
        .sum();

    Ok(checksum.to_string())
}

fn defrag_move_file(file_id: u64, files: Vec<File>) -> Vec<File> {
    // New copies of the file and gap for update
    let mut file = files
        .iter()
        .find(|f| f.id == Some(file_id))
        .expect("Missing file id")
        .clone();
    let gap = files
        .iter()
        .find(|f| f.id.is_none() && f.size >= file.size && f.position < file.position);

    let mut gap = match gap {
        Some(gap) => gap.clone(),
        None => {
            tracing::debug!("No gap found for file {:?}", file);
            return files.clone();
        }
    };

    // Moving file to a new location will leave a gap
    let new_file_gap = File {
        id: None,
        size: file.size,
        position: file.position,
    };

    // Update the file to new position
    let previous_position = file.position;
    file.position = gap.position;
    tracing::trace!(
        "Updated file {:?} position from {:?} to {:?}",
        file_id,
        previous_position,
        file.position
    );

    // Drop gap size and move as needed
    if gap.size == file.size {
        gap.size = 0;
        tracing::trace!("Dropped Gap at position {:?}", gap.position);
    } else {
        gap.size -= file.size;
        gap.position = file.position + file.size as usize;
        tracing::trace!(
            "Moved Gap to position {:?}, size to {:?}",
            gap.position,
            gap.size
        );
    }

    let mut result: Vec<File> = files
        .iter()
        .filter_map(|existing_file| {
            if existing_file.position == file.position || existing_file.id == file.id {
                None
            } else {
                Some(existing_file.clone())
            }
        })
        .collect();

    result.push(file);
    result.push(new_file_gap);
    if gap.size > 0 {
        result.push(gap);
    }

    result.into_iter().sorted_by_key(|f| f.position).collect()
}

fn print_file_map(files: &[File]) -> String {
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

    tracing::trace!("Disk Map:\t{}", disk_map);
    disk_map
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
