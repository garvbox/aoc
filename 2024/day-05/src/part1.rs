use tracing::trace;

pub fn process(input: &str) -> miette::Result<String> {
    let rules: Vec<(usize, usize)> = input
        .lines()
        .by_ref()
        .filter(|l| l.contains("|"))
        .map(|line| {
            let mut nums = line.split("|");
            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let result: usize = input
        .lines()
        .skip(rules.len())
        .filter(|l| l.contains(","))
        .map(|line| {
            trace!("Line: {line}");
            let page_nums: Vec<usize> = line
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            let is_ordered = page_nums.iter().enumerate().all(|(page_index, page)| {
                rules
                    .iter()
                    .filter(|rule| rule.0 == *page || rule.1 == *page)
                    .all(|rule| {
                        // Finding the position of the "other" element of the current rule set
                        let other_position = page_nums.iter().position(|&pos| {
                            if rule.0 == *page {
                                rule.1 == pos
                            } else {
                                rule.0 == pos
                            }
                        });
                        if other_position.is_none() {
                            return true;
                        }
                        trace!(
                            "Page {}, Rule: {}|{}, other position {}",
                            page,
                            rule.0,
                            rule.1,
                            other_position.unwrap()
                        );

                        if rule.0 == *page {
                            Some(page_index) < other_position
                        } else {
                            Some(page_index) > other_position
                        }
                    })
            });

            if is_ordered {
                let middle_num = page_nums[page_nums.len() / 2];
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
