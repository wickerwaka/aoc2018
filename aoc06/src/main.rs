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

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let points: Vec<Point> = s.lines().filter_map(|x| x.parse::<Point>().ok()).collect();

    println!("{:?}", points);

    Ok(())
}
