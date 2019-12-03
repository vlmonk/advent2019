#[derive(PartialEq, Debug)]
struct Segment {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

#[derive(PartialEq, Debug)]
enum Step {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl Step {
    fn parse(input: &str) -> Self {
        let dir = input.chars().nth(0).unwrap();
        let len = &input[1..].trim_matches('\n').parse::<i32>().unwrap();

        match dir {
            'U' => Self::Up(*len),
            'R' => Self::Right(*len),
            'D' => Self::Down(*len),
            'L' => Self::Left(*len),
            _ => panic!("wrong direction"),
        }
    }

    fn next(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up(i) => (x, y + i),
            Self::Right(i) => (x + i, y),
            Self::Down(i) => (x, y - i),
            Self::Left(i) => (x - i, y),
        }
    }
}

impl Segment {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Segment {
        Segment { x0, y0, x1, y1 }
    }

    fn build(input: &str) -> Vec<Segment> {
        let mut result = vec![];
        let mut x = 0;
        let mut y = 0;

        input.split(',').for_each(|step| {
            let step = Step::parse(step);
            let (x1, y1) = step.next(x, y);
            result.push(Segment::new(x, y, x1, y1));

            x = x1;
            y = y1;
        });
        result
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_step() {
        assert_eq!(Step::parse("U10"), Step::Up(10));
        assert_eq!(Step::parse("L11\n"), Step::Left(11));
    }

    #[test]
    fn test_build() {
        let input = "R10,U1\n";
        let result = Segment::build(input);
        let expected = vec![Segment::new(0, 0, 10, 0), Segment::new(10, 0, 10, 1)];

        assert_eq!(expected, result);
    }
}
