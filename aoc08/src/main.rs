use std::fs;
use std::io;

#[derive(Debug)]
struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>,
    metasum: i32,
    value: i32,
}

impl Node {
    fn from_vec(v: &mut Vec<i32>) -> Node {
        let child_count = v.pop().unwrap();
        let meta_count = v.pop().unwrap();

        let mut children = Vec::new();
        let mut metasum = 0;
        let mut value = 0;

        for _ in 0..child_count {
            let child = Node::from_vec(v);
            metasum += child.metasum;
            children.push(child);
        }

        let mut meta = Vec::new();
        for _ in 0..meta_count {
            let m = v.pop().unwrap();
            metasum += m;
            meta.push(m);
        }

        if child_count == 0 {
            value = meta.iter().sum();
        } else {
            value = meta
                .iter()
                .filter_map(|x| {
                    if x == &0 {
                        None
                    } else if let Some(child) = children.get((x - 1) as usize) {
                        Some(child.value)
                    } else {
                        None
                    }
                }).sum();
        }

        Node {
            metadata: meta,
            children: children,
            metasum: metasum,
            value: value,
        }
    }
}

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input/input.txt")?;

    let mut series: Vec<i32> = s
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    series.reverse();
    let root = Node::from_vec(&mut series);
    println!("Part1: {}", root.metasum);
    println!("Part2: {}", root.value);

    Ok(())
}
