use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::io;

use std::error::Error;

use std::str::FromStr;

#[derive(Debug, Clone)]
enum SegmentType {
    Empty,
    Horizontal,
    Vertical,
    Intersection,
    UpRight,
    UpLeft,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&self) -> Direction {
        self.turn_left().invert()
    }

    fn up_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Up => Direction::Right,
        }
    }

    fn up_left(&self) -> Direction {
        self.up_right().invert()
    }
}

#[derive(Debug, Clone)]
enum Action {
    TurnLeft,
    Straight,
    TurnRight,
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_action: Action,
    removed: bool,
}

impl Cart {
    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            x,
            y,
            direction,
            next_action: Action::TurnLeft,
            removed: false,
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn rotate(&mut self, seg: &SegmentType) {
        self.direction = match seg {
            SegmentType::UpLeft => self.direction.up_left(),
            SegmentType::UpRight => self.direction.up_right(),
            SegmentType::Intersection => match self.next_action {
                Action::TurnLeft => {
                    self.next_action = Action::Straight;
                    self.direction.turn_left()
                }
                Action::Straight => {
                    self.next_action = Action::TurnRight;
                    self.direction.clone()
                }
                Action::TurnRight => {
                    self.next_action = Action::TurnLeft;
                    self.direction.turn_right()
                }
            },
            SegmentType::Empty => panic!("empty segment"),
            _ => self.direction.clone(),
        };
    }

    fn do_move(&mut self) {
        match self.direction {
            Direction::Down => self.y += 1,
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

enum Input {
    Segment(SegmentType),
    Cart(Direction, SegmentType),
}

impl From<char> for Input {
    fn from(ch: char) -> Self {
        match ch {
            '|' => Input::Segment(SegmentType::Vertical),
            '-' => Input::Segment(SegmentType::Horizontal),
            '/' => Input::Segment(SegmentType::UpRight),
            '\\' => Input::Segment(SegmentType::UpLeft),
            '+' => Input::Segment(SegmentType::Intersection),
            '<' => Input::Cart(Direction::Left, SegmentType::Horizontal),
            '>' => Input::Cart(Direction::Right, SegmentType::Horizontal),
            '^' => Input::Cart(Direction::Up, SegmentType::Vertical),
            'v' => Input::Cart(Direction::Down, SegmentType::Vertical),
            _ => Input::Segment(SegmentType::Empty),
        }
    }
}

#[derive(Clone)]
struct System {
    width: usize,
    height: usize,
    grid: Vec<SegmentType>,
    carts: Vec<Cart>,
}

impl FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let height = lines.len();
        let width = lines.iter().map(|x| x.len()).max().unwrap();
        let mut grid = vec![SegmentType::Empty; width * height];
        let mut carts = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let index = (y * width) + x;
                grid[index] = match Input::from(ch) {
                    Input::Segment(ty) => ty,
                    Input::Cart(d, ty) => {
                        carts.push(Cart::new(x, y, d.clone()));
                        ty
                    }
                }
            }
        }

        Ok(System {
            width,
            height,
            grid,
            carts,
        })
    }
}

impl System {
    fn get_segment(&self, x: usize, y: usize) -> Option<&SegmentType> {
        let idx = (y * self.width) + x;
        self.grid.get(idx)
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut carts = self.carts.clone();
        carts.sort_by_key(|x| x.position());

        let num_carts = carts.len();
        for idx in 0..num_carts {
            let new_pos = {
                let mut cart = carts.get_mut(idx).unwrap();
                let segment = self.get_segment(cart.x, cart.y).unwrap();
                cart.rotate(segment);
                cart.do_move();
                cart.position().clone()
            };

            let dups = carts.iter().filter(|x| x.position() == new_pos).count();
            if dups > 1 {
                return Some(new_pos);
            }
        }

        std::mem::swap(&mut self.carts, &mut carts);

        None
    }

    fn tick_with_cleanup(&mut self) {
        let mut carts = self.carts.clone();
        carts.sort_by_key(|x| x.position());

        let num_carts = carts.len();
        for idx in 0..num_carts {
            let new_pos = {
                let mut cart = carts.get_mut(idx).unwrap();
                let segment = self.get_segment(cart.x, cart.y).unwrap();
                cart.rotate(segment);
                cart.do_move();
                cart.position().clone()
            };

            let dups = carts
                .iter()
                .filter(|x| (x.position() == new_pos) && (x.removed == false))
                .count();
            if dups > 1 {
                for cart in carts.iter_mut() {
                    if cart.position() == new_pos {
                        cart.removed = true;
                    }
                }
            }
        }

        carts.retain(|x| x.removed == false);
        std::mem::swap(&mut self.carts, &mut carts);
    }
}
fn main() -> Result<(), Box<Error>> {
    let s = fs::read_to_string("input/input.txt")?;

    let mut system1 = System::from_str(&s)?;
    let mut system2 = system1.clone();

    println!("{} x {}", system1.width, system1.height);

    loop {
        //println!("Tick");
        let collision = system1.tick();
        if collision.is_some() {
            println!("{:?}", collision);
            break;
        }
    }

    while system2.carts.len() > 1 {
        system2.tick_with_cleanup();
    }

    println!("{:?}", system2.carts.get(0));

    Ok(())
}
