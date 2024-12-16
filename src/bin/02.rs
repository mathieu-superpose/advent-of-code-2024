advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    // count the number of safe reports

    let count: i32 = input.lines().fold(0, |acc, line| {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let sign: i32 = if report[0] < report[1] { 1 } else { -1 };

        let mut safe = true;

        for i in 0..report.len() - 1 {
            let difference = report[i + 1] - report[i];

            if difference != sign && difference != 2 * sign && difference != 3 * sign {
                safe = false;
                break;
            }
        }

        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("count: {}", count);

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
