#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let _ = input.lines();
    todo!("{{crate_name}} - part 1");
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
