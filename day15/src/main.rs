use std::fs;
use vm::CPU;

mod vm;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

enum Step {
    Nord,
    East,
    South,
    West,
}

impl Step {
    pub fn to_i64(&self) -> i64 {
        match self {
            Self::Nord => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}

#[derive(Debug)]
enum StepResult {
    Wall,
    Moved,
    MovedToOxygen,
}

impl StepResult {
    pub fn from_i64(i: i64) -> Self {
        match i {
            0 => Self::Wall,
            1 => Self::Moved,
            2 => Self::MovedToOxygen,
            _ => panic!("invalid value"),
        }
    }
}

struct Robot {
    cpu: CPU,
}

impl Robot {
    pub fn new(input: &str) -> Self {
        let cpu = CPU::new_from_str(input);
        Self { cpu }
    }

    pub fn walk(&mut self, step: Step) -> StepResult {
        let input = || step.to_i64();
        let output = self.cpu.run_till_output(input).unwrap();
        StepResult::from_i64(output)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut robot = Robot::new(&input);

    let r = robot.walk(Step::Nord);
    dbg!(r);

    Ok(())
}
