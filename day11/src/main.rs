mod vm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::hash::Hash;
use vm::CPU;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("invalid value"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, " "),
            Self::White => write!(f, "█"),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Self::Up => Self::Left,
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
            },
            Turn::Right => match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            },
        }
    }
}

enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("invalid value"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    pub fn walk(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Coord(self.0, self.1 + 1),
            Direction::Right => Coord(self.0 + 1, self.1),
            Direction::Down => Coord(self.0, self.1 - 1),
            Direction::Left => Coord(self.0 - 1, self.1),
        }
    }
}

struct GameState {
    field: HashMap<Coord, Color>,
    position: Coord,
    current_output: Option<i64>,
    direction: Direction,
}

impl GameState {
    pub fn feed(&mut self, b: i64) -> Option<(i64, i64)> {
        match self.current_output {
            Some(a) => {
                self.current_output = None;
                Some((a, b))
            }
            _ => {
                self.current_output = Some(b);
                None
            }
        }
    }

    pub fn paint(&mut self, color: Color) {
        self.field.insert(self.position, color);
    }

    pub fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    pub fn walk(&mut self) {
        self.position = self.position.walk(&self.direction)
    }
}

struct Game {
    programm: String,
    state: RefCell<GameState>,
}

impl Game {
    pub fn new(programm: &str, color: Color) -> Self {
        let mut state = GameState {
            field: HashMap::new(),
            position: Coord(0, 0),
            current_output: None,
            direction: Direction::Up,
        };

        state.paint(color);

        Self {
            state: RefCell::new(state),
            programm: programm.into(),
        }
    }

    pub fn run(&mut self) {
        let mut cpu = CPU::new_from_str(&self.programm);
        cpu.input(|| self.handle_input());
        cpu.output(|v| self.handle_output(v));
        cpu.run();
    }

    pub fn total_painted(&self) -> usize {
        self.state.borrow().field.len()
    }

    pub fn field(self) -> Field {
        Field(self.state.into_inner().field)
    }

    fn handle_input(&self) -> i64 {
        let state = self.state.borrow();

        let current = state.field.get(&state.position).unwrap_or(&Color::Black);
        match current {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn handle_output(&self, value: i64) {
        let mut state = self.state.borrow_mut();
        if let Some((color, turn)) = state.feed(value) {
            state.paint(color.into());
            state.turn(turn.into());
            state.walk();
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field: {:?}", self.state.borrow().field)
    }
}

struct Field(HashMap<Coord, Color>);

impl Field {
    fn x(&self) -> impl Iterator<Item = i32> {
        let x_min = self.0.keys().map(|k| k.0).min().unwrap_or(0);
        let x_max = self.0.keys().map(|k| k.0).max().unwrap_or(0);

        (x_min..=x_max)
    }
    fn y(&self) -> impl Iterator<Item = i32> {
        let y_min = self.0.keys().map(|k| k.1).min().unwrap_or(0);
        let y_max = self.0.keys().map(|k| k.1).max().unwrap_or(0);

        (y_min..=y_max).rev()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.y() {
            for x in self.x() {
                let color = self.0.get(&Coord(x, y)).unwrap_or(&Color::Black);
                write!(f, "{}", color)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut game_a = Game::new(&input, Color::Black);
    game_a.run();
    let task_a = game_a.total_painted();

    let mut game_b = Game::new(&input, Color::White);
    game_b.run();
    let task_b = game_b.field();

    println!("Task I:  {}", task_a);
    println!("Task II:\n{}", task_b);

    Ok(())
}
