use crate::vm::{CPU, IO};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

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
    field: Field,
    cpu: CPU,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let cpu = CPU::new_from_str(input);
        let field = Field::new();

        Self { field, cpu }
    }

    pub fn block_num(&mut self) -> usize {
        let mut output = vec![];
        let io = IO::output(|value| output.push(value));
        self.cpu.run(io);

        for chunk in output.chunks(3) {
            let coord = Coord::new(chunk[0], chunk[1]);
            let tile = Tile::from_i64(chunk[2]);
            self.field.insert(coord, tile);
        }
        self.field.block_num()
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
    let input = fs::read_to_string("input.txt").expect("cant' read input.txt");
    let mut game = Game::new(&input);
    let task_1 = game.block_num();

    println!("Task I: {}", task_1);
}
