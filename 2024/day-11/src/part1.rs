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

    #[test]
    fn test_six_blink_example() -> miette::Result<()> {
        let input = "125 17"
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let first = blink(input);
        assert_eq!("253000 1 7".split(" ").collect::<Vec<_>>(), first);

        let second = blink(first);
        assert_eq!("253 0 2024 14168".split(" ").collect::<Vec<_>>(), second);

        let mut last = second.clone();
        for _ in 0..4 {
            // Blink called twice, we want to get to 6
            last = blink(last);
        }
        assert_eq!(
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
                .split(" ")
                .collect::<Vec<_>>(),
            last
        );
        assert!(last.len() == 22);

        Ok(())
    }
}
