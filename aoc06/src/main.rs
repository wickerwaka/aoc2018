#[macro_use]
use std::error::Error;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::result;
use std::str::FromStr;

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

    println!("{:?}", points);

    Ok(())
}
