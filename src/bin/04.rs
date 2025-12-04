advent_of_code::solution!(4);

const OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 0,
                    '.' => -1,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn lift(grid: &mut Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut lifts = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 0 {
                continue;
            }

            let count = OFFSETS
                .iter()
                .filter(|(dx, dy)| {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx < 0 || ny < 0 || ny >= h || nx >= w {
                        false
                    } else {
                        grid[ny as usize][nx as usize] == 0
                    }
                })
                .count();

            if count < 4 {
                lifts.push((x, y));
            }
        }
    }

    lifts
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);

    lift(&mut grid).len().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);

    let mut forkables = 0;

    loop {
        let lifts = lift(&mut grid);
        match lifts.len() {
            0 => break,
            a => {
                forkables += a;
                for (x, y) in lifts {
                    grid[y][x] = -1;
                }
            }
        }
    }

    forkables.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
