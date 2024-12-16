use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn report_from(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_range(input: &Vec<i32>) -> RangeInclusive<i32> {
    let mut dir = 0;

    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];

        if diff > 0 {
            dir += 1;
        } else if diff < 0 {
            dir -= 1;
        }
    }

    if dir < 0 {
        return -3..=-1;
    } else {
        return 1..=3;
    }
}

fn is_safe(report: Vec<i32>) -> bool {
    let range = get_range(&report);

    for i in 0..report.len() - 1 {
        let difference = report[i + 1] - report[i];

        if !range.contains(&difference) {
            return false;
        }
    }

    true
}

fn is_less_safe(report: Vec<i32>) -> bool {
    let range = get_range(&report);

    for i in 0..report.len() - 1 {
        let difference = report[i + 1] - report[i];

        if range.contains(&difference) {
            continue;
        }

        // test the report without index i
        let mut safer_report = report.clone();
        safer_report.remove(i);

        if is_safe(safer_report) {
            return true;
        }

        // test the report without index i + 1
        let mut safer_report = report.clone();
        safer_report.remove(i + 1);

        return is_safe(safer_report);
    }

    true
}

pub fn part_one(input: &str) -> Option<i32> {
    // count the number of safe reports
    let count: i32 = input.lines().fold(0, |acc, line| {
        let report = report_from(line);

        match is_safe(report) {
            true => acc + 1,
            false => acc,
        }
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    // count the number of safe reports
    let count: i32 = input.lines().fold(0, |acc, line| {
        let report = report_from(line);

        match is_less_safe(report) {
            true => acc + 1,
            false => acc,
        }
    });

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<i32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
