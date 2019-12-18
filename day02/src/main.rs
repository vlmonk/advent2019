use std::error::Error;
use std::fmt;
use std::fs;
use std::time::Instant;

#[derive(PartialEq, Debug, Clone)]
struct Code {
    data: Vec<i32>,
}

impl Code {
    pub fn parse(input: &str) -> Result<Code, Box<dyn Error>> {
        input
            .split(',')
            .map(|num| num.trim_matches('\n'))
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|data| Code { data })
            .map_err(|e| e.into())
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self
            .data
            .iter()
            .map(|n| format!("{}", n))
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "[{}]", x)
    }
}

#[derive(PartialEq, Debug)]
enum StepResult {
    Next,
    Done,
    Error,
}

fn step(input: &mut Code, pos: usize) -> StepResult {
    let code = input.data[pos];
    match code {
        99 => StepResult::Done,
        1 => {
            let a = input.data[pos + 1] as usize;
            let b = input.data[pos + 2] as usize;
            let res = input.data[pos + 3] as usize;

            input.data[res] = input.data[a] + input.data[b];
            StepResult::Next
        }

        2 => {
            let a = input.data[pos + 1] as usize;
            let b = input.data[pos + 2] as usize;
            let res = input.data[pos + 3] as usize;

            input.data[res] = input.data[a] * input.data[b];
            StepResult::Next
        }
        _ => StepResult::Error,
    }
}

fn run(input: &mut Code, a: i32, b: i32) -> i32 {
    let mut pos = 0;

    input.data[1] = a;
    input.data[2] = b;

    loop {
        match step(input, pos) {
            StepResult::Done => break,
            StepResult::Error => {
                println!("Step error");
                break;
            }
            StepResult::Next => pos = pos + 4,
        }
    }

    input.data[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt")?;
    let code = Code::parse(&input)?;
    let mut code2 = code.clone();
    let r01 = run(&mut code2, 12, 2);

    let r02 = (0..100)
        .rev()
        .flat_map(|a| (0..100).rev().map(move |b| (a, b)))
        .find(|(a, b)| {
            code2.data.clone_from_slice(&code.data);
            run(&mut code2, *a, *b) == 19690720
        });

    let total_time = now.elapsed();

    println!("Q1: {}", r01);
    match r02 {
        Some((a, b)) => println!("Q2: {}", a * 100 + b),
        None => println!("Q2: not found"),
    }
    println!("Total time: {}μs", total_time.as_micros());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let code = Code::parse("1,9,10,3").unwrap();
        let expected = Code {
            data: vec![1, 9, 10, 3],
        };

        assert_eq!(expected, code);
    }

    #[test]
    fn test_parse_newline() {
        let code = Code::parse("1,2\n").unwrap();
        let expected = Code { data: vec![1, 2] };
        assert_eq!(expected, code);
    }

    #[test]
    fn test_step_done() {
        let mut code = Code::parse("99").unwrap();
        let result = step(&mut code, 0);
        assert_eq!(result, StepResult::Done);
    }
    #[test]
    fn test_add() {
        let mut code = Code::parse("1,4,5,3,10,20").unwrap();
        let result = step(&mut code, 0);
        let expected_code = Code::parse("1,4,5,30,10,20").unwrap();

        assert_eq!(result, StepResult::Next);
        assert_eq!(code, expected_code);
    }

    #[test]
    fn test_mul() {
        let mut code = Code::parse("2,4,5,3,10,20").unwrap();
        let result = step(&mut code, 0);
        let expected_code = Code::parse("2,4,5,200,10,20").unwrap();

        assert_eq!(result, StepResult::Next);
        assert_eq!(code, expected_code);
    }
}
