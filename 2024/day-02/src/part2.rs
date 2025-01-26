use crate::part1;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let n: usize = input
        .lines()
        .map(|line| {
            tracing::debug!("Line: {:?}", line);
            let nums: Vec<isize> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            if part1::is_line_safe(nums.clone()) {
                tracing::trace!("Line Safe: {:?}", line);
                return 1;
            }

            let mut has_save_variant = 0;
            for index in 0..nums.len() {
                let mut removed_one = nums.clone();
                removed_one.remove(index);
                tracing::trace!("Testing Line Variant: {:?}", removed_one.clone());
                if part1::is_line_safe(removed_one) {
                    tracing::trace!("Safe Line Variant");
                    has_save_variant += 1;
                    break; // This is critical as we only want to count one safe variant per line
                }
            }
            has_save_variant
        })
        .sum();

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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
