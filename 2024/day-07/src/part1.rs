use itertools::Itertools;
use nom::{bytes, character, multi, sequence, IResult};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let result: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (_discard, equation) = parse(line).unwrap();
            let ops = vec![Operator::Add, Operator::Multiply];
            let num_operators = equation.numbers.len() - 1;
            tracing::info!(
                "Line Parse result: {:?}, from input {:?}, num operators: {:?}",
                equation,
                line,
                num_operators
            );

            let solution = (0..num_operators)
                .map(|_| ops.clone())
                .multi_cartesian_product()
                .find(|operators| {
                    tracing::debug!("Trying operator permutation: {:?}", operators);
                    solve(&equation, operators)
                });

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

fn parse(input: &str) -> IResult<&str, Equation> {
    let (_remaining, pair) = sequence::separated_pair(
        character::complete::u64,
        bytes::complete::tag(": "),
        multi::separated_list0(character::complete::space1, character::complete::u64),
    )(input)?;
    tracing::trace!("Parse Result: {:?}", pair);
    Ok((
        input,
        Equation {
            test_value: pair.0,
            numbers: pair.1,
        },
    ))
}

#[tracing::instrument]
fn solve(equation: &Equation, operators: &Vec<Operator>) -> bool {
    let mut collector = equation.numbers[0];
    for (index, number) in equation.numbers[1..].iter().enumerate() {
        let operator = operators.get(index).unwrap();
        tracing::trace!("Collector: {:?} -> {:?} {:?}", collector, operator, number);
        match operator {
            Operator::Add => collector += number,
            Operator::Multiply => collector *= number,
        }
        if collector > equation.test_value {
            tracing::trace!("Collector Exceeded test value - breaking: {:?}", collector);
            break;
        }
    }
    tracing::trace!(
        "Result -> Collector {:?}, operators {:?}, test value: {:?}",
        collector,
        operators,
        equation.test_value
    );
    collector == equation.test_value
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
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
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
