advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut splits = vec![true; input.lines().next()?.chars().count()];
    splits[input.lines().next()?.find('S')?] = true;

    let mut count = 0;
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(s, c)| {
            if c == '^' && splits[s] {
                splits[s + 1] = true;
                splits[s - 1] = true;
                splits[s] = false;
                count += 1;
            }
        })
    });

    count.into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut splits = vec![0; input.lines().next()?.chars().count()];
    splits[input.lines().next()?.find('S')?] = 1;

    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(s, c)| {
            if c == '^' && splits[s] != 0 {
                let beams = splits[s];
                splits[s + 1] += beams;
                splits[s - 1] += beams;
                splits[s] = 0;
            }
        })
    });

    splits.iter().sum::<u64>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
