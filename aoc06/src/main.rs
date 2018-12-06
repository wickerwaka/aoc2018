use std::cmp;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ints: Vec<_> = s.split(',').collect();

        let x = ints[0].trim().parse::<i32>()?;
        let y = ints[1].trim().parse::<i32>()?;

        Ok(Point { x, y })
    }
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn nearest(&self, points: &[Point]) -> Option<usize> {
        let mut distances: Vec<(usize, i32)> = points
            .iter()
            .map(|x| self.distance(&x))
            .enumerate()
            .collect();
        distances.sort_by_key(|x| x.1);

        if distances.len() == 0 {
            None
        } else if distances.len() == 1 {
            Some(distances[0].0)
        } else {
            if distances[0].1 == distances[1].1 {
                None
            } else {
                Some(distances[0].0)
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let points: Vec<Point> = s.lines().filter_map(|x| x.parse::<Point>().ok()).collect();
    let mut min_point = Point {
        x: i32::max_value(),
        y: i32::max_value(),
    };
    let mut max_point = Point {
        x: i32::min_value(),
        y: i32::min_value(),
    };
    for point in &points {
        min_point.x = cmp::min(min_point.x, point.x);
        min_point.y = cmp::min(min_point.y, point.y);
        max_point.x = cmp::max(max_point.x, point.x);
        max_point.y = cmp::max(max_point.y, point.y);
    }

    part1(&points, &min_point, &max_point);
    part2(&points, &min_point, &max_point);
    Ok(())
}

fn part2(points: &[Point], min_point: &Point, max_point: &Point) {
    let mut area_size = 0;
    for y in min_point.y..max_point.y + 1 {
        for x in min_point.x..max_point.y + 1 {
            let point = Point { x, y };
            let distance = points.iter().fold(0, |acc, p| acc + point.distance(p));
            if distance < 10000 {
                area_size += 1;
            }
        }
    }

    println!("Part2 {}", area_size);
}

fn part1(points: &[Point], min_point: &Point, max_point: &Point) {
    let mut infinite_points = HashSet::new();
    let mut area = HashMap::new();

    for y in min_point.y..max_point.y + 1 {
        for x in min_point.x..max_point.y + 1 {
            let point = Point { x, y };
            if let Some(nearest) = point.nearest(points) {
                if (x == min_point.x)
                    || (y == min_point.y)
                    || (x == max_point.x)
                    || (y == max_point.y)
                {
                    infinite_points.insert(nearest);
                }
                let mut count = area.entry(nearest).or_insert(0 as u32);
                *count += 1;
            }
        }
    }

    let largest = area
        .iter()
        .filter(|(idx, _)| !infinite_points.contains(idx))
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!("Part1: {}", largest.1);
}
