use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_width = grid[0].len() as i32;
    let max_height = grid.len() as i32;

    // list and group every antenna and their location on the grid
    // an antena is marked ether by a number or a letter

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut resonances: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];

            if c.is_alphabetic() || c.is_numeric() {
                let entry = antennas.entry(c).or_insert(Vec::new());

                // update resonances if entry is not empty
                if !entry.is_empty() {
                    for (x0, y0) in entry.iter() {
                        let vec_x = x as i32 - *x0 as i32;
                        let vec_y = y as i32 - *y0 as i32;

                        // previous resonance
                        let prev_x = *x0 as i32 - &vec_x;
                        let prev_y = *y0 as i32 - &vec_y;

                        if prev_x >= 0 && prev_x < max_width && prev_y >= 0 && prev_y < max_height {
                            resonances.insert((prev_x, prev_y));
                        }

                        // next resonance
                        let next_x = x as i32 + vec_x;
                        let next_y = y as i32 + vec_y;

                        if next_x >= 0 && next_x < max_width && next_y >= 0 && next_y < max_height {
                            resonances.insert((next_x, next_y));
                        }
                    }
                }

                entry.push((x, y));
            }
        }
    }

    // count resonances
    let count = resonances.len();

    Some(count)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
