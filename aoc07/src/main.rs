use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let re = Regex::new(r".*Step (\S) must.*(\S) can begin.*").unwrap();

    let mut m = HashMap::new();

    s.lines().for_each(|x| {
        if let Some(captures) = re.captures(x) {
            let prereq = captures.get(1).unwrap().as_str().as_bytes()[0];
            let step = captures.get(2).unwrap().as_str().as_bytes()[0];
            m.entry(prereq).or_insert(HashSet::new());

            let mut set = m.entry(step).or_insert(HashSet::new());
            set.insert(prereq);
        }
    });

    let mut v: Vec<(u8, HashSet<u8>)> = m.iter().map(|(x, y)| (x.clone(), y.clone())).collect();

    loop {
        v.sort_unstable_by(|(a, apre), (b, bpre)| {
            let diff = bpre.len().cmp(&apre.len());
            if diff == Ordering::Equal {
                b.cmp(a)
            } else {
                diff
            }
        });

        if let Some(a) = v.pop() {
            assert!(a.1.len() == 0);
            print!("{}", a.0 as char);
            v.iter_mut().for_each(|(_, pre)| {
                pre.remove(&a.0);
            });
        } else {
            break;
        }
    }

    Ok(())
}
