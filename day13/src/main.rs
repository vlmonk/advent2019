use crate::vm::{CPU, IO};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::mem;
use std::thread;
use std::time;
use termion::{clear, color, cursor};

enum Triplet<T>
where
    T: Copy,
{
    None,
    One(T),
    Two(T, T),
}

impl<T> Triplet<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self::None
    }

    pub fn add(&mut self, value: T) -> Option<(T, T, T)> {
        let result = match self {
            Self::Two(a, b) => Some((*a, *b, value)),
            Self::One(_) | Self::None => None,
        };

        let replace = match self {
            Self::None => Triplet::One(value),
            Self::One(a) => Triplet::Two(*a, value),
            Self::Two(_, _) => Triplet::None,
        };

        mem::replace(self, replace);
        result
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    pub fn from_i64(n: i64) -> Self {
        use Tile::*;

        match n {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("invalid input"),
        }
    }

    pub fn is_block(&self) -> bool {
        self == &Self::Block
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Tile::*;
        match self {
            Empty => write!(fmt, " "),
            Wall => write!(
                fmt,
                "{}█{}",
                color::Fg(color::Yellow),
                color::Fg(color::Reset)
            ),
            Block => write!(fmt, "▄"),
            Paddle => write!(fmt, "↑"),
            Ball => write!(fmt, "⊗"),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Game {
    input: String,
    field: Field,
}

impl Game {
    pub fn new(input: &str) -> Self {
        Self {
            field: Field::new(),
            input: input.to_owned(),
        }
    }

    pub fn block_num(&mut self) -> usize {
        let mut cpu = CPU::new_from_str(&self.input);
        let mut output = vec![];
        let io = IO::output(|value| output.push(value));

        cpu.run(io);

        for chunk in output.chunks(3) {
            let coord = Coord::new(chunk[0], chunk[1]);
            let tile = Tile::from_i64(chunk[2]);
            self.field.insert(coord, tile);
        }

        self.field.block_num()
    }

    pub fn final_score(&mut self) -> i64 {
        let mut triplet = Triplet::new();
        let mut cpu = CPU::new_from_str(&self.input);
        cpu.set_mem(0, 2);

        let draw = |x: i64, y: i64, c: i64| {
            let goto = cursor::Goto(x as u16 + 1, y as u16 + 1);
            let tile = Tile::from_i64(c);

            println!("{}{}", goto, tile);
        };

        let show_score = |v: i64| {
            let goto = cursor::Goto(40, 0);
            println!("{}Score:        ", goto);
            let goto = cursor::Goto(40, 0);
            println!("{}Score: {}", goto, v)
        };

        let sleep_interval = time::Duration::from_millis(300);

        let input = || {
            thread::sleep(sleep_interval);
            0
        };

        let output = |value| match triplet.add(value) {
            Some((x, y, v)) if x == -1 && y == 0 => show_score(v),
            Some((x, y, c)) => draw(x, y, c),
            None => {}
        };

        let io = IO::new(input, output);

        cpu.run(io);

        0
    }
}

struct Field {
    data: HashMap<Coord, Tile>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, coord: Coord, tile: Tile) {
        self.data.insert(coord, tile);
    }

    pub fn block_num(&self) -> usize {
        self.data.values().filter(|field| field.is_block()).count()
    }
}

mod vm;

fn main() {
    println!("{}", clear::All);

    let input = fs::read_to_string("input.txt").expect("cant' read input.txt");
    let mut game = Game::new(&input);

    let task_1 = game.block_num();
    let task_2 = game.final_score();

    println!("Task I : {}", task_1);
    println!("Task II: {}", task_2);
}
