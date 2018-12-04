use std::collections::HashMap;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Region {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let re = Regex::new(r"#(\d+)\s*@\s*(\d+),(\d+)\s*:\s*(\d+)x(\d+)").unwrap();

    let series = s
        .lines()
        .filter_map(|x| {
            if let Some(captures) = re.captures(x) {
                Some(Region {
                    id: captures.get(1).unwrap().as_str().parse().unwrap(),
                    x: captures.get(2).unwrap().as_str().parse().unwrap(),
                    y: captures.get(3).unwrap().as_str().parse().unwrap(),
                    w: captures.get(4).unwrap().as_str().parse().unwrap(),
                    h: captures.get(5).unwrap().as_str().parse().unwrap(),
                })
            } else {
                None
            }
        }).collect();

    part1_2(&series);

    Ok(())
}

fn part1_2(input: &Vec<Region>) {
    let mut h = HashMap::new();
    for region in input.iter() {
        for x in region.x..region.x + region.w {
            for y in region.y..region.y + region.h {
                let counter = h.entry((x, y)).or_insert(0);
                *counter += 1;
            }
        }
    }

    println!("{}", h.values().filter(|&x| *x > 1).count());

    for region in input.iter() {
        let mut overlapped = false;
        for x in region.x..region.x + region.w {
            for y in region.y..region.y + region.h {
                let counter = h.get(&(x, y)).unwrap_or(&0);
                if *counter > 1 {
                    overlapped = true;
                }
            }
        }
        if !overlapped {
            println!("Intacted: {}", region.id);
        }
    }
}
