use std::collections::HashSet;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i32> {
    // // example init
    // let height = 7;
    // let width = 7;
    // let max_byte = 12;
    // let i_height: i32 = height as i32;
    // let i_width: i32 = width as i32;

    // final init
    let max_byte = 1024;
    let height = 71;
    let width = 71;
    let i_height: i32 = height as i32;
    let i_width: i32 = width as i32;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];

    let mut lines = input.lines();
    let mut valid_lines: usize = 0;

    while valid_lines < max_byte {
        let data = lines
            .next()
            .unwrap()
            .trim()
            .split(",")
            .collect::<Vec<&str>>();
        let x = data[0].parse::<usize>().unwrap();
        let y = data[1].parse::<usize>().unwrap();

        if x < width && y < height {
            grid[y][x] = '#';
            valid_lines += 1;
        }
    }

    // // print grid
    // println!("Initial grid:");
    // for row in grid.iter() {
    //     for cell in row.iter() {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }
    // println!();

    let mut round = 1;
    grid[0][0] = 'O';

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
                if x < 0 || x >= i_width || y < 0 || y >= i_height {
                    continue;
                }

                if grid[y as usize][x as usize] == '#' {
                    continue;
                }

                if visited.contains(&(x, y)) {
                    continue;
                }

                grid[y as usize][x as usize] = 'O';

                if x == i_width - 1 && y == i_height - 1 {
                    // println!("Final grid:");
                    // for row in grid.iter() {
                    //     for cell in row.iter() {
                    //         print!("{}", cell);
                    //     }
                    //     println!();
                    // }

                    return Some(round);
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

    // println!("Exit grid:");
    // for row in grid.iter() {
    //     for cell in row.iter() {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }
    Some(round)
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_one_full() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY, None));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
