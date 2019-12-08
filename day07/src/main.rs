use crate::vm::CPU;
use permutohedron::heap_recursive;
use std::fs;

mod vm;

fn parse_programm(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .expect("invalid input")
        .split(',')
        .map(|v| v.parse::<i32>().expect("invalid number"))
        .collect::<Vec<_>>()
}

fn format_output(output: &[u8]) -> String {
    let inner = output
        .iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", inner)
}

fn run_programm(programm: &[i32], settings: &[u8]) -> i32 {
    let mut current = 0;
    for i in 0..5 {
        let mut cpu = CPU::new(programm.to_owned(), vec![settings[i] as i32, current]);
        cpu.run();
        current = cpu.output[0];
    }

    current
}

fn run_programm_loop(programm: &[i32], settings: &[u8]) -> i32 {
    let mut cpu = settings
        .iter()
        .map(|i| CPU::new(programm.to_owned(), vec![*i as i32]))
        .collect::<Vec<_>>();

    let mut max = 0;
    let mut current = 0;
    'here: loop {
        for i in 0..settings.len() {
            cpu[i].push(current);

            match cpu[i].run_part() {
                Some(v) => {
                    current = v;
                    if i == settings.len() - 1 && v > max {
                        max = v;
                    }
                }
                None => break 'here,
            }
        }
    }

    max
}

type Solution = (i32, Vec<u8>);

fn solve_task_1(programm: &[i32]) -> Option<Solution> {
    let mut phase = vec![0, 1, 2, 3, 4];
    let mut solutins = vec![];

    heap_recursive(&mut phase, |variaint| {
        let variaint = variaint.to_owned();
        let score = run_programm(programm, &variaint);

        solutins.push((score, variaint));
    });

    solutins.into_iter().fold(None, |a, e| match a {
        None => Some(e),
        Some(a) if a.0 < e.0 => Some(e),
        Some(a) => Some(a),
    })
}

fn solve_task_2(programm: &[i32]) -> Option<Solution> {
    let mut phase = vec![5, 6, 7, 8, 9];
    let mut solutins = vec![];

    heap_recursive(&mut phase, |variaint| {
        let variaint = variaint.to_owned();
        let score = run_programm_loop(programm, &variaint);

        solutins.push((score, variaint));
    });

    solutins.into_iter().fold(None, |a, e| match a {
        None => Some(e),
        Some(a) if a.0 < e.0 => Some(e),
        Some(a) => Some(a),
    })
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("can't read");
    let programm = parse_programm(&raw);

    if let Some((truster, phase)) = solve_task_1(&programm) {
        println!(
            "Max thruster signal I : {} with phase setting {}",
            truster,
            format_output(&phase)
        )
    }

    if let Some((truster, phase)) = solve_task_2(&programm) {
        println!(
            "Max thruster signal II: {} with phase setting {}",
            truster,
            format_output(&phase)
        )
    }
}
