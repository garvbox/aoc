use itertools::Itertools;

use crate::part1::{parse, Operator};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let result: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (_discard, equation) = parse(line).unwrap();
            // NOTE: This is the main difference from part1, the extra operator in the list
            // introduces this as an option for solver attempts
            let ops = vec![Operator::Add, Operator::Multiply, Operator::Concat];

            let solution = (0..equation.numbers.len() - 1)
                .map(|_| ops.clone())
                .multi_cartesian_product()
                .find(|operators| crate::part1::solve(&equation, operators));

            if solution.is_some() {
                tracing::info!(
                    "Solution Found: {:?} -> Adding {:?}",
                    solution,
                    equation.test_value
                );
                Some(equation.test_value)
            } else {
                tracing::warn!("No solutions found for: {:?}", equation,);
                None
            }
        })
        .sum();
    tracing::info!("Result: {:?}", result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test(ignore)]
    fn test_process() -> miette::Result<()> {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!("11387", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_line3() -> miette::Result<()> {
        let input = "7290: 6 8 6 15";
        assert_eq!("7290", process(input)?);
        Ok(())
    }
}
