advent_of_code::solution!(6);

fn sum_visited(visited: Vec<Vec<Vec<char>>>) -> i32 {
    let mut count = 0;

    for y in 0..visited.len() {
        for x in 0..visited[y].len() {
            if visited[y][x].len() > 0 {
                count += 1;
            }
        }
    }

    count
}

fn count_visited_cells(
    grid: Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    mut direction: char,
) -> i32 {
    let mut visited: Vec<Vec<Vec<char>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];

    loop {
        if visited[y][x].contains(&direction) {
            return -1;
        } else {
            visited[y][x].push(direction);
        }

        match direction {
            '>' => {
                if x + 1 == grid[y].len() {
                    return sum_visited(visited);
                } else if grid[y][x + 1] == '#' {
                    direction = 'v';
                } else {
                    x += 1;
                }
            }
            '<' => {
                if x == 0 {
                    return sum_visited(visited);
                } else if grid[y][x - 1] == '#' {
                    direction = '^';
                } else {
                    x -= 1;
                }
            }
            '^' => {
                if y == 0 {
                    return sum_visited(visited);
                } else if grid[y - 1][x] == '#' {
                    direction = '>';
                } else {
                    y -= 1;
                }
            }
            _ => {
                if y + 1 == grid.len() {
                    return sum_visited(visited);
                } else if grid[y + 1][x] == '#' {
                    direction = '<';
                } else {
                    y += 1;
                }
            }
        }
    }
}

fn get_robot_data(grid: &Vec<Vec<char>>, height: usize, width: usize) -> (usize, usize, char) {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut dir = '>';

    for y in 0..height {
        for x in 0..width {
            let curr = grid[y][x];

            if curr != '^' && curr != '>' && curr != 'v' && curr != '<' {
                continue;
            }

            dir = curr;
            pos_x = x;
            pos_y = y;

            break;
        }
    }

    (pos_x, pos_y, dir)
}

pub fn part_one(input: &str) -> Option<i32> {
    // grid data
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    // robot data
    let (pos_x, pos_y, dir) = get_robot_data(&grid, height, width);

    let count = count_visited_cells(grid, pos_x, pos_y, dir);

    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    // grid data
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    // robot data
    let (pos_x, pos_y, dir) = get_robot_data(&grid, height, width);

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '.' {
                continue;
            }

            let mut new_grid = grid.clone();
            new_grid[y][x] = '#';

            if count_visited_cells(new_grid, pos_x, pos_y, dir) == -1 {
                count += 1;
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(6));
    }
}
