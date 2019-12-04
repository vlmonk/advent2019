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

#[derive(PartialEq, Debug)]
struct Wire(Vec<Segment>);

impl Wire {
    pub fn parse(input: &str) -> Self {
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
        Self { 0: result }
    }

    pub fn steps_to(&self, p: &Point) -> Option<i32> {
        let mut current = 0;

        for step in self.0.iter() {
            if step.constain(p) {
                return Some(current + step.start().distance_to(p));
            } else {
                current = current + step.lenght();
            }
        }

        None
    }
}

// Board - 2 set of segments
#[derive(PartialEq, Debug)]
struct Board {
    a: Wire,
    b: Wire,
}

impl Board {
    pub fn parse(input: &str) -> Self {
        let mut wires = input.lines().map(|line| Wire::parse(line));
        let a = wires.next().expect("invalid input");
        let b = wires.next().expect("invalid input");

        Self { a, b }
    }

    pub fn cross_distance(&self) -> Option<i32> {
        self.crossing().map(|point| point.distance()).min()
    }

    pub fn step_distance(&self) -> Option<i32> {
        self.crossing()
            .filter_map(
                |point| match (self.a.steps_to(&point), self.b.steps_to(&point)) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                },
            )
            .min()
    }

    fn crossing(&self) -> impl Iterator<Item = Point> + '_ {
        self.a
            .0
            .iter()
            .map(move |a| self.b.0.iter().map(move |b| (a, b)))
            .flatten()
            .filter_map(|(a, b)| cross(a, b))
    }
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
        let dir = match (x0, x1, y0, y1) {
            (x0, x1, y0, y1) if x0 == x1 && y0 != y1 => Direction::Vertical,
            (x0, x1, y0, y1) if x0 != x1 && y0 == y1 => Direction::Horizontal,
            _ => panic!("invalid segment"),
        };

        Segment {
            x0,
            y0,
            x1,
            y1,
            dir,
        }
    }

    pub fn constain(&self, p: &Point) -> bool {
        match self.dir {
            Direction::Horizontal => p.x >= self.x_min() && p.x <= self.x_max() && p.y == self.y0,
            Direction::Vertical => p.y >= self.y_min() && p.y <= self.y_max() && p.x == self.x0,
        }
    }

    pub fn start(&self) -> Point {
        Point {
            x: self.x0,
            y: self.y0,
        }
    }

    fn x_min(&self) -> i32 {
        if self.x0 <= self.x1 {
            self.x0
        } else {
            self.x1
        }
    }

    fn x_max(&self) -> i32 {
        if self.x0 <= self.x1 {
            self.x1
        } else {
            self.x0
        }
    }
    fn y_min(&self) -> i32 {
        if self.y0 <= self.y1 {
            self.y0
        } else {
            self.y1
        }
    }

    fn y_max(&self) -> i32 {
        if self.y0 <= self.y1 {
            self.y1
        } else {
            self.y0
        }
    }

    fn lenght(&self) -> i32 {
        (self.x1 - self.x0).abs() + (self.y1 - self.y0).abs()
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Segment {} ({},{}) -> ({},{})",
            self.dir, self.x0, self.y0, self.x1, self.y1
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

    fn distance_to(&self, p: &Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

fn cross<'a>(a: &'a Segment, b: &'a Segment) -> Option<Point> {
    if a.dir == b.dir {
        return None;
    }

    let (a, b) = match (a, b) {
        (a, b) if a.dir == Direction::Vertical => (b, a),
        (a, b) => (a, b),
    };

    // a - horizontal, b - vertical

    let x_min = a.x0.min(a.x1);
    let x_max = a.x0.max(a.x1);

    let y_min = b.y0.min(b.y1);
    let y_max = b.y0.max(b.y1);

    if x_min < b.x0 && x_max > b.x0 && y_min < a.y0 && y_max > a.y0 {
        Some(Point::new(b.x0, a.y0))
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");
    let board = Board::parse(&input);

    match board.cross_distance() {
        Some(q1) => println!("Q1: {}", q1),
        _ => println!("Q1: Not found"),
    }

    match board.step_distance() {
        Some(q2) => println!("Q1: {}", q2),
        _ => println!("Q1: Not found"),
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
        let parsed = Segment::new(-10, 0, 0, 0);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_build() {
        let input = "R10,U1\n";
        let result = Wire::parse(input);
        let expected = Wire {
            0: vec![Segment::new(0, 0, 10, 0), Segment::new(10, 0, 10, 1)],
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            None,
            cross(&Segment::new(0, 0, 10, 0), &Segment::new(0, 0, 5, 0))
        );
        assert_eq!(
            Some(Point::new(5, 0)),
            cross(&Segment::new(0, 0, 10, 0), &Segment::new(5, 5, 5, -5))
        );
        assert_eq!(
            Some(Point::new(2, 1)),
            cross(&Segment::new(2, 2, 2, -2), &Segment::new(3, 1, 1, 1))
        );
    }

    #[test]
    fn test_cross_distance_1() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let board = Board::parse(input);
        assert_eq!(Some(6), board.cross_distance());
    }

    #[test]
    fn test_cross_distance_2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let board = Board::parse(input);
        assert_eq!(Some(159), board.cross_distance());
    }
    #[test]
    fn test_cross_distance_3() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let board = Board::parse(input);
        assert_eq!(Some(135), board.cross_distance());
    }
}
