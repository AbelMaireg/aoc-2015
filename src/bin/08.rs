use std::collections::BinaryHeap;

advent_of_code::solution!(8);

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut coords = s.split(',').map(|coord| coord.parse::<i64>().unwrap());
        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }

    fn parse_input(input: &str) -> Vec<Self> {
        input.lines().map(Self::from_str).collect()
    }

    fn distance(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Eq, PartialEq)]
struct Pair {
    p1: usize,
    p2: usize,
    dist: i64,
}

impl Pair {
    fn new(p1: usize, p2: usize, dist: i64) -> Self {
        Self { p1, p2, dist }
    }

    fn from_points(points: &[Point]) -> BinaryHeap<Self> {
        let mut pairs = BinaryHeap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dist = points[i].distance(&points[j]);
                pairs.push(Self::new(i, j, dist));
            }
        }
        pairs
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

struct UF {
    id: Vec<usize>,
    sz: Vec<usize>,
    components: usize,
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            id: (0..n).collect(),
            sz: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, p: usize) -> usize {
        let mut root = p;
        while root != self.id[root] {
            root = self.id[root];
        }

        let mut p = p;
        while p != root {
            let next = self.id[p];
            self.id[p] = root;
            p = next;
        }

        root
    }

    fn unify(&mut self, p: usize, q: usize) {
        let root1 = self.find(p);
        let root2 = self.find(q);

        if root1 == root2 {
            return;
        }

        if self.sz[root1] < self.sz[root2] {
            self.id[root1] = root2;
            self.sz[root2] += self.sz[root1];
        } else {
            self.id[root2] = root1;
            self.sz[root1] += self.sz[root2];
        }

        self.components -= 1;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = Point::parse_input(input);
    let mut uf = UF::new(points.len());
    let mut pairs = Pair::from_points(&points);

    for _ in 0..1000 {
        let pair = pairs.pop().unwrap();
        uf.unify(pair.p1, pair.p2);
    }

    let mut sizes = vec![];
    let mut visited = vec![false; points.len()];

    for i in 0..points.len() {
        let root = uf.find(i);
        if !visited[root] {
            visited[root] = true;
            sizes.push(uf.sz[root]);
        }
    }

    sizes.sort();
    sizes.iter().rev().take(3).product::<usize>().into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let points = Point::parse_input(input);
    let mut uf = UF::new(points.len());
    let mut pairs = Pair::from_points(&points);

    while let Some(pair) = pairs.pop() {
        uf.unify(pair.p1, pair.p2);
        if uf.components == 1 {
            return Some(points[pair.p1].x * points[pair.p2].x);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
