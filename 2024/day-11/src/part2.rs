use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, blinks: usize) -> miette::Result<String> {
    let mut stones: HashMap<u64, usize> = input
        .split_whitespace()
        .map(|stone| (stone.parse().unwrap(), 1 as usize))
        .collect();

    for _ in 0..blinks {}

    Ok(stones.values().sum().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("55312".to_string(), process("125 17", 6).unwrap());
        Ok(())
    }
}
