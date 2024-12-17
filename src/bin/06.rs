advent_of_code::solution!(6);

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

    // move the robot until it leaves the grid

    loop {
        let next_x = pos_x as i32 + dir_x;
        let next_y = pos_y as i32 + dir_y;

        // check if the robot is still on the grid
        if next_y < 0 || next_y >= height as i32 || next_x < 0 || next_x >= width as i32 {
            break;
        }

        let next = grid[next_y as usize][next_x as usize];

        // turn clockwise if next is a wall (#)
        if next == '#' {
            if dir_x == 0 {
                dir_x = -dir_y;
                dir_y = 0;
            } else {
                dir_y = dir_x;
                dir_x = 0;
            }
        } else {
            // move the robot
            pos_x = next_x as usize;
            pos_y = next_y as usize;
            grid[pos_y][pos_x] = 'X';
        }
    }

    // count the number of visited cells
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'X' {
                count += 1;
            }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
