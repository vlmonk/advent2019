use std::fs;

const PATTERN: [i32; 4] = [0, 1, 0, -1];
const PATTERN_SIZE: usize = 4;

struct Pattern {
    step: usize,
    i: usize,
    j: usize,
}

impl Pattern {
    pub fn new(step: usize) -> Self {
        Self {
            step: step + 1,
            i: 0,
            j: 0,
        }
    }

    fn inc(&mut self) {
        self.i += 1;

        if self.i >= self.step {
            self.i = 0;
            self.j += 1;
        }

        if self.j >= PATTERN_SIZE {
            self.j = 0
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inc();
        Some(PATTERN[self.j])
    }
}

#[derive(Debug, PartialEq)]
struct Signal {
    data: Vec<u8>,
}

impl Signal {
    pub fn parse(input: &str) -> Self {
        let data = input
            .split("")
            .filter_map(|ch| ch.parse::<u8>().ok())
            .collect::<Vec<_>>();
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn next(mut self) -> Self {
        let data = (0..self.len()).map(|n| {
            self.data
                .iter()
                .zip(Pattern::new(n))
                .map(|(a, b)| (*a as i32) * b)
                .sum()
        });

        self.data = data.map(|u: i32| (u.abs() % 10) as u8).collect();
        self
    }

    pub fn first_n(&self, n: usize) -> String {
        self.data
            .iter()
            .take(n)
            .fold(String::new(), |a, e| format!("{}{}", a, e))
    }
}

fn task_a(input: &str) -> String {
    let mut input = Signal::parse(&input);

    for _ in 0..100 {
        input = input.next();
    }

    input.first_n(8)
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("cant read input.txt");
    println!("task I: {}", task_a(&raw));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pattern_0() {
        let values = Pattern::new(0).take(6).collect::<Vec<_>>();
        let expected = vec![1, 0, -1, 0, 1, 0];

        assert_eq!(values, expected);
    }

    #[test]
    fn test_pattern_3() {
        let values = Pattern::new(3).take(6).collect::<Vec<_>>();
        let expected = vec![0, 0, 0, 1, 1, 1];

        assert_eq!(values, expected);
    }

    #[test]
    fn test_signal_parse() {
        let input = "112\n";
        let parsed = Signal::parse(&input);
        let expected = Signal {
            data: vec![1, 1, 2],
        };

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_signal_next() {
        assert_eq!(Signal::parse("12345678").next(), Signal::parse("48226158"));
        assert_eq!(Signal::parse("34040438").next(), Signal::parse("03415518"));
    }
    #[test]
    fn test_task_1() {
        assert_eq!(task_a("80871224585914546619083218645595"), "24176176");
        assert_eq!(task_a("19617804207202209144916044189917\n"), "73745418");
        assert_eq!(task_a("69317163492948606335995924319873\n"), "52432133");
    }
}
