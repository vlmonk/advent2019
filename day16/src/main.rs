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

fn main() {
    println!("Hello, world!");
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
}
