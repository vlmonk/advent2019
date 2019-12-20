use std::fs;
use std::iter;

const PATTERN: [i32; 4] = [0, 1, 0, -1];
const PATTERN_SIZE: usize = 4;
const REPEAT: usize = 100;

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
        // Some(1)
        self.inc();
        Some(PATTERN[self.j])
    }
}

#[derive(Debug, PartialEq)]
struct Signal {
    data: Vec<i32>,
}

impl Signal {
    pub fn parse(input: &str) -> Self {
        let data = input
            .split("")
            .filter_map(|ch| ch.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Self { data }
    }

    pub fn parse_10k(input: &str) -> Self {
        let single = input.split("").filter_map(|ch| ch.parse::<i32>().ok());
        let data = iter::repeat(single)
            .take(REPEAT)
            .flatten()
            .collect::<Vec<_>>();

        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn next(mut self, matrix: &[i32]) -> Self {
        let len = self.len();
        dbg!(len);
        assert_eq!(matrix.len(), len * len);

        let mut next_data = vec![0; len];

        for i in 0..len {
            for j in 0..len {
                next_data[i] += self.data[j] * matrix[i * len + j]
            }
        }

        for i in 0..len {
            next_data[i] = next_data[i].abs() % 10;
        }

        self.data = next_data;
        self
    }

    pub fn first_n(&self, n: usize) -> String {
        self.data
            .iter()
            .take(n)
            .fold(String::new(), |a, e| format!("{}{}", a, e))
    }

    pub fn first_n_skip(&self, n: usize, skip: usize) -> String {
        self.data
            .iter()
            .skip(skip)
            .take(n)
            .fold(String::new(), |a, e| format!("{}{}", a, e))
    }
}

fn task_a(input: &str) -> String {
    let mut input = Signal::parse(&input);
    let size = input.len();

    let matrix = (0..size)
        .map(|n| Pattern::new(n).take(size))
        .flatten()
        .collect::<Vec<_>>();

    for _ in 0..100 {
        input = input.next(&matrix);
    }

    input.first_n(8)
}

fn task_b(input: &str) -> String {
    let mut input = Signal::parse_10k(&input);
    dbg!("I");
    let offset = input.first_n(7).parse::<usize>().unwrap();
    dbg!("J");

    let size = input.len();

    dbg!(size);

    let matrix = (0..size)
        .map(|n| Pattern::new(n).take(size))
        .flatten()
        .collect::<Vec<_>>();

    dbg!(offset);

    for _ in 0..100 {
        input = input.next(&matrix);
    }

    input.first_n_skip(8, offset)
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("cant read input.txt");
    println!("task I : {}", task_a(&raw));
    println!("task II: {}", task_b(&raw));
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

    // #[test]
    // fn test_signal_next() {
    //     assert_eq!(Signal::parse("12345678").next(), Signal::parse("48226158"));
    //     assert_eq!(Signal::parse("34040438").next(), Signal::parse("03415518"));
    // }
    #[test]
    fn test_task_1() {
        assert_eq!(task_a("80871224585914546619083218645595\n"), "24176176");
        assert_eq!(task_a("19617804207202209144916044189917\n"), "73745418");
        assert_eq!(task_a("69317163492948606335995924319873\n"), "52432133");
    }
    // #[test]
    // fn test_task_2() {
    //     assert_eq!(task_b("03036732577212944063491565474664\n"), "84462026");
    //     assert_eq!(task_b("02935109699940807407585447034323\n"), "78725270");
    //     assert_eq!(task_b("03081770884921959731165446850517\n"), "53553731");
    // }
}
