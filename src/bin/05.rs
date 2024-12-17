use std::i32;

advent_of_code::solution!(5);

fn filter_updates(
    updates: Vec<Vec<i32>>,
    ordering: &Vec<(i32, i32)>,
    correct: bool,
) -> Vec<Vec<i32>> {
    let mut filtered_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut valid = true;

        for (first, last) in ordering {
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

        if valid == correct {
            filtered_updates.push(update);
        }
    }

    filtered_updates
}

fn sum_middle(updates: Vec<Vec<i32>>) -> i32 {
    let mut sum_of_valid_middle = 0;

    for update in updates {
        let middle_index = update.len() / 2;
        sum_of_valid_middle += update[middle_index];
    }

    sum_of_valid_middle
}

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

    let valid_updates = filter_updates(updates, &ordering, true);
    let sum_of_valid_middle = sum_middle(valid_updates);

    Some(sum_of_valid_middle)
}

pub fn part_two(input: &str) -> Option<i32> {
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

    let invalid_updates = filter_updates(updates, &ordering, false);

    // intanciate the total
    let mut total = 0;

    // check if an update is valid according to the rules
    for mut update in invalid_updates {
        let mut valid = false;

        while valid == false {
            let mut changed = false;

            for rule in &ordering {
                // check if rule.0 is in the update and retrieve its index
                let index_0 = update.iter().position(|&x| x == rule.0);

                // check if rule.1 is in the update and retrieve its index
                let index_1 = update.iter().position(|&x| x == rule.1);

                // continue if one of the rule is missing
                if index_0.is_none() || index_1.is_none() {
                    continue;
                }

                // swap both values
                if index_0.unwrap() > index_1.unwrap() {
                    changed = true;

                    let temp = update[index_0.unwrap()];
                    update[index_0.unwrap()] = update[index_1.unwrap()];
                    update[index_1.unwrap()] = temp;
                }
            }

            if !changed {
                valid = true;
            }
        }

        let middle_index = update.len() / 2;
        let middle_value = update[middle_index];

        total += middle_value;
    }

    Some(total)
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
        assert_eq!(result, Some(123));
    }
}
