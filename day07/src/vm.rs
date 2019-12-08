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

type ModeSet = (Mode, Mode, Mode);

#[derive(Debug)]
enum Command {
    Halt,
    Input(usize),
    Output(usize),
    Add(i32, i32, i32, ModeSet),
    Mul(i32, i32, i32, ModeSet),
    JumpTrue(i32, i32, ModeSet),
    JumpFalse(i32, i32, ModeSet),
    LessThan(i32, i32, i32, ModeSet),
    Equals(i32, i32, i32, ModeSet),
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

pub struct CPU {
    mem: Vec<i32>,
    input: Vec<i32>,
    pub output: Vec<i32>,

    ip: usize,
    ticks: usize,
}

impl CPU {
    pub fn new(programm: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            mem: programm,
            input,
            output: vec![],
            ip: 0,
            ticks: 0,
        }
    }
    fn tick(&mut self) -> State {
        let original_ip = self.ip;

        let command = decode(&self.mem[self.ip..]);
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

    pub fn push(&mut self, i: i32) {
        self.input.push(i);
    }

    pub fn run(&mut self) {
        loop {
            if self.tick() == State::Halted {
                break;
            }
        }
    }

    pub fn run_part(&mut self) -> Option<i32> {
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
                self.mem[*addr] = value;
            }
            Command::Output(addr) => {
                let value = self.mem[*addr];
                self.output.push(value);
            }
            Command::Add(a, b, c, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                self.mem[*c as usize] = a + b;
            }
            Command::Mul(a, b, c, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                self.mem[*c as usize] = a * b;
            }
            Command::JumpTrue(a, b, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                if a != 0 {
                    self.ip = b as usize;
                }
            }
            Command::JumpFalse(a, b, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                if a == 0 {
                    self.ip = b as usize;
                }
            }
            Command::LessThan(a, b, c, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                if a < b {
                    self.mem[*c as usize] = 1
                } else {
                    self.mem[*c as usize] = 0
                }
            }
            Command::Equals(a, b, c, modeset) => {
                let a = self.get_value(a, &modeset.0);
                let b = self.get_value(b, &modeset.1);

                if a == b {
                    self.mem[*c as usize] = 1
                } else {
                    self.mem[*c as usize] = 0
                }
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

fn decode_opcode(input: i32) -> (i32, ModeSet) {
    let opcode = input % 100;
    let c = (input / 10_000) % 10;
    let b = (input / 1_000) % 10;
    let a = (input / 100) % 10;

    (
        opcode,
        (Mode::from_i32(a), Mode::from_i32(b), Mode::from_i32(c)),
    )
}

fn decode(mem: &[i32]) -> Command {
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
        let (opcode, modeset) = decode_opcode(10102);
        assert_eq!(2, opcode);
        assert_eq!((Mode::Immediate, Mode::Position, Mode::Immediate), modeset);
    }

    #[test]
    fn test_add() {
        let programm = vec![1101, 11, 22, 0, 101, -30, 0, 1, 99];
        let mut cpu = CPU::new(programm, vec![]);
        cpu.run();

        assert_eq!(cpu.mem[1], 3);
    }
}
