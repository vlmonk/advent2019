use crate::vm::{CPU, IO};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::mem;
use std::thread;
use std::time;
use termion::{clear, color, cursor};

enum Triplet {
    None,
    One(i64),
    Two(i64, i64),
}

fn convert(a: i64, b: i64, c: i64) -> Output {
    if a == -1 && b == 0 {
        Output::ScoreInfo(c)
    } else {
        let coord = Coord::new(a, b);
        let tile = Tile::from_i64(c);
        Output::BlockInfo(coord, tile)
    }
}

impl Triplet {
    pub fn new() -> Self {
        Self::None
    }

    pub fn add(&mut self, value: i64) -> Option<Output> {
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
        result.map(|(a, b, c)| convert(a, b, c))
    }
}

#[derive(PartialEq, Copy, Clone)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

enum Output {
    BlockInfo(Coord, Tile),
    ScoreInfo(i64),
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

        let draw = |coord: Coord, tile| {
            let goto = cursor::Goto(coord.x as u16 + 1, coord.y as u16 + 1);
            println!("{}{}", goto, tile);
        };

        let show_score = |v: i64| {
            let goto = cursor::Goto(40, 0);
            println!("{}Score:        ", goto);
            let goto = cursor::Goto(40, 0);
            println!("{}Score: {}", goto, v)
        };

        let sleep_interval = time::Duration::from_millis(40);

        let input = || {
            thread::sleep(sleep_interval);
            self.field.predict()
        };

        let output = |value| match triplet.add(value) {
            Some(Output::ScoreInfo(v)) => show_score(v),
            Some(Output::BlockInfo(coord, tile)) => {
                draw(coord, tile);
                self.field.insert(coord, tile);
            }
            None => {}
        };

        let io = IO::new(input, output);

        cpu.run(io);

        0
    }
}

struct Field {
    data: RefCell<HashMap<Coord, Tile>>,
    ball: RefCell<Option<Coord>>,
    paddle: RefCell<Option<Coord>>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            ball: RefCell::new(None),
            paddle: RefCell::new(None),
        }
    }

    pub fn insert(&self, coord: Coord, tile: Tile) {
        self.data.borrow_mut().insert(coord, tile);

        if let Tile::Ball = tile {
            self.ball.replace(Some(coord));
        }

        if let Tile::Paddle = tile {
            self.paddle.replace(Some(coord));
        }
    }

    pub fn block_num(&self) -> usize {
        self.data
            .borrow()
            .values()
            .filter(|field| field.is_block())
            .count()
    }

    pub fn predict(&self) -> i64 {
        let ball = *self.ball.borrow();
        let paddle = *self.paddle.borrow();

        match (ball, paddle) {
            (Some(ball), Some(paddle)) => predict_by_coord(ball, paddle),
            _ => 0,
        }
    }
}

fn predict_by_coord(ball: Coord, paddle: Coord) -> i64 {
    match (ball.x, paddle.x) {
        (ball, paddle) if ball > paddle => 1,
        (ball, paddle) if ball < paddle => -1,
        _ => 0,
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
