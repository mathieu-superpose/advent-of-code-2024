advent_of_code::solution!(3);

use regex::Regex;

fn parse_input(input: &str) -> String {
    input.replace("\n", "")
}

pub fn part_one(input: &str) -> Option<i32> {
    let one_line = parse_input(input);

    let mut total: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(&one_line) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        total += a * b;
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
