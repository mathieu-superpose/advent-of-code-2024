advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let data = line.split(": ").collect::<Vec<&str>>();
        let target = data[0].parse().unwrap();
        let values: Vec<u64> = data[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut results = Vec::new();
        results.push(values[0]);

        for i in 1..values.len() {
            let mut new_results = Vec::new();
            let curr = values[i];

            for result in &results {
                let sum = result + curr;
                if sum <= target {
                    new_results.push(sum);
                }

                let product = result * curr;
                if product <= target {
                    new_results.push(product);
                }
            }

            results = new_results;
        }

        if results.contains(&target) {
            sum += target;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let data = line.split(": ").collect::<Vec<&str>>();
        let target = data[0].parse().unwrap();
        let values: Vec<u64> = data[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut results = Vec::new();
        results.push(values[0]);

        for i in 1..values.len() {
            let mut new_results = Vec::new();
            let curr = values[i];

            for result in &results {
                let sum = result + curr;
                if sum <= target {
                    new_results.push(sum);
                }

                let product = result * curr;
                if product <= target {
                    new_results.push(product);
                }

                let concat: u64 = format!("{}{}", result, curr).parse().unwrap();
                if concat <= target {
                    new_results.push(concat);
                }
            }

            results = new_results;
        }

        if results.contains(&target) {
            sum += target;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(11387));
    }
}
