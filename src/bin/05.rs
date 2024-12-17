use std::i32;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let ordering: Vec<(i32, i32)> = data[0]
        .lines()
        .map(|l| {
            let mut nums = l.trim().split("|").map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let updates: Vec<Vec<i32>> = data[1]
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum_of_valid_middle = 0;

    for update in updates {
        let mut valid = true;

        for (first, last) in &ordering {
            let mut first_index = -1;
            let mut last_index = i32::MAX;

            for (i, &u) in update.iter().enumerate() {
                if u == *first {
                    first_index = i as i32;
                }
                if u == *last {
                    last_index = i as i32;
                }
            }

            if first_index > last_index {
                valid = false;
                break;
            }
        }

        if valid {
            let middle_index = update.len() / 2;
            sum_of_valid_middle += update[middle_index];
        }
    }

    Some(sum_of_valid_middle)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
