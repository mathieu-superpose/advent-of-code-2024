use std::collections::HashSet;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<i32> {
    let data: Vec<&str> = input.split("\n\n").collect();
    let data_towels = data[0].trim().split(", ").collect::<Vec<&str>>();
    let data_design = data[1];

    let mut possible_designs: HashSet<String> = HashSet::new();

    for towel in data_towels {
        possible_designs.insert(towel.to_string());
    }

    let mut check_possible = |design: &str, start: usize, end: usize| -> bool {
        let d = (&design[start..end]).to_string();

        if possible_designs.contains(&d) {
            if start > 0 {
                let commulative_design = (&design[0..end]).to_string();
                possible_designs.insert(commulative_design);
            }
            return true;
        } else {
            return false;
        }
    };

    let mut count = 0;

    for d in data_design.lines() {
        let design: &str = d.trim();

        let mut possible = false;

        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((0, design.len()));

        while stack.len() > 0 {
            if possible {
                break;
            }

            let (start, end) = stack.pop().unwrap();

            for i in start + 1..=end {
                if check_possible(design, start, i) {
                    if i == design.len() {
                        possible = true;
                        break;
                    } else {
                        stack.push((i, end));
                    }
                }
            }
        }

        if possible {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data: Vec<&str> = input.split("\n\n").collect();
    let data_towels = data[0].trim().split(", ").collect::<Vec<&str>>();
    let data_design = data[1];

    let mut possible_designs: HashSet<String> = HashSet::new();

    for towel in data_towels {
        possible_designs.insert(towel.to_string());
    }

    let check_possible = |design: &str, start: usize, end: usize| -> bool {
        let d = (&design[start..end]).to_string();
        possible_designs.contains(&d)
    };

    let mut count = 0;

    for d in data_design.lines() {
        let design: &str = d.trim();

        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((0, design.len()));

        while stack.len() > 0 {
            let (start, end) = stack.pop().unwrap();

            for i in start + 1..=end {
                if check_possible(design, start, i) {
                    if i == design.len() {
                        count += 1;
                    } else {
                        stack.push((i, end));
                    }
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(16));
    }
}
