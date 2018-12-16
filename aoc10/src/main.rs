use std::cmp;
use std::fs;
use std::io;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Light {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn get_cap(cap: &regex::Captures, i: usize) -> Option<i32> {
    cap.get(i)?.as_str().trim().parse::<i32>().ok()
}

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input/input.txt")?;
    let re = Regex::new(r"position=<(.*),(.*)> velocity=<(.*),(.*)>").unwrap();

    let mut lights: Vec<Light> = s
        .lines()
        .map(|x| {
            let captures = re.captures(x).unwrap();
            let x = get_cap(&captures, 1).unwrap();
            let y = get_cap(&captures, 2).unwrap();
            let vx = get_cap(&captures, 3).unwrap();
            let vy = get_cap(&captures, 4).unwrap();
            Light { x, y, vx, vy }
        }).collect();

    part(&mut lights);
    Ok(())
}

fn frame(lights: &mut Vec<Light>) {
    lights.iter_mut().for_each(|x| {
        x.x += x.vx;
        x.y += x.vy;
    });
}

fn part(lights: &mut Vec<Light>) {
    let mut elapsed = 0;
    loop {
        frame(lights);
        elapsed += 1;
        let mut minx = i32::max_value();
        let mut miny = i32::max_value();
        let mut maxx = i32::min_value();
        let mut maxy = i32::min_value();

        lights.iter().for_each(|p| {
            minx = cmp::min(minx, p.x);
            miny = cmp::min(miny, p.y);
            maxx = cmp::max(maxx, p.x);
            maxy = cmp::max(maxy, p.y);
        });
        if maxy - miny == 9 {
            for y in miny..maxy + 1 {
                for x in minx..maxx + 1 {
                    let exists = lights.iter().any(|p| p.x == x && p.y == y);
                    if exists {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
            println!("Elapsed: {}", elapsed);
            break;
        }
    }
}
