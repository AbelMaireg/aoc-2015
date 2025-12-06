advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev();

    let ops: Vec<char> = lines
        .next()?
        .split_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect();

    let values: Vec<Vec<u64>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let len = ops.len();
    let mut results: u64 = 0;

    for i in 0..len {
        let vals: Vec<u64> = values.iter().map(|v| v[i]).collect::<Vec<u64>>();

        match ops[i] {
            '+' => results += vals.iter().sum::<u64>(),
            '*' => results += vals.iter().product::<u64>(),
            _ => unreachable!(),
        }
    }

    Some(results)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input
        .lines()
        .map(|line| line.chars().rev())
        .collect::<Vec<_>>();
    let size = lines.len();

    let mut results = 0u64;
    let mut bucket: Vec<u64> = vec![];

    loop {
        let mut st = String::new();

        for line in lines[0..size - 1].iter_mut() {
            if let Some(c) = line.next()
                && c.is_ascii_digit()
            {
                st.push(c);
            }
        }

        if let Some(op) = lines[size - 1].next() {
            if st.is_empty() {
                continue;
            }
            let mut num = st.parse::<u64>().unwrap();

            if op == ' ' {
                bucket.push(num);
            } else if op == '+' {
                num += bucket.iter().sum::<u64>();
                bucket.clear();
                results += num;
            } else if op == '*' {
                num *= bucket.iter().product::<u64>();
                bucket.clear();
                results += num;
            }
        } else {
            break;
        }
    }

    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
