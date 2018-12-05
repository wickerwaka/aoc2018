use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut v = Vec::new();
    io::stdin().read_to_end(&mut v)?;
    v.retain(|&x| x >= 65); // remove anything < 'A'

    {
        let mut basic = v.clone();
        react_polymer(&mut basic);
        println!("Part1 {}", basic.len());
    }

    let mut h = HashSet::new();
    for x in &v {
        let c = x | 0x20;
        h.insert(c);
    }

    let mut lengths = Vec::new();

    for to_remove in &h {
        let mut reduced = v
            .iter()
            .cloned()
            .filter(|&x| (x | 0x20) != *to_remove)
            .collect();
        react_polymer(&mut reduced);
        lengths.push(reduced.len());
        println!("Part2.1 {}", reduced.len());
    }

    println!("Part2.2 {}", lengths.iter().min().unwrap());

    Ok(())
}

fn react_polymer(v: &mut Vec<u8>) {
    loop {
        let mut write_ofs: usize = 0;
        let mut read_a_ofs: usize = 0;
        let mut read_b_ofs: usize = 1;
        while read_b_ofs < v.len() {
            let b = *v.get(read_b_ofs).unwrap();
            let a = *v.get(read_a_ofs).unwrap();
            if (a != b) && ((a | 0x20) == (b | 0x20)) {
                read_a_ofs += 2;
                read_b_ofs += 2;
            } else {
                v.swap(write_ofs, read_a_ofs);
                write_ofs += 1;
                read_a_ofs += 1;
                read_b_ofs += 1;
            }
        }
        if read_a_ofs < v.len() {
            v.swap(write_ofs, read_a_ofs);
            write_ofs += 1;
        }

        if write_ofs == v.len() {
            break;
        }
        v.truncate(write_ofs);
    }
}
