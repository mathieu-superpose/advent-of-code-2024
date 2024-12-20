use std::collections::HashSet;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // print grid
    for line in &grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    let height = grid.len();
    let width: usize = grid[0].len();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
            if grid[y][x] == 'E' {
                end = (x, y);
            }
        }
    }

    println!("height: {}, width: {}", height, width);
    println!("start: {:?}", start);
    println!("end: {:?}", end);

    let mut grid_distances: Vec<Vec<usize>> = vec![vec![std::usize::MAX; width]; height];

    let mut stack: Vec<(usize, usize, usize)> = vec![(end.0, end.1, 0)];

    grid_distances[end.1][end.0] = 0;

    while !stack.is_empty() {
        let (init_x, init_y, distance) = stack.pop().unwrap();

        for dir in [
            (0, -1), // up
            (0, 1),  // down
            (-1, 0), // left
            (1, 0),  // right
        ]
        .iter()
        {
            let (dx, dy) = dir;

            if *dx == -1 && init_x == 0 {
                continue;
            }
            if *dx == 1 && init_x == width - 1 {
                continue;
            }
            if *dy == -1 && init_y == 0 {
                continue;
            }
            if *dy == 1 && init_y == height - 1 {
                continue;
            }

            let nx = (init_x as i32 + dx) as usize;
            let ny = (init_y as i32 + dy) as usize;

            if grid[ny][nx] == '#' || grid_distances[ny][nx] <= distance + 1 {
                continue;
            } else {
                grid_distances[ny][nx] = distance + 1;
                stack.push((nx, ny, distance + 1));
            }
        }
    }

    // count the number of shortrack that saves at least 100 steps
    let mut count = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut stack: Vec<(usize, usize)> = vec![(start.0, start.1)];

    while !stack.is_empty() {
        let (init_x, init_y) = stack.pop().unwrap();

        let init_distance = grid_distances[init_y][init_x];

        for dir in [
            (0, -2),  // up
            (0, 2),   // down
            (-2, 0),  // left
            (2, 0),   // right
            (1, 1),   // right-down
            (-1, 1),  // left-down
            (1, -1),  // right-up
            (-1, -1), // left-up
        ]
        .iter()
        {
            let (dx, dy) = dir;

            if *dx == -1 && init_x < 1 {
                continue;
            }
            if *dx == -2 && init_x < 2 {
                continue;
            }
            if *dx == 1 && init_x > width - 2 {
                continue;
            }
            if *dx == 2 && init_x > width - 3 {
                continue;
            }
            if *dy == -1 && init_y < 1 {
                continue;
            }
            if *dy == -2 && init_y < 2 {
                continue;
            }
            if *dy == 1 && init_y > height - 2 {
                continue;
            }
            if *dy == 2 && init_y > height - 3 {
                continue;
            }

            let nx = (init_x as i32 + dx) as usize;
            let ny = (init_y as i32 + dy) as usize;

            if visited.contains(&(nx, ny)) {
                continue;
            }

            if grid[ny][nx] == '#' {
                continue;
            }

            let next_distance = grid_distances[ny][nx];

            if next_distance >= init_distance + 1 {
                continue;
            }

            if init_distance - next_distance > 100 {
                println!(
                    "({}, {}) -> ({}, {}) saves {}",
                    init_x,
                    init_y,
                    nx,
                    ny,
                    init_distance - next_distance
                );

                count += 1;
            }
        }

        visited.insert((init_x, init_y));

        for dir in [
            (0, -1), // up
            (0, 1),  // down
            (-1, 0), // left
            (1, 0),  // right
        ]
        .iter()
        {
            let (dx, dy) = dir;

            if *dx == -1 && init_x == 0 {
                continue;
            }
            if *dx == 1 && init_x == width - 1 {
                continue;
            }
            if *dy == -1 && init_y == 0 {
                continue;
            }
            if *dy == 1 && init_y == height - 1 {
                continue;
            }

            let nx = (init_x as i32 + dx) as usize;
            let ny = (init_y as i32 + dy) as usize;

            if visited.contains(&(nx, ny)) {
                continue;
            }

            if grid[ny][nx] == '#' {
                continue;
            }

            stack.push((nx, ny));
        }
    }

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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
