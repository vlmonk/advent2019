use std::fs;
use vm::CPU;

mod vm;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Step {
    Nord,
    East,
    South,
    West,
}

impl Step {
    pub fn to_i64(self) -> i64 {
        match self {
            Self::Nord => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }

    pub fn back(self) -> Self {
        match self {
            Self::Nord => Self::South,
            Self::East => Self::West,
            Self::South => Self::Nord,
            Self::West => Self::East,
        }
    }
}

pub fn all_step() -> impl Iterator<Item = Step> {
    vec![Step::Nord, Step::East, Step::South, Step::West].into_iter()
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

#[derive(Debug, Copy, Clone)]
enum NextStep {
    MoveTo(Step),
    BackTo(Step),
}

struct Robot {
    cpu: CPU,
}

impl Robot {
    pub fn new(input: &str) -> Self {
        let cpu = CPU::new_from_str(input);
        Self { cpu }
    }

    pub fn solve_a(&mut self) -> i64 {
        let mut steps = 0;
        let mut next_moves = vec![];

        for step in all_step() {
            next_moves.push(NextStep::MoveTo(step));
        }

        loop {
            let next_move = next_moves.pop();
            dbg!(next_move);

            match next_move {
                Some(NextStep::MoveTo(step)) => {
                    let r = self.walk(step);
                    dbg!(&r);

                    match r {
                        StepResult::Wall => {
                            println!("WALL FOUND");
                        }
                        StepResult::Moved => {
                            println!("MOVED");
                            let back = step.back();
                            steps += 1;
                            next_moves.push(NextStep::BackTo(back));

                            for step in all_step() {
                                if step != back {
                                    next_moves.push(NextStep::MoveTo(step));
                                }
                            }
                        }

                        StepResult::MovedToOxygen => {
                            println!("OXYGEN FOUND");
                            return steps;
                        }
                    }
                }
                Some(NextStep::BackTo(step)) => {
                    println!("Back to {:?}", step);
                    self.walk(step);
                }
                None => panic!("out of moves"),
            }
        }
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

    let r = robot.solve_a();
    dbg!(r);

    Ok(())
}
