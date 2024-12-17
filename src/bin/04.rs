advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // count the number of occurence of the word XMAS in the grid in all directions
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let curr = grid[i][j];

            if curr != 'X' {
                continue;
            }

            for (dir_y, dir_x) in vec![
                (-1, -1), // top left
                (-1, 0),  // top
                (-1, 1),  // top right
                (0, -1),  // left
                (0, 1),   // right
                (1, -1),  // bottom right
                (1, 0),   // bottom
                (1, 1),   // bottom left
            ] {
                let mut xmas: String = String::new();

                for index in 0..=3 {
                    let y = i as i32 + dir_y * index;
                    let x = j as i32 + dir_x * index;

                    if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[i].len() as i32 {
                        break;
                    }

                    xmas.push(grid[y as usize][x as usize]);
                }

                if xmas == "XMAS" {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // count the number of occurence of the word XMAS in the grid in all directions
    let mut count = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let curr = grid[i][j];

            if curr != 'A' {
                continue;
            }

            let surrounding: String = vec![
                grid[i - 1][j - 1], // top left
                grid[i - 1][j + 1], // top right
                grid[i + 1][j + 1], // bottom right
                grid[i + 1][j - 1], // bottom left
            ]
            .iter()
            .collect();

            match surrounding.as_str() {
                "MMSS" | "MSSM" | "SSMM" | "SMMS" => count += 1,
                _ => (),
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(9));
    }
}
