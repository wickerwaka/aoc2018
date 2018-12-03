use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let series = s.lines().map(|x| x.trim()).collect();

    part1(&series);
    part2(&series);

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

fn part2(input: &Vec<&str>) {
    let max_len = input.get(0).unwrap().len();
    for split_pos in 0..max_len {
        let mut h = HashSet::new();
        for code in input.iter() {
            let t = (&code[0..split_pos], &code[split_pos + 1..]);
            if !h.insert(t) {
                println!("{}{}", t.0, t.1);
                return;
            }
        }
    }
}
