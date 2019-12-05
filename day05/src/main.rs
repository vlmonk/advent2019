use std::fs;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from_i32(input: i32) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            n => panic!("invalid mode: {}", n),
        }
    }
}

#[derive(Debug)]
enum Command {
    Halt,
    Input(usize),
    Output(usize),
    Add {
        a: i32,
        b: i32,
        c: i32,
        mode_a: Mode,
        mode_b: Mode,
        mode_c: Mode,
    },
    Mul {
        a: i32,
        b: i32,
        c: i32,
        mode_a: Mode,
        mode_b: Mode,
        mode_c: Mode,
    },
}

impl Command {
    fn size(&self) -> usize {
        match self {
            Self::Halt => 1,
            Self::Input { .. } => 2,
            Self::Output { .. } => 2,
            Self::Add { .. } => 4,
            Self::Mul { .. } => 4,
        }
    }

    fn is_halt(&self) -> bool {
        match self {
            Command::Halt => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Running,
    Halted,
}

struct CPU {
    mem: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,

    ip: usize,
    ticks: usize,
}

impl CPU {
    fn new(programm: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            mem: programm,
            input,
            output: vec![],
            ip: 0,
            ticks: 0,
        }
    }
    fn tick(&mut self) -> State {
        let command = decode(&self.mem[self.ip..]);
        self.process(&command);
        self.ip += command.size();
        self.ticks += 1;

        if command.is_halt() {
            State::Halted
        } else {
            State::Running
        }
    }

    fn run(&mut self) {
        loop {
            if self.tick() == State::Halted {
                break;
            }
        }
    }

    fn process(&mut self, command: &Command) {
        match command {
            Command::Halt => {}
            Command::Input(addr) => {
                let value = self.input.remove(0);
                self.mem[*addr] = value;
            }
            Command::Output(addr) => {
                let value = self.mem[*addr];
                self.output.push(value);
            }
            Command::Add {
                a,
                b,
                c,
                mode_a,
                mode_b,
                ..
            } => {
                let a = self.get_value(a, mode_a);
                let b = self.get_value(b, mode_b);

                self.mem[*c as usize] = a + b;
            }
            Command::Mul {
                a,
                b,
                c,
                mode_a,
                mode_b,
                ..
            } => {
                let a = self.get_value(a, mode_a);
                let b = self.get_value(b, mode_b);

                self.mem[*c as usize] = a * b;
            }
        }
    }

    fn get_value(&self, x: &i32, mode_x: &Mode) -> i32 {
        match mode_x {
            Mode::Immediate => *x,
            Mode::Position => self.mem[*x as usize],
        }
    }
}

fn decode_opcode(input: i32) -> (i32, Mode, Mode, Mode) {
    let opcode = input % 100;
    let c = (input / 10_000) % 10;
    let b = (input / 1_000) % 10;
    let a = (input / 100) % 10;

    (
        opcode,
        Mode::from_i32(a),
        Mode::from_i32(b),
        Mode::from_i32(c),
    )
}

fn decode(mem: &[i32]) -> Command {
    let (opcode, mode_a, mode_b, mode_c) = decode_opcode(mem[0]);
    match opcode {
        1 => Command::Add {
            mode_a,
            mode_b,
            mode_c,
            a: mem[1],
            b: mem[2],
            c: mem[3],
        },
        2 => Command::Mul {
            mode_a,
            mode_b,
            mode_c,
            a: mem[1],
            b: mem[2],
            c: mem[3],
        },
        3 => Command::Input(mem[1] as usize),
        4 => Command::Output(mem[1] as usize),
        99 => Command::Halt,
        n => panic!("invalid opcode: {}", n),
    }
}

fn format_output(output: &[i32]) -> String {
    let inner = output
        .iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", inner)
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("can't read");
    let programm = raw
        .lines()
        .next()
        .expect("invalid input")
        .split(',')
        .map(|v| v.parse::<i32>().expect("invalid number"))
        .collect::<Vec<_>>();

    let input = vec![1];

    let mut cpu = CPU::new(programm, input);
    cpu.run();
    println!(
        "Output: {}, ticks: {}",
        format_output(&cpu.output),
        cpu.ticks
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_halt() {
        let mut cpu = CPU::new(vec![99], vec![]);
        let state = cpu.tick();

        assert_eq!(state, State::Halted);
        assert_eq!(cpu.ip, 1);
    }

    #[test]
    fn test_write_to_memory() {
        let mut cpu = CPU::new(vec![3, 2, 1], vec![42]);
        let state = cpu.tick();

        assert_eq!(state, State::Running);
        assert_eq!(42, cpu.mem[2]);
    }

    #[test]
    fn test_write_to_output() {
        let mut cpu = CPU::new(vec![4, 2, 99], vec![]);
        cpu.run();

        assert_eq!(vec![99], cpu.output);
    }

    #[test]
    fn test_decode_opcode() {
        let (opcode, a, b, c) = decode_opcode(10102);
        assert_eq!(2, opcode);
        assert_eq!(Mode::Immediate, a);
        assert_eq!(Mode::Position, b);
        assert_eq!(Mode::Immediate, c);
    }

    #[test]
    fn test_add() {
        let programm = vec![1101, 11, 22, 0, 101, -30, 0, 1, 99];
        let mut cpu = CPU::new(programm, vec![]);
        cpu.run();

        assert_eq!(cpu.mem[1], 3);
    }
}
