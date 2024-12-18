use std::collections::HashSet;

advent_of_code::solution!(18);

fn steps_to_exit(grid: &Vec<Vec<char>>, height: i32, width: i32) -> i32 {
    let mut round = 1;

    let mut stack: Vec<(i32, i32)> = vec![(0, 0)];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        let mut new_stack: Vec<(i32, i32)> = vec![];

        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();

            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));

            for (x, y) in vec![
                (x, y - 1), // top
                (x, y + 1), // bottom
                (x - 1, y), // left
                (x + 1, y), // right
            ] {
                if x < 0 || x >= width || y < 0 || y >= height {
                    continue;
                }

                if grid[y as usize][x as usize] == '#' {
                    continue;
                }

                if visited.contains(&(x, y)) {
                    continue;
                }

                if x == width - 1 && y == height - 1 {
                    return round;
                }

                new_stack.push((x, y));
            }
        }

        if new_stack.len() == 0 {
            break;
        }

        stack = new_stack;
        round += 1;
    }

    -1
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut width = 0;
    let mut height = 0;

    let adresses: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let data = line.trim().split(",").collect::<Vec<&str>>();
            let x = data[0].parse::<usize>().unwrap();
            let y = data[1].parse::<usize>().unwrap();

            if x >= width {
                width = x + 1;
            }
            if y >= height {
                height = y + 1;
            }

            (x, y)
        })
        .collect();

    let max_byte = if width < 10 { 12 } else { 1024 };

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];

    for byte_index in 0..max_byte {
        let (x, y) = adresses[byte_index];
        grid[y][x] = '#';
    }

    let round = steps_to_exit(&grid, height as i32, width as i32);

    Some(round)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut width = 0;
    let mut height = 0;

    let adresses: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let data = line.trim().split(",").collect::<Vec<&str>>();
            let x = data[0].parse::<usize>().unwrap();
            let y = data[1].parse::<usize>().unwrap();

            if x >= width {
                width = x + 1;
            }
            if y >= height {
                height = y + 1;
            }

            (x, y)
        })
        .collect();

    let max_byte = if width < 10 { 12 } else { 1024 };

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];

    for byte_index in 0..max_byte {
        let (x, y) = adresses[byte_index];
        grid[y][x] = '#';
    }

    let mut byte_index = max_byte - 1;

    while steps_to_exit(&grid, height as i32, width as i32) != -1 {
        byte_index += 1;
        let (x, y) = adresses[byte_index];
        grid[y][x] = '#';
    }

    let last_memory =
        adresses[byte_index].0.to_string() + "," + &adresses[byte_index].1.to_string();

    Some(last_memory.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
