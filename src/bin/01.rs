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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
