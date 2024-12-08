#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day_05 - part 2");
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
