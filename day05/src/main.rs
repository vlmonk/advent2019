#[derive(Debug)]
enum Command {
    Halt,
    Input { addr: usize },
    Output { addr: usize },
}

impl Command {
    fn size(&self) -> usize {
        match self {
            Self::Halt => 1,
            Self::Input { .. } => 2,
            Self::Output { .. } => 2,
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
}

impl CPU {
    fn new(programm: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            mem: programm,
            input,
            output: vec![],
            ip: 0,
        }
    }
    fn tick(&mut self) -> State {
        let command = decode(&self.mem[self.ip..]);
        self.process(&command);

        self.ip += command.size();

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
            Command::Input { addr } => {
                let value = self.input.remove(0);
                self.mem[*addr] = value;
            }
            Command::Output { addr } => {
                let value = self.mem[*addr];
                self.output.push(value);
            }
        }
    }
}

fn decode(mem: &[i32]) -> Command {
    match mem[0] {
        3 => Command::Input {
            addr: mem[1] as usize,
        },

        4 => Command::Output {
            addr: mem[1] as usize,
        },

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
    let mut cpu = CPU::new(vec![99], vec![]);
    cpu.run();
    println!("Output: {}", format_output(&cpu.output));
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
}
