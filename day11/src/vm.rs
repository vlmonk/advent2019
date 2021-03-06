#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from_i64(input: u8) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            n => panic!("invalid mode: {}", n),
        }
    }
}

type ModeSet = (Mode, Mode, Mode);

struct Mem {
    raw: Vec<i64>,
    max_addr: usize,
}

impl Mem {
    pub fn set(&mut self, addr: usize, value: i64) {
        if addr > self.max_addr {
            self.raw.resize_with(addr + 1, Default::default);
            self.max_addr = addr;
        }
        self.raw[addr] = value;
    }

    pub fn get(&mut self, addr: usize) -> i64 {
        if addr > self.max_addr {
            self.raw.resize_with(addr + 1, Default::default);
            self.max_addr = addr;
        }
        self.raw[addr]
    }

    pub fn setup(programm: Vec<i64>) -> Self {
        let max_addr = programm.len() - 1;

        Self {
            raw: programm,
            max_addr,
        }
    }

    pub fn get_opcodes(&mut self, addr: usize) -> &[i64] {
        if (addr + 3) > self.max_addr {
            self.raw.resize_with(addr + 4, Default::default);
            self.max_addr = addr;
        }
        &self.raw[addr..]
    }
}

enum Command {
    Halt,
    Input(i64, ModeSet),
    Output(i64, ModeSet),
    Add(i64, i64, i64, ModeSet),
    Mul(i64, i64, i64, ModeSet),
    JumpTrue(i64, i64, ModeSet),
    JumpFalse(i64, i64, ModeSet),
    LessThan(i64, i64, i64, ModeSet),
    Equals(i64, i64, i64, ModeSet),
    UpdateRelative(i64, ModeSet),
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
            Self::UpdateRelative { .. } => 2,
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

pub struct CPUInfo {
    pub ticks: usize,
    pub addr: usize,
}

pub struct CPU<'a> {
    mem: Mem,
    ip: usize,
    ticks: usize,
    rb: i64,
    input: Option<Box<dyn FnMut() -> i64 + 'a>>,
    output: Option<Box<dyn FnMut(i64) + 'a>>,
}

impl<'a> CPU<'a> {
    fn new(programm: Vec<i64>) -> Self {
        let mem = Mem::setup(programm);

        Self {
            mem,
            ip: 0,
            ticks: 0,
            rb: 0,
            input: None,
            output: None,
        }
    }

    pub fn new_from_str(programm: &str) -> Self {
        let programm = parse_programm(programm);
        Self::new(programm)
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
            Command::Output(_, _) => State::Output,
            _ => State::Running,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.tick() == State::Halted {
                break;
            }
        }
    }

    fn process(&mut self, command: &Command) {
        match command {
            Command::Halt => {}
            Command::Input(addr, modeset) => {
                let value = match &mut self.input {
                    Some(x) => (x)(),
                    None => panic!("input not provided!"),
                };
                self.set_value(*addr, value, &modeset.0)
            }
            Command::Output(addr, modeset) => {
                let value = self.get_value(*addr as i64, &modeset.0);

                match &mut self.output {
                    Some(x) => (x)(value),
                    None => panic!("output not provided!"),
                };
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
            Command::UpdateRelative(value, modeset) => {
                let value = self.get_value(*value, &modeset.0);
                self.rb += value;
            }
        }
    }

    fn get_value(&mut self, addr: i64, mode_x: &Mode) -> i64 {
        match mode_x {
            Mode::Immediate => addr,
            Mode::Position => self.mem.get(addr as usize),
            Mode::Relative => self.mem.get((self.rb + addr) as usize),
        }
    }

    fn set_value(&mut self, addr: i64, value: i64, mode_x: &Mode) {
        match mode_x {
            Mode::Immediate => panic!("write with Mode::Immediate"),
            Mode::Position => self.mem.set(addr as usize, value),
            Mode::Relative => self.mem.set((self.rb + addr) as usize, value),
        }
    }

    pub fn input<F>(&mut self, f: F)
    where
        F: FnMut() -> i64 + 'a,
    {
        self.input = Some(Box::new(f))
    }

    pub fn output<F>(&mut self, f: F)
    where
        F: FnMut(i64) + 'a,
    {
        self.output = Some(Box::new(f))
    }

    pub fn info(&self) -> CPUInfo {
        CPUInfo {
            ticks: self.ticks,
            addr: self.mem.max_addr,
        }
    }
}

fn decode_opcode(input: i64) -> (i64, ModeSet) {
    let opcode = input % 100;
    let c = (input / 10_000) as u8 % 10;
    let b = (input / 1_000) as u8 % 10;
    let a = (input / 100) as u8 % 10;

    (
        opcode,
        (Mode::from_i64(a), Mode::from_i64(b), Mode::from_i64(c)),
    )
}

fn decode(mem: &[i64]) -> Command {
    let (opcode, modeset) = decode_opcode(mem[0]);
    match opcode {
        1 => Command::Add(mem[1], mem[2], mem[3], modeset),
        2 => Command::Mul(mem[1], mem[2], mem[3], modeset),
        3 => Command::Input(mem[1], modeset),
        4 => Command::Output(mem[1], modeset),
        5 => Command::JumpTrue(mem[1], mem[2], modeset),
        6 => Command::JumpFalse(mem[1], mem[2], modeset),
        7 => Command::LessThan(mem[1], mem[2], mem[3], modeset),
        8 => Command::Equals(mem[1], mem[2], mem[3], modeset),
        9 => Command::UpdateRelative(mem[1], modeset),
        99 => Command::Halt,
        n => panic!("invalid opcode: {}", n),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_halt() {
        let mut cpu = CPU::new(vec![99]);
        let state = cpu.tick();

        assert_eq!(state, State::Halted);
        assert_eq!(cpu.ip, 1);
    }

    #[test]
    fn test_write_to_memory() {
        let mut cpu = CPU::new(vec![3, 2, 1]);
        cpu.input(|| 42);
        let state = cpu.tick();

        assert_eq!(state, State::Running);
        assert_eq!(42, cpu.mem.get(2));
    }

    #[test]
    fn test_write_to_output() {
        let mut output = vec![];
        let mut cpu = CPU::new(vec![4, 2, 99]);
        cpu.output(|value| {
            output.push(value);
        });
        cpu.run();
        core::mem::drop(cpu);

        assert_eq!(vec![99], output);
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
        let mut cpu = CPU::new(programm);
        cpu.run();

        assert_eq!(3, cpu.mem.get(1));
    }

    #[test]
    fn test_big_num() {
        let mut output = vec![];
        let programm = "1102,34915192,34915192,7,4,7,99,0";
        let mut cpu = CPU::new_from_str(programm);
        cpu.output(|value| output.push(value));
        cpu.run();
        drop(cpu);

        assert_eq!(1219070632396864, output[0]);
    }

    #[test]
    fn test_relative_mode() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut output = vec![];
        let mut cpu = CPU::new(code.clone());
        cpu.output(|v| output.push(v));
        cpu.run();
        drop(cpu);

        assert_eq!(code, output);
    }
}
