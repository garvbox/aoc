use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str, count: usize) -> miette::Result<String> {
    let mut stones: HashMap<u64, usize> = input
        .split_whitespace()
        .map(|stone| (stone.parse().unwrap(), 1))
        .collect();

    for _ in 0..count {
        stones = blink(stones);
    }

    Ok(stones.values().sum::<usize>().to_string())
}

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    tracing::info!("blinking: {stones:?}");
    let mut new_stones: HashMap<u64, usize> = HashMap::new();

    for (stone, count) in stones.iter() {
        let stone_name = stone.to_string();
        if stone_name == "0" {
            *new_stones.entry(1).or_insert(0) += count;
        } else if stone_name.len() % 2 == 0 {
            let (left, right) = stone_name.split_at(stone_name.len() / 2);
            *new_stones.entry(left.parse::<u64>().unwrap()).or_insert(0) += count;
            *new_stones.entry(right.parse::<u64>().unwrap()).or_insert(0) += count;
        } else {
            *new_stones.entry(stone * 2024).or_insert(0) += count;
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process_6() -> miette::Result<()> {
        assert_eq!("22".to_string(), process("125 17", 6).unwrap());
        Ok(())
    }

    #[test]
    fn test_process_25() -> miette::Result<()> {
        assert_eq!("55312".to_string(), process("125 17", 25).unwrap());
        Ok(())
    }
}
