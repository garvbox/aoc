const BOUNDS: std::ops::RangeInclusive<isize> = 1..=3;

pub fn process(input: &str) -> miette::Result<String> {
    let n: usize = input
        .lines()
        .map(|line| {
            tracing::trace!("Line: '{}'", line);
            let nums: Vec<isize> = line
                .split_whitespace()
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect();

            if is_line_safe(nums) {
                tracing::trace!("Line Safe");
                1
            } else {
                0
            }
        })
        .sum();

    Ok(n.to_string())
}

pub fn is_line_safe(nums: Vec<isize>) -> bool {
    let mut is_increasing: Option<bool> = None;
    let mut last_num: Option<isize> = None;
    let mut line_safe: bool = true;

    for num in nums {
        if last_num.is_none() {
            last_num = Some(num);
            continue;
        }

        let current_last_num = last_num.unwrap();
        let mut diff = num - current_last_num;
        tracing::trace!("Num: {num}, Last Num: {current_last_num}, Diff: {diff}",);

        if diff == 0 {
            tracing::trace!("Zero diff - line unsafe");
            line_safe = false;
            break;
        }

        let increasing = is_increasing.get_or_insert(num > current_last_num);
        if !*increasing {
            tracing::trace!("Is decreasing - flipped diff: {diff}");
            diff = -diff;
        }

        if !BOUNDS.contains(&diff) {
            tracing::trace!("Unsafe diff: {diff}");
            line_safe = false;
            break;
        }

        last_num = Some(num);
    }
    line_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn test_process_with_initial_diff() -> miette::Result<()> {
        let input = "12 6 4 2 1";
        assert_eq!("0", process(input)?);
        Ok(())
    }
}

