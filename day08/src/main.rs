use std::fmt;
use std::fs;

struct Layer {
    data: Vec<u8>,
    w: usize,
    h: usize,
}

impl Layer {
    pub fn new(input: &[u8], w: usize, h: usize) -> Self {
        Self {
            data: input.to_owned(),
            w,
            h,
        }
    }

    pub fn n_digit(&self, d: u8) -> usize {
        self.data.iter().filter(|c| **c == d).count()
    }

    pub fn parse_by(input: &str, w: usize, h: usize) -> Vec<Self> {
        input
            .as_bytes()
            .chunks(w * h)
            .map(|ch| Self::new(ch, w, h))
            .collect()
    }

    pub fn merge(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| if *a == b'2' { *b } else { *a })
            .collect();

        Self {
            data,
            w: self.w,
            h: self.h,
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.data.chunks(self.w) {
            for point in chunk.iter() {
                let display = match point {
                    b'0' => ' ',
                    b'1' => '.',
                    b'2' => '?',
                    _ => panic!("invalid char"),
                };
                write!(f, "{}", display)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("read error");
    let line = raw.lines().next().expect("empty input").trim_matches('\n');
    let mut layers = Layer::parse_by(line, 25, 6);

    let task_a = layers
        .iter()
        .min_by_key(|l| l.n_digit(b'0'))
        .map(|l| l.n_digit(b'1') * l.n_digit(b'2'));

    if let Some(n) = task_a {
        println!("Task I: {}", n)
    }

    let top = layers.remove(0);
    let task_b = layers.iter().fold(top, |a, e| a.merge(e));

    println!("Task II\n{}", task_b);
}
