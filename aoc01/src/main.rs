use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let series = s.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    part1(&series);
    part2(&series);

    Ok(())
}

fn part1(input: &Vec<i64>) {
    let mut accum = 0;
    for value in input.iter() {
        accum += value;
    }
    println!("{}", accum);
}

fn part2(input: &Vec<i64>) {
    let mut occurs = HashSet::new();
    let mut accum = 0;
    occurs.insert(accum);
    for value in input.iter().cycle() {
        accum += value;
        if !occurs.insert(accum) {
            println!("{}", accum);
            return;
        }
    }
}
