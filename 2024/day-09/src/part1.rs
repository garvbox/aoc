#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let _ = input.lines();
    todo!("day_09 - part 1");
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
