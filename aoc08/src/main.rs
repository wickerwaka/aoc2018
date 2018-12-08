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
    fn from_iter<I>(v: &mut I) -> Result<Node, &'static str>
    where
        I: Iterator<Item = i32>,
    {
        let child_count = v.next().ok_or("missing child")?;
        let meta_count = v.next().ok_or("missing meta")?;

        let mut children = Vec::new();
        let mut metasum = 0;

        for _ in 0..child_count {
            let child = Node::from_iter(v)?;
            metasum += child.metasum;
            children.push(child);
        }

        let mut meta = Vec::new();
        for _ in 0..meta_count {
            let m = v.next().ok_or("short meta read")?;
            metasum += m;
            meta.push(m);
        }

        let value = match child_count {
            0 => meta.iter().sum(),
            _ => meta
                .iter()
                .filter_map(|x| {
                    if *x == 0 {
                        None
                    } else if let Some(child) = children.get((x - 1) as usize) {
                        Some(child.value)
                    } else {
                        None
                    }
                }).sum(),
        };

        Ok(Node {
            metadata: meta,
            children: children,
            metasum: metasum,
            value: value,
        })
    }
}

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input/input.txt")?;

    let mut series = s.split_whitespace().filter_map(|x| x.parse::<i32>().ok());
    let root = Node::from_iter(&mut series).unwrap();
    println!("Part1: {}", root.metasum);
    println!("Part2: {}", root.value);

    Ok(())
}
