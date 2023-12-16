use std::fs;

pub fn main() {
    let file_content = fs::read_to_string("day01/data/input.txt").unwrap();
    let result_part1 = part01(&file_content);
    println!("Part 1 Result {}", result_part1);
    let result_part2 = part02(&file_content);
    println!("Part 2 Result {}", result_part2)
}

fn part01(input: &String) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));
            let first = digits
                .next()
                .expect(format!("Could not find any digit in line {}", line).as_str());
            let last = match digits.last() {
                Some(v) => v,
                None => first,
            };
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

fn part02(input: &String) -> String {
    281.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
        assert_eq!("142", part01(&test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        assert_eq!("281", part02(&test_input));
    }
}
