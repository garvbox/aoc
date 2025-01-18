#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let _ = input.lines();
    todo!("day_11 - part 1");
}

fn blink(stones: Vec<String>) -> Vec<String> {
    stones.iter().fold(Vec::new(), |mut acc, stone| {
        let stone = stone.as_str();
        if stone == "0" {
            acc.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let (left, right) = stone.split_at(stone.len() / 2);
            acc.push(left.parse::<u64>().unwrap().to_string());
            acc.push(right.parse::<u64>().unwrap().to_string());
        } else {
            acc.push((&stone.parse::<u64>().unwrap() * 2024).to_string());
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_one_blink() -> miette::Result<()> {
        let input = "0 1 10 99 999"
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(
            "1 2024 1 0 9 9 2021976".split(" ").collect::<Vec<_>>(),
            blink(input)
        );
        Ok(())
    }
}
