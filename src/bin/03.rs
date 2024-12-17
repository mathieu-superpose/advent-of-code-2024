advent_of_code::solution!(3);

use regex::Regex;

fn parse_input(input: &str) -> String {
    input.replace("\n", "")
}

fn filter_line(line: String) -> Option<String> {
    // regex to catch all valid entries
    // "mul(a,b)" or "do()" or "don't()"
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    // catch all valid entries within a line and return them as a single string
    let mut result = String::new();
    for cap in re.captures_iter(&line) {
        result.push_str(&cap[0]);
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn sum_line(line: String) -> u64 {
    let mut total: u64 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(&line) {
        let a: u64 = cap[1].parse().unwrap();
        let b: u64 = cap[2].parse().unwrap();
        total += a * b;
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let one_line = parse_input(input);
    let total = sum_line(one_line);

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let one_line = parse_input(input);
    let filtered_line = filter_line(one_line)?;

    // use regex to delete all disabled mul, the ones that follow don't() instruction from total_line
    let re = Regex::new(r"don\'t\(\)(mul\(\d+,\d+\))+").unwrap();
    let final_line = re.replace_all(&filtered_line, "").to_string();

    let total = sum_line(final_line);

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, Some("-b")));
        assert_eq!(result, Some(48));
    }
}
