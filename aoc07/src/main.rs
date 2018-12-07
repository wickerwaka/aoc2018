use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::str;

extern crate regex;
use regex::Regex;

type PendingSteps = HashMap<u8, HashSet<u8>>;

fn add_step(pending: &mut PendingSteps, step: u8, prereq: u8) {
    pending.entry(prereq).or_default();
    let set = pending.entry(step).or_default();
    set.insert(prereq);
}

fn get_next_step(pending: &mut PendingSteps) -> Option<u8> {
    let next_step = pending
        .iter()
        .filter_map(
            |(step, prereqs)| {
                if prereqs.len() == 0 {
                    Some(step)
                } else {
                    None
                }
            },
        ).min()
        .cloned();

    match next_step {
        Some(step) => {
            pending.remove(&step);
            Some(step)
        }
        None => None,
    }
}

fn complete_step(pending: &mut PendingSteps, step: u8) {
    pending.iter_mut().for_each(|(_, prereqs)| {
        prereqs.remove(&step);
    });
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let re = Regex::new(r".*Step (\S) must.*(\S) can begin.*").unwrap();

    let mut pending_steps = PendingSteps::new();

    s.lines().for_each(|x| {
        if let Some(captures) = re.captures(x) {
            let prereq = captures.get(1).unwrap().as_str().as_bytes()[0];
            let step = captures.get(2).unwrap().as_str().as_bytes()[0];
            add_step(&mut pending_steps, step, prereq);
        }
    });

    part1(&pending_steps);
    part2(&pending_steps);

    Ok(())
}

fn part1(pending: &PendingSteps) {
    let mut pending_steps = pending.clone();
    let mut steps = Vec::new();
    while let Some(step) = get_next_step(&mut pending_steps) {
        steps.push(step);
        complete_step(&mut pending_steps, step);
    }

    let result = str::from_utf8(&steps).unwrap();
    println!("Part1 {}", result);
}

struct Worker {
    step: u8,
    finished: i32,
}

fn part2(pending: &PendingSteps) {
    let mut pending_steps = pending.clone();
    let mut timer = 0;

    let mut workers: Vec<Worker> = Vec::new();

    let mut completed_steps = Vec::new();

    loop {
        while let Some(step) = get_next_step(&mut pending_steps) {
            workers.push(Worker {
                step: step,
                finished: (step - 4) as i32 + timer,
            });
            if workers.len() == 5 {
                break;
            }
        }
        workers.sort_by_key(|x| -x.finished);
        if let Some(next_to_finish) = workers.pop() {
            timer = next_to_finish.finished;
            complete_step(&mut pending_steps, next_to_finish.step);
            completed_steps.push(next_to_finish.step);
            println!("{} {}", str::from_utf8(&completed_steps).unwrap(), timer);
        }

        if pending_steps.len() == 0 && workers.len() == 0 {
            break;
        }
    }

    println!("Part2 {}", timer);
}
