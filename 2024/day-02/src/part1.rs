// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut n = 0;
    let bounds = 1..=3;

    for line in input.lines() {
        tracing::trace!("Line: '{}'", line);
        let nums: Vec<isize> = line
            .split_whitespace()
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect();

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

            if !bounds.contains(&diff) {
                tracing::trace!("Unsafe diff: {diff}");
                line_safe = false;
                break;
            }

            last_num = Some(num);
        }
        if line_safe {
            tracing::trace!("Line Safe");
            n += 1;
        }
    }

    Ok(n.to_string())
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
