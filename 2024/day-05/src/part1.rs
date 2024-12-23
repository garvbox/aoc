use std::collections::{HashMap, HashSet};

use tracing::trace;

pub fn process(input: &str) -> miette::Result<String> {
    // NOTE: Rules is a map of entries to collections of elements which should be after that
    // that element, allowing easy sorting
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in input.lines().filter(|l| l.contains("|")) {
        let mut nums = line.split("|");
        let left = nums.next().unwrap().parse().unwrap();
        let right = nums.next().unwrap().parse().unwrap();
        rules.entry(left).or_default().insert(right);
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
        .filter(|update| {
            update.is_sorted_by(|a, b| rules.get(a).is_some_and(|pages| pages.contains(b)))
        })
        .map(|update| {
            let middle_num = update[update.len() / 2];
            trace!("Found result: {middle_num}");
            middle_num
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
