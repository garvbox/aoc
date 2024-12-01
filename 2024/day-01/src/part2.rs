use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut lefts: Vec<usize> = vec![];
    let mut rights: Vec<usize> = vec![];

    for line in input.lines() {
        let mut split_line = line.split_whitespace();
        lefts.push(split_line.next().unwrap().parse().unwrap());
        rights.push(split_line.next().unwrap().parse().unwrap());
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut total: usize = 0;

    for l in lefts {
        let num_occurences = counts
            .entry(l)
            .or_insert_with(|| rights.iter().filter(|x| **x == l).count());
        total += *num_occurences * l;
        tracing::trace!("Got num_occurences: {} for entry {}", num_occurences, l);
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
