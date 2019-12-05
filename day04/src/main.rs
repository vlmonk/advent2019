use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Num([u8; 6]);

impl Num {
    fn from_i32(input: i32) -> Self {
        let d0 = ((input / 100_000) % 10) as u8;
        let d1 = ((input / 10_000) % 10) as u8;
        let d2 = ((input / 1_000) % 10) as u8;
        let d3 = ((input / 100) % 10) as u8;
        let d4 = ((input / 10) % 10) as u8;
        let d5 = (input % 10) as u8;

        Self {
            0: [d0, d1, d2, d3, d4, d5],
        }
    }

    fn inc(&mut self) {
        for i in (0..6).into_iter().rev() {
            self.0[i] += 1;
            if self.0[i] != 10 {
                return;
            }

            self.0[i] = 0;
        }
    }
}

fn is_2_digit_same(input: &Num) -> bool {
    (0..5).any(|i| input.0[i] == input.0[i + 1])
}

fn is_2_digit_same_advanced(input: &Num) -> bool {
    let input = input.0;

    (0..5).any(|i| match i {
        0 => (input[0] == input[1]) && (input[0] != input[2]),
        4 => (input[4] == input[5]) && (input[4] != input[3]),
        n => (input[n] == input[n + 1]) && (input[n] != input[n - 1]) && (input[n] != input[n + 2]),
    })
}

fn is_increase(input: &Num) -> bool {
    (0..5).all(|i| input.0[i] <= input.0[i + 1])
}

struct NumIter {
    current: Num,
    max: Num,
}

impl NumIter {
    pub fn new(from: i32, to: i32) -> Self {
        Self {
            current: Num::from_i32(from),
            max: Num::from_i32(to),
        }
    }
}

impl Iterator for NumIter {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let cloned = self.current.clone();
        self.current.inc();

        Some(cloned)
    }
}

fn main() {
    let now = Instant::now();

    let iter = NumIter::new(137683, 596253);
    let mut task_a = 0;
    let mut task_b = 0;

    for i in iter.filter(|i| is_increase(&i)) {
        if is_2_digit_same(&i) {
            task_a += 1;
        }

        if is_2_digit_same_advanced(&i) {
            task_b += 1;
        }
    }

    println!("Q1: {}", task_a);
    println!("Q2: {}", task_b);

    let total_time = now.elapsed();
    println!("Total time: {}  μs", total_time.as_micros());
}
