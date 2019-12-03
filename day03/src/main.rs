use std::fmt;
use std::fs;

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
    wire_id: usize,
}

#[derive(PartialEq, Debug)]
enum Step {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
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
    fn new(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, wire_id: usize) -> Segment {
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
                wire_id,
            }
        } else if y0 == y1 && x0 != x1 {
            Segment {
                x0,
                y0,
                x1,
                y1,
                dir: Direction::Horizontal,
                wire_id,
            }
        } else {
            panic!("invalid segment")
        }
    }

    fn parse(input: &str) -> Vec<Segment> {
        input
            .lines()
            .enumerate()
            .map(|(i, line)| Segment::parse_line(line, i))
            .flatten()
            .collect()
    }

    fn parse_line(input: &str, wire_id: usize) -> Vec<Segment> {
        let mut result = vec![];
        let mut x = 0;
        let mut y = 0;

        input.split(',').for_each(|step| {
            let step = Step::parse(step);
            let (x1, y1) = step.next(x, y);
            result.push(Segment::new(x, y, x1, y1, wire_id));

            x = x1;
            y = y1;
        });
        result
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Segment ID: {} {} ({},{}) -> ({},{})",
            self.wire_id, self.dir, self.x0, self.y0, self.x1, self.y1
        )
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Horizontal => write!(f, "-"),
            Self::Vertical => write!(f, "|"),
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn cross<'a>(mut a: &'a Segment, mut b: &'a Segment) -> Option<Point> {
    if a.dir == b.dir {
        return None;
    }

    if a.dir == Direction::Vertical {
        std::mem::swap(&mut a, &mut b)
    }

    // a - horizontal, b - vertical

    if a.x0 < b.x0 && a.x1 > b.x0 && b.y0 < a.y0 && b.y1 > a.y0 {
        Some(Point::new(b.x0, a.y0))
    } else {
        None
    }
}

fn cross_distance(input: &[Segment]) -> Option<i32> {
    let len = input.len();

    let cross = (0..len)
        .map(|i| (i..len).map(move |j| (&input[i], &input[j])))
        .flatten()
        .filter(|(a, b)| a.wire_id != b.wire_id)
        .filter_map(|(a, b)| cross(a, b).map(|p| p.distance()))
        .collect::<Vec<_>>();

    cross.into_iter().fold(None, |a, e| match a {
        None => Some(e),
        Some(a) if a > e => Some(e),
        Some(a) => Some(a),
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");
    let segments = Segment::parse(&input);
    if let Some(q1) = cross_distance(&segments) {
        println!("Q1: {}", q1);
    } else {
        println!("Q1: Not found");
    }
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
            wire_id: 123,
        };
        let parsed = Segment::new(0, 0, 10, 0, 123);
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
            wire_id: 10,
        };
        let parsed = Segment::new(0, 0, -10, 0, 10);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_build() {
        let input = "R10,U1\n";
        let result = Segment::parse(input);
        let expected = vec![Segment::new(0, 0, 10, 0, 0), Segment::new(10, 0, 10, 1, 0)];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            None,
            cross(&Segment::new(0, 0, 10, 0, 0), &Segment::new(0, 0, 5, 0, 1))
        );
        assert_eq!(
            Some(Point::new(5, 0)),
            cross(&Segment::new(0, 0, 10, 0, 0), &Segment::new(5, 5, 5, -5, 1))
        );
        assert_eq!(
            Some(Point::new(2, 1)),
            cross(&Segment::new(2, 2, 2, -2, 0), &Segment::new(3, 1, 1, 1, 1))
        );
    }

    #[test]
    fn test_cross_distance_1() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let segments = Segment::parse(input);
        assert_eq!(Some(6), cross_distance(&segments));
    }

    #[test]
    fn test_cross_distance_2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let segments = Segment::parse(input);
        for i in segments.iter() {
            println!("{}", i)
        }

        assert_eq!(Some(159), cross_distance(&segments));
    }
    #[test]
    fn test_cross_distance_3() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let segments = Segment::parse(input);
        assert_eq!(Some(135), cross_distance(&segments));
    }
}
