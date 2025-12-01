advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    input.lines().for_each(|line| {
        let delta = if line[..1].eq("L") {
            -line[1..].parse::<i32>().unwrap()
        } else {
            line[1..].parse::<i32>().unwrap()
        };

        dial = (dial + delta).rem_euclid(100);

        if dial == 0 {
            password += 1;
        }
    });

    password.into()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;

    input.lines().for_each(|line| {
        let delta = if line[..1].eq("L") {
            -line[1..].parse::<i32>().unwrap()
        } else {
            line[1..].parse::<i32>().unwrap()
        };

        if delta >= 0 {
            password += (dial + delta) / 100;
        } else {
            password += ((dial - 100) % 100 + delta).abs() / 100;
        }

        dial = (dial + delta).rem_euclid(100);
    });

    password.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
