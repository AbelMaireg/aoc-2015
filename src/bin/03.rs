use std::cmp::min;

advent_of_code::solution!(3);

// Generic picker function to select the largest possible number by picking 'picks' digits
fn picker(input: &str, picks: usize) -> u64 {
    let len = input.lines().next().unwrap().len(); // All lines are of equal length
    let rm = len - picks; // Number of digits to remove

    input
        .lines()
        .map(|bank| {
            let bank = bank.as_bytes(); // Convert line to byte array for easier manipulation
            let mut best: Vec<usize> = vec![]; // Indices of selected digits
            let mut drops: usize = 0; // Count of removed digits

            while best.len() != picks {
                let from = best.last().map_or(0, |x| x + 1); // Start index for the next selection

                // If we've removed enough digits, take the rest
                if drops == rm {
                    for i in from..bank.len() {
                        best.push(i);
                    }
                    break;
                }

                let range = rm - drops + 1; // Range to search for the next maximum digit
                let to = from + range; // End index for the next selection

                let max = bank[from..min(to, len)].iter().max().unwrap();
                let pos = bank[from..min(to, len)]
                    .iter()
                    .position(|x| x == max)
                    .unwrap();

                best.push(pos + from);

                drops += pos;
            }

            best.iter()
                .rev()
                .enumerate()
                .map(|(i, x)| {
                    let digit = bank[*x] - b'0';
                    digit as u64 * 10u64.pow(i as u32)
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    picker(input, 2).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    picker(input, 12).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
