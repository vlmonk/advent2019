use std::fs;
use std::iter;

const PATTERN: [i32; 4] = [0, 1, 0, -1];
const PATTERN_SIZE: usize = 4;
const REPEAT: usize = 10000;

#[derive(Debug, PartialEq)]
enum Take {
    Pos(usize, usize),
    Neg(usize, usize),
}

impl Take {
    pub fn range(&self) -> (usize, usize) {
        use Take::*;
        match self {
            Pos(a, b) => (*a, *b),
            Neg(a, b) => (*a, *b),
        }
    }
}

enum NextStep {
    Pos(usize),
    Neg(usize),
    None,
}

struct Pattern {
    limit: usize,
    step: usize,
    next_step: NextStep,
}

impl Pattern {
    pub fn new(step: usize, limit: usize) -> Self {
        let first_pos = step;
        let next_step = match first_pos {
            n if n < limit => NextStep::Pos(n),
            _ => NextStep::None,
        };

        Self {
            step,
            limit,
            next_step,
        }
    }
}

impl Iterator for Pattern {
    type Item = Take;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_step {
            NextStep::None => None,
            NextStep::Pos(n) => {
                let current_start = n;
                let current_stop = (n + self.step + 1).min(self.limit);
                self.next_step = match n + (self.step + 1) * 2 {
                    n if n < self.limit => NextStep::Neg(n),
                    _ => NextStep::None,
                };

                Some(Take::Pos(current_start, current_stop))
            }
            NextStep::Neg(n) => {
                let current_start = n;
                let current_stop = (n + self.step + 1).min(self.limit);
                self.next_step = match n + (self.step + 1) * 2 {
                    n if n < self.limit => NextStep::Pos(n),
                    _ => NextStep::None,
                };

                Some(Take::Neg(current_start, current_stop))
            }
        }
    }
}

#[derive(Debug)]
struct RangeRequest {
    start: usize,
    len: usize,
}

impl RangeRequest {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn from_takes(input: &[Take]) -> Self {
        assert!(input.len() > 0);
        let ranges = input.iter().map(Take::range).collect::<Vec<_>>();
        let min = ranges.iter().map(|r| r.0).min().unwrap();
        let max = ranges.iter().map(|r| r.1).max().unwrap();

        Self {
            start: min,
            len: max - min,
        }
    }
}

#[derive(Debug)]
struct DataRange {
    offset: usize,
    data: Vec<i32>,
}

impl DataRange {
    pub fn get<'a>(&'a self, a: usize, b: usize) -> &'a [i32] {
        assert!(a >= self.offset);
        assert!(b >= a);

        let i = a - self.offset;
        let j = b - self.offset;

        &self.data[i..j]
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

    pub fn next(mut self) -> Self {
        let mut total = 0;
        let next_data: Vec<_> = (0..self.len())
            .map(|step| {
                total = 0;
                let full = Pattern::new(step, self.len())
                    .map(|pat| match pat {
                        Take::Pos(a, b) => self.data[a..b].iter().sum::<i32>(),
                        Take::Neg(a, b) => self.data[a..b].iter().sum::<i32>() * -1,
                    })
                    .inspect(|_| total += 1)
                    .sum::<i32>();
                dbg!(total);
                full.abs() % 10
            })
            .collect();

        self.data = next_data;

        self
    }

    pub fn first_n(&self, n: usize) -> String {
        self.data
            .iter()
            .take(n)
            .fold(String::new(), |a, e| format!("{}{}", a, e))
    }

    pub fn range(&self, req: RangeRequest, iter: usize) -> DataRange {
        println!("Range, iter: {}", iter);
        if iter == 0 {
            let data = self.data[req.start..(req.start + req.len)].to_vec();
            return DataRange {
                data: data,
                offset: req.start,
            };
        }

        let takes = (req.start..=(req.start + req.len))
            .map(|n| Pattern::new(n, self.len()).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();

        dbg!(&takes);

        let next_req = RangeRequest::from_takes(&takes[..]);
        dbg!(&next_req);
        unimplemented!();

        let range = self.range(next_req, iter - 1);

        let next_data: Vec<_> = (req.start..=(req.start + req.len))
            .map(|step| {
                let full = Pattern::new(step, self.len())
                    .map(|pat| match pat {
                        Take::Pos(a, b) => range.get(a, b).iter().sum::<i32>(),
                        Take::Neg(a, b) => range.get(a, b).iter().sum::<i32>() * -1,
                    })
                    .sum::<i32>();

                full.abs() % 10
            })
            .collect();

        println!("DONE ITER {}", iter);

        return DataRange {
            data: next_data,
            offset: req.start,
        };
    }
}

fn task_a(input: &str) -> String {
    let mut input = Signal::parse(&input);

    for _ in 0..100 {
        input = input.next();
    }

    input.first_n(8)
}

fn task_b(input: &str) -> String {
    let input = Signal::parse_10k(&input);
    let offset = input.first_n(7).parse::<usize>().unwrap();
    let req = RangeRequest::new(offset, 8);
    let range = input.range(req, 100);

    dbg!(range);

    "foo".to_owned()
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("cant read input.txt");
    // println!("task I : {}", task_a(&raw));
    println!("task II: {}", task_b(&raw));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pattern_0() {
        let mut pat = Pattern::new(0, 8);
        assert_eq!(pat.next(), Some(Take::Pos(0, 1)));
        assert_eq!(pat.next(), Some(Take::Neg(2, 3)));
        assert_eq!(pat.next(), Some(Take::Pos(4, 5)));
        assert_eq!(pat.next(), Some(Take::Neg(6, 7)));
        assert_eq!(pat.next(), None);
    }

    #[test]
    fn test_pattern_1() {
        let mut pat = Pattern::new(4, 8);
        assert_eq!(pat.next(), Some(Take::Pos(4, 8)));
        assert_eq!(pat.next(), None);
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
        assert_eq!(task_a("80871224585914546619083218645595\n"), "24176176");
        assert_eq!(task_a("19617804207202209144916044189917\n"), "73745418");
        assert_eq!(task_a("69317163492948606335995924319873\n"), "52432133");
    }
    #[test]
    fn test_task_2() {
        assert_eq!(task_b("03036732577212944063491565474664\n"), "84462026");
        assert_eq!(task_b("02935109699940807407585447034323\n"), "78725270");
        assert_eq!(task_b("03081770884921959731165446850517\n"), "53553731");
    }
}
