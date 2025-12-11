use std::collections::BinaryHeap;

advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone)]
struct Point {
    y: i64,
    x: i64,
}

impl Point {
    fn area(&self, other: &Point) -> i64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        (width * height) as i64
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Self {
            y: y.parse().expect("row should be numeric"),
            x: x.parse().expect("col should be numeric"),
        }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
    area: i64,
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
}

impl Eq for Rectangle {}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.area.cmp(&other.area))
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

impl Rectangle {
    fn new(p1: &Point, p2: &Point) -> Self {
        let area = p1.area(&p2);
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            area,
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2)))
        .map(|(p1, p2)| p1.area(p2))
        .max()
        .expect("points should not be empty")
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let points: Vec<Point> = input.lines().map(Point::from).collect();

    let edges: Vec<(&Point, &Point)> = points
        .windows(2)
        .map(|vertices| (&vertices[0], &vertices[1]))
        .chain([(&points[points.len() - 1], &points[0])]) // closing edge
        .collect();

    let mut possible_rects: BinaryHeap<Rectangle> = BinaryHeap::new();
    points.iter().enumerate().for_each(|(i, p1)| {
        points[i + 1..]
            .iter()
            .for_each(|p2| possible_rects.push(Rectangle::new(p1, p2)));
    });

    while let Some(Rectangle { p1, p2, area }) = possible_rects.pop() {
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);
        // all edges in the full polygon must be:
        //   - leftmost point of edge must be left of this rect OR
        //   - rightmost point of edge must be right of this rect OR
        //   - topmost point of edge must be above this rect OR
        //   - bottommost point of edge must be below this rect
        if edges.iter().all(|(start, end)| {
            x_max <= start.x.min(end.x) || // is before
            x_min >= start.x.max(end.x) || // is after
            y_max <= start.y.min(end.y) || // is above
            y_min >= start.y.max(end.y) // is below
        }) {
            return Some(area);
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
