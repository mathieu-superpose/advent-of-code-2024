use std::collections::HashMap;

advent_of_code::solution!(6);

fn count_visited_cells(
    grid: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    mut pos: (usize, usize),
    mut dir: (i32, i32),
    new_wall: Option<(usize, usize)>,
) -> i32 {
    let mut visited: HashMap<(usize, usize), String> = HashMap::new();

    loop {
        let next_x = pos.1 as i32 + dir.1;
        let next_y = pos.0 as i32 + dir.0;

        // check if the robot is still on the grid
        if next_y < 0 || next_y >= height as i32 || next_x < 0 || next_x >= width as i32 {
            break;
        }

        let next = match new_wall {
            Some((x, y)) => {
                if x == next_x as usize && y == next_y as usize {
                    '#'
                } else {
                    grid[next_y as usize][next_x as usize]
                }
            }
            None => grid[next_y as usize][next_x as usize],
        };

        // turn clockwise if next is a wall (#)
        if next == '#' {
            if dir.1 == 0 {
                dir.1 = -dir.0;
                dir.0 = 0;
            } else {
                dir.0 = dir.1;
                dir.1 = 0;
            }
        } else {
            // move the robot
            pos.1 = next_x as usize;
            pos.0 = next_y as usize;
        }

        // mark the cell as visited
        let sign = if dir.1 == 0 {
            if dir.0 == -1 {
                '^'
            } else {
                'v'
            }
        } else {
            if dir.1 == -1 {
                '<'
            } else {
                '>'
            }
        };

        if visited.contains_key(&(pos.1, pos.0)) {
            let visits = visited.get_mut(&(pos.1, pos.0)).unwrap();
            if visits.contains(sign) {
                return -1;
            } else {
                visits.push(sign);
            }
        } else {
            visited.insert((pos.1, pos.0), sign.to_string());
        }
    }

    // return the number of visited cells
    visited.len() as i32
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    // robot data
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut dir_x = 0;
    let mut dir_y = 0;

    // init robot
    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                '^' => {
                    dir_x = 0;
                    dir_y = -1;
                }
                '>' => {
                    dir_x = 1;
                    dir_y = 0;
                }
                'v' => {
                    dir_x = 0;
                    dir_y = 1;
                }
                '<' => {
                    dir_x = -1;
                    dir_y = 0;
                }
                _ => continue,
            }

            pos_x = x;
            pos_y = y;
            grid[y][x] = 'X';
        }
    }

    let count = count_visited_cells(&grid, height, width, (pos_y, pos_x), (dir_y, dir_x), None);

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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
