use miette::{IntoDiagnostic, Result};
use std::iter::zip;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut lefts: Vec<u32> = vec![];
    let mut rights: Vec<u32> = vec![];

    for line in input.lines() {
        tracing::trace!("line: '{}'", line);
        let mut split_line = line.split_whitespace();
        lefts.push(split_line.next().unwrap().parse().into_diagnostic()?);
        rights.push(split_line.next().unwrap().parse().into_diagnostic()?);
    }

    lefts.sort();
    rights.sort();

    let distance: u32 = zip(lefts, rights).map(|(l, r)| l.abs_diff(r)).sum();
    Ok(distance.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
