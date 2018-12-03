use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let series = s.lines().map(|x| x.trim()).collect();

    part1(&series);

    Ok(())
}

fn part1(input: &Vec<&str>) {
    let mut twos = 0;
    let mut threes = 0;
    for code in input.iter() {
        let mut h = HashMap::new();
        for c in code.as_bytes().iter() {
            let counter = h.entry(c).or_insert(0);
            *counter += 1;
        }
        if h.values().any(|&x| x == 2) {
            twos += 1;
        }
        if h.values().any(|&x| x == 3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
}
