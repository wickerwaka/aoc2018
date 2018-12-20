struct Grid {
    cells: Vec<i64>,
    width: usize,
}

impl Grid {
    fn new(width: usize, height: usize, serial: i64) -> Grid {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let rack_id = (x + 11) as i64;
                let level = ((rack_id * (y + 1) as i64) + serial) * rack_id;
                let power = ((level.abs() / 100) % 10) - 5;
                cells.push(power);
            }
        }
        Grid { cells, width }
    }

    fn cell_power(&self, x: usize, y: usize) -> Option<i64> {
        if x >= self.width {
            None
        } else {
            self.cells.get(x + (y * self.width)).cloned()
        }
    }

    fn quad_power(&self, x: usize, y: usize, sz: usize) -> Option<i64> {
        let mut total = 0;
        for yy in y..y + sz {
            for xx in x..x + sz {
                total += self.cell_power(xx, yy)?;
            }
        }
        Some(total)
    }
}

#[derive(Debug)]
struct QuadPower {
    x: usize,
    y: usize,
    sz: usize,
    power: i64,
}

fn main() {
    let grid = Grid::new(300, 300, 9435);
    let mut results = Vec::new();
    for q in 1..301 {
        for y in 0..300 - q {
            for x in 0..300 - q {
                if let Some(power) = grid.quad_power(x, y, q) {
                    results.push(QuadPower {
                        x: x + 1,
                        y: y + 1,
                        sz: q,
                        power: power,
                    });
                }
            }
        }
    }

    let part1 = results
        .iter()
        .filter(|x| x.sz == 3)
        .max_by_key(|quad| quad.power)
        .unwrap();
    let part2 = results.iter().max_by_key(|quad| quad.power).unwrap();

    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
