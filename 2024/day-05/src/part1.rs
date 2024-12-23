use std::collections::{HashMap, HashSet};

use tracing::trace;

pub fn process(input: &str) -> miette::Result<String> {
    // NOTE: Rules is a map of entries to collections of elements which should be to the left or
    // right of that element for quick lookup
    let mut rules: HashMap<usize, (HashSet<usize>, HashSet<usize>)> = HashMap::new();

    for line in input.lines().filter(|l| l.contains("|")) {
        let mut nums = line.split("|");
        let left = nums.next().unwrap().parse().unwrap();
        let right = nums.next().unwrap().parse().unwrap();
        rules
            .entry(left)
            .or_insert((HashSet::new(), HashSet::new()))
            .1
            .insert(right);

        rules
            .entry(right)
            .or_insert((HashSet::new(), HashSet::new()))
            .0
            .insert(left);
    }

    let updates: Vec<Vec<usize>> = input
        .lines()
        .skip(rules.len())
        .filter(|l| l.contains(","))
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let result: usize = updates
        .iter()
        .map(|update| {
            let is_ordered =
                update
                    .iter()
                    .enumerate()
                    .all(|(page_index, page)| match rules.get(page) {
                        Some((lefts, rights)) => {
                            update[0..page_index].iter().all(|p| lefts.contains(p))
                                && update[page_index + 1..].iter().all(|p| rights.contains(p))
                        }
                        _ => false,
                    });

            if is_ordered {
                let middle_num = update[update.len() / 2];
                trace!("Found result: {middle_num}");
                middle_num
            } else {
                0
            }
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
