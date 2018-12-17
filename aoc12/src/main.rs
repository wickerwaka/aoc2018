use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::iter::repeat;

type Pattern = [bool; 5];
type PatternMap = HashMap<Pattern, bool>;

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input/input.txt")?;

    let lines: Vec<&str> = s.lines().collect();

    let initial_state: Vec<bool> = lines[0]
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        }).collect();

    let patterns: PatternMap = lines[2..]
        .iter()
        .map(|line| {
            let mut pattern = [false; 5];
            line.chars()
                .take(5)
                .enumerate()
                .for_each(|(idx, c)| match c {
                    '.' => pattern[idx] = false,
                    '#' => pattern[idx] = true,
                    _ => panic!(),
                });
            let output = {
                if line.ends_with("#") {
                    true
                } else {
                    false
                }
            };
            (pattern, output)
        }).collect();

    let mut input = initial_state.clone();
    let mut output = Vec::new();
    let mut zero_point = 0;

    let report_gen: HashSet<usize> = vec![20, 50, 500, 5000, 50000, 500000].into_iter().collect();

    for gen in 0..500000 {
        zero_point += generation(&patterns, &input, &mut output);
        trim_false(&mut output);
        if report_gen.contains(&(gen + 1)) {
            print_sum(&output, zero_point, gen + 1);
        }
        std::mem::swap(&mut input, &mut output);
    }
    Ok(())
}

fn trim_false(v: &mut Vec<bool>) {
    let mut endp = v.len() - 1;
    while v.get(endp) == Some(&false) {
        endp -= 1;
    }
    v.resize(endp + 1, false);
}

fn print_sum(state: &Vec<bool>, zero_point: i64, generation: usize) {
    let total: i64 = state
        .iter()
        .enumerate()
        .map(|(idx, x)| if *x { (idx as i64) - zero_point } else { 0 })
        .sum();
    println!("{} {}", generation, total);
}

fn generation(patterns: &PatternMap, input: &Vec<bool>, output: &mut Vec<bool>) -> i64 {
    output.clear();
    output.reserve(input.len() + 10);

    let padded_input: Vec<bool> = repeat(false)
        .take(4)
        .chain(input.iter().cloned())
        .chain(repeat(false).take(4))
        .collect();

    let mut prepend = 2;
    let mut added = false;

    output.extend(
        padded_input
            .windows(5)
            .filter_map(|inp| match patterns.get(inp) {
                None => panic!(),
                Some(&false) => if added {
                    Some(false)
                } else {
                    prepend -= 1;
                    None
                },
                Some(&true) => {
                    added = true;
                    Some(true)
                }
            }),
    );

    prepend
}
