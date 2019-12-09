use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from_i64(input: i64) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            n => panic!("invalid mode: {}", n),
        }
    }
}

type ModeSet = (Mode, Mode, Mode);

struct Mem {
    raw: HashMap<i64, i64>,
}

impl Mem {
    pub fn set(&mut self, addr: i64, value: i64) {
        self.raw.insert(addr, value);
    }

    pub fn get(&self, addr: i64) -> i64 {
        self.raw.get(&addr).map(|v| *v).unwrap_or(0)
    }

    pub fn setup(programm: &[i64]) -> Self {
        let mut raw = HashMap::new();

        for (addr, value) in programm.iter().enumerate() {
            raw.insert(addr as i64, *value);
        }

        Self { raw }
    }

    pub fn get_opcodes(&self, addr: usize) -> [i64; 4] {
        let addr = addr as i64;
        [
            self.get(addr),
            self.get(addr + 1),
            self.get(addr + 2),
            self.get(addr + 3),
        ]
    }
}

#[derive(Debug)]
enum Command {
    Halt,
    Input(usize),
    Output(usize),
    Add(i64, i64, i64, ModeSet),
    Mul(i64, i64, i64, ModeSet),
    JumpTrue(i64, i64, ModeSet),
    JumpFalse(i64, i64, ModeSet),
    LessThan(i64, i64, i64, ModeSet),
    Equals(i64, i64, i64, ModeSet),
}

impl Command {
    fn size(&self) -> usize {
        match self {
            Self::Halt => 1,
            Self::Input { .. } => 2,
            Self::Output { .. } => 2,
            Self::Add { .. } => 4,
            Self::Mul { .. } => 4,
            Self::JumpTrue { .. } => 3,
            Self::JumpFalse { .. } => 3,
            Self::LessThan { .. } => 4,
            Self::Equals { .. } => 4,
        }
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Running,
    Halted,
    Output,
}

fn parse_programm(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .expect("invalid input")
        .split(',')
        .map(|v| v.parse::<i64>().expect("invalid number"))
        .collect::<Vec<_>>()
}

pub struct CPU {
    mem: Mem,
    input: Vec<i64>,
    pub output: Vec<i64>,

    ip: usize,
    ticks: usize,

    rb: i64,
}

impl CPU {
    pub fn new(programm: Vec<i64>, input: Vec<i64>) -> Self {
        let mem = Mem::setup(&programm);

        Self {
            mem,
            input,
            output: vec![],
            ip: 0,
            ticks: 0,
            rb: 0,
        }
    }

    pub fn new_from_str(programm: &str, input: Vec<i64>) -> Self {
        let programm = parse_programm(programm);
        Self::new(programm, input)
    }

    fn tick(&mut self) -> State {
        let original_ip = self.ip;

        let command = decode(self.mem.get_opcodes(self.ip));
        self.process(&command);
        self.ticks += 1;

        if self.ip == original_ip {
            self.ip += command.size();
        }

        match command {
            Command::Halt => State::Halted,
            Command::Output(_) => State::Output,
            _ => State::Running,
        }
    }

    pub fn push(&mut self, i: i64) {
        self.input.push(i);
    }

    pub fn run(&mut self) {
        loop {
            if self.tick() == State::Halted {
                break;
            }
        }
    }

    pub fn run_part(&mut self) -> Option<i64> {
        loop {
            match self.tick() {
                State::Halted => return None,
                State::Output => return self.output.pop(),
                _ => {}
            }
        }
    }

    fn process(&mut self, command: &Command) {
        match command {
            Command::Halt => {}
            Command::Input(addr) => {
                let value = self.input.remove(0);
                self.mem.set(*addr as i64, value);
            }
            Command::Output(addr) => {
                let value = self.get_value(*addr as i64, &Mode::Position);
                self.output.push(value);
            }
            Command::Add(a, b, c, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);
                self.set_value(*c, a + b, &modeset.2)
            }
            Command::Mul(a, b, c, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);
                self.set_value(*c, a * b, &modeset.2)
            }
            Command::JumpTrue(a, b, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);

                if a != 0 {
                    self.ip = b as usize;
                }
            }
            Command::JumpFalse(a, b, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);

                if a == 0 {
                    self.ip = b as usize;
                }
            }
            Command::LessThan(a, b, c, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);

                if a < b {
                    self.set_value(*c, 1, &modeset.2)
                } else {
                    self.set_value(*c, 0, &modeset.2)
                }
            }
            Command::Equals(a, b, c, modeset) => {
                let a = self.get_value(*a, &modeset.0);
                let b = self.get_value(*b, &modeset.1);

                if a == b {
                    self.set_value(*c, 1, &modeset.2)
                } else {
                    self.set_value(*c, 0, &modeset.2)
                }
            }
        }
    }

    fn get_value(&self, value: i64, mode_x: &Mode) -> i64 {
        match mode_x {
            Mode::Immediate => value,
            Mode::Position => self.mem.get(value),
        }
    }

    fn set_value(&mut self, addr: i64, value: i64, mode_x: &Mode) {
        match mode_x {
            Mode::Immediate => panic!("write with Mode::Immediate"),
            Mode::Position => self.mem.set(addr, value),
        }
    }
}

fn decode_opcode(input: i64) -> (i64, ModeSet) {
    let opcode = input % 100;
    let c = (input / 10_000) % 10;
    let b = (input / 1_000) % 10;
    let a = (input / 100) % 10;

    (
        opcode,
        (Mode::from_i64(a), Mode::from_i64(b), Mode::from_i64(c)),
    )
}

fn decode(mem: [i64; 4]) -> Command {
    let (opcode, modeset) = decode_opcode(mem[0]);
    match opcode {
        1 => Command::Add(mem[1], mem[2], mem[3], modeset),
        2 => Command::Mul(mem[1], mem[2], mem[3], modeset),
        3 => Command::Input(mem[1] as usize),
        4 => Command::Output(mem[1] as usize),
        5 => Command::JumpTrue(mem[1], mem[2], modeset),
        6 => Command::JumpFalse(mem[1], mem[2], modeset),
        7 => Command::LessThan(mem[1], mem[2], mem[3], modeset),
        8 => Command::Equals(mem[1], mem[2], mem[3], modeset),
        99 => Command::Halt,
        n => panic!("invalid opcode: {}", n),
    }
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
        assert_eq!(42, cpu.mem.get(2));
    }

    #[test]
    fn test_write_to_output() {
        let mut cpu = CPU::new(vec![4, 2, 99], vec![]);
        cpu.run();

        assert_eq!(vec![99], cpu.output);
    }

    #[test]
    fn test_decode_opcode() {
        let (opcode, modeset) = decode_opcode(10102);
        assert_eq!(2, opcode);
        assert_eq!((Mode::Immediate, Mode::Position, Mode::Immediate), modeset);
    }

    #[test]
    fn test_add() {
        let programm = vec![1101, 11, 22, 0, 101, -30, 0, 1, 99];
        let mut cpu = CPU::new(programm, vec![]);
        cpu.run();

        assert_eq!(3, cpu.mem.get(1));
    }

    #[test]
    fn test_big_num() {
        let programm = "1102,34915192,34915192,7,4,7,99,0";
        let mut cpu = CPU::new_from_str(programm, vec![]);
        cpu.run();

        assert_eq!(1219070632396864, cpu.output[0]);
    }
}
