#[derive(PartialEq, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Debug)]
struct Segment {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    dir: Direction,
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
    fn new(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) -> Segment {
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1)
        }

        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1)
        }
        if x0 == x1 && y0 != y1 {
            Segment {
                x0,
                y0,
                x1,
                y1,
                dir: Direction::Vertical,
            }
        } else if y0 == y1 && x0 != x1 {
            Segment {
                x0,
                y0,
                x1,
                y1,
                dir: Direction::Horizontal,
            }
        } else {
            panic!("invalid segment")
        }
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

fn cross(a: &Segment, b: &Segment) -> Option<(i32, i32)> {
    if a.dir == Direction::Horizontal && b.dir == Direction::Vertical {
        if a.x0 < b.x0 && a.x1 > b.x0 && b.y0 < a.y0 && b.y1 > a.y0 {
            return Some((b.x0, a.y0));
        }
    } else if a.dir == Direction::Vertical && b.dir == Direction::Horizontal {
        if b.x0 < a.x0 && b.x1 > a.x0 && a.y0 < b.y0 && a.y1 > b.y0 {
            return Some((a.x0, b.y0));
        }
    }
    None
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
    fn test_segment_new_1() {
        let expected = Segment {
            x0: 0,
            y0: 0,
            x1: 10,
            y1: 0,
            dir: Direction::Horizontal,
        };
        let parsed = Segment::new(0, 0, 10, 0);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_segment_new_2() {
        let expected = Segment {
            x0: -10,
            y0: 0,
            x1: 0,
            y1: 0,
            dir: Direction::Horizontal,
        };
        let parsed = Segment::new(0, 0, -10, 0);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_build() {
        let input = "R10,U1\n";
        let result = Segment::build(input);
        let expected = vec![Segment::new(0, 0, 10, 0), Segment::new(10, 0, 10, 1)];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            None,
            cross(&Segment::new(0, 0, 10, 0), &Segment::new(0, 0, 5, 0))
        );
        assert_eq!(
            Some((5, 0)),
            cross(&Segment::new(0, 0, 10, 0), &Segment::new(5, 5, 5, -5))
        );
        assert_eq!(
            Some((2, 1)),
            cross(&Segment::new(2, 2, 2, -2), &Segment::new(3, 1, 1, 1))
        );
    }
}
