use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left_column.push(iter.next()?.parse().ok()?);
        right_column.push(iter.next()?.parse().ok()?);
    }

    // sort the columns
    left_column.sort();
    right_column.sort();

    // sum the distance between each pair of columns
    let mut sum = 0;
    for (left, right) in left_column.iter().zip(right_column.iter()) {
        sum += (right - left).abs();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left_column.push(iter.next()?.parse().ok()?);
        right_column.push(iter.next()?.parse().ok()?);
    }

    // retrieve how many time each number on the left colum appears on the right column
    // multiply the two numbers and sum the results
    let mut sum = 0;

    let right_number_occurences: HashMap<i32, i32> =
        right_column.iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    for i in 0..left_column.len() {
        let left = left_column[i];
        let count = right_number_occurences.get(&left);

        if count.is_none() {
            continue;
        }

        sum += left * count.unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
