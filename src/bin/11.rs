use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

struct Reactor {
    atoms: HashMap<String, HashSet<String>>,
}

impl Reactor {
    fn from_input(input: &str) -> Self {
        let atoms = input
            .lines()
            .map(|line| {
                let mut atom_it = line.split_whitespace();

                let l: String = atom_it.next().unwrap()[..3].into();
                let rs: HashSet<String> = atom_it.map(|i| i.into()).collect();

                (l, rs)
            })
            .collect();

        Self { atoms }
    }

    fn paths(&mut self, start: &str, end: &str) -> u64 {
        let mut cache: HashMap<String, u64> = HashMap::new();
        self.dfs(start, end, &mut cache)
    }

    fn dfs(&mut self, start: &str, end: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if start == end {
            1
        } else if !cache.contains_key(start) {
            let result = self
                .atoms
                .get(start)
                .unwrap_or(&HashSet::new())
                .clone()
                .iter()
                .map(|nxt| self.dfs(nxt, end, cache))
                .sum();
            cache.insert(start.into(), result);
            result
        } else {
            *cache.get(start).unwrap()
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut reactor = Reactor::from_input(input);
    Some(reactor.paths("you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut reactor = Reactor::from_input(input);
    Some(reactor.paths("svr", "fft") * reactor.paths("fft", "dac") * reactor.paths("dac", "out"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
