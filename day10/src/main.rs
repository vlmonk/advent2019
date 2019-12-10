#![feature(drain_filter)]

use std::cmp::max;
use std::error::Error;
use std::fmt::Debug;
use std::io::Read;
use std::time::Instant;

fn bench<T, F>(name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let res = (f)();
    let elapsed = start.elapsed();
    println!("{}: {:?}", name, elapsed);
    res
}

use nom::{branch::*, character::complete::*, combinator::*, multi::*, sequence::*, IResult};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Object {
    Asteriod,
    Nothing,
}

impl Object {
    fn is_asteriod(&self) -> bool {
        if let Object::Asteriod = self {
            true
        } else {
            false
        }
    }
}

fn asteroid(input: &str) -> IResult<&str, Object> {
    let (input, _) = char('#')(input)?;
    Ok((input, Object::Asteriod))
}

fn nothing(input: &str) -> IResult<&str, Object> {
    let (input, _) = char('.')(input)?;
    Ok((input, Object::Nothing))
}

fn asteroids_row(input: &str) -> IResult<&str, Vec<Object>> {
    many1(alt((asteroid, nothing)))(input)
}

fn asteroids(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    all_consuming(many0(delimited(multispace0, asteroids_row, multispace1)))(input)
}

use std::ops::{Add, Div, Mul, Rem};

#[derive(Debug, Clone, Copy)]
struct OffSet {
    s: Sign,
    n: usize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Center,
}

impl Dir {
    fn dir((x, y): &(OffSet, OffSet)) -> Dir {
        if x.n == 0 && y.n == 0 {
            Dir::Center
        } else if x.n == 0 {
            match y.s {
                Sign::Pos => Dir::Down,
                Sign::Neg => Dir::Up,
            }
        } else if y.n == 0 {
            match x.s {
                Sign::Pos => Dir::Right,
                Sign::Neg => Dir::Left,
            }
        } else {
            match (x.s, y.s) {
                (Sign::Pos, Sign::Pos) => Dir::DownRight,
                (Sign::Neg, Sign::Neg) => Dir::UpLeft,
                (Sign::Pos, Sign::Neg) => Dir::UpRight,
                (Sign::Neg, Sign::Pos) => Dir::DownLeft,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    Neg,
    Pos,
}

impl Add for OffSet {
    type Output = OffSet;

    fn add(self, other: Self) -> Self::Output {
        match (self.s, other.s) {
            (Sign::Pos, Sign::Pos) => Self {
                n: self.n + other.n,
                s: Sign::Pos,
            },
            (Sign::Neg, Sign::Neg) => Self {
                n: self.n + other.n,
                s: Sign::Neg,
            },
            (Sign::Pos, Sign::Neg) => {
                if other.n > self.n {
                    Self {
                        n: other.n - self.n,
                        s: Sign::Neg,
                    }
                } else {
                    Self {
                        n: self.n - other.n,
                        s: Sign::Pos,
                    }
                }
            }
            (Sign::Neg, Sign::Pos) => {
                if self.n > other.n {
                    Self {
                        n: self.n - other.n,
                        s: Sign::Neg,
                    }
                } else {
                    Self {
                        n: other.n - self.n,
                        s: Sign::Pos,
                    }
                }
            }
        }
    }
}

impl Add<usize> for OffSet {
    type Output = OffSet;

    fn add(self, other: usize) -> Self::Output {
        match self.s {
            Sign::Pos => Self {
                n: self.n + other,
                s: Sign::Pos,
            },
            Sign::Neg => {
                if self.n > other {
                    Self {
                        n: self.n - other,
                        s: Sign::Neg,
                    }
                } else {
                    Self {
                        n: other - self.n,
                        s: Sign::Pos,
                    }
                }
            }
        }
    }
}

impl Mul for OffSet {
    type Output = OffSet;

    fn mul(self, other: Self) -> Self::Output {
        match (self.s, other.s) {
            (Sign::Pos, Sign::Pos) => Self {
                n: self.n * other.n,
                s: Sign::Pos,
            },
            (Sign::Neg, Sign::Neg) => Self {
                n: self.n * other.n,
                s: Sign::Pos,
            },
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Self {
                n: self.n * other.n,
                s: Sign::Neg,
            },
        }
    }
}

impl Mul<usize> for OffSet {
    type Output = OffSet;

    fn mul(self, other: usize) -> Self::Output {
        match self.s {
            Sign::Pos => Self {
                n: self.n * other,
                s: Sign::Pos,
            },
            Sign::Neg => Self {
                n: self.n * other,
                s: Sign::Neg,
            },
        }
    }
}

impl Div for OffSet {
    type Output = OffSet;

    fn div(self, other: Self) -> Self::Output {
        match (self.s, other.s) {
            (Sign::Pos, Sign::Pos) => Self {
                n: self.n / other.n,
                s: Sign::Pos,
            },
            (Sign::Neg, Sign::Neg) => Self {
                n: self.n / other.n,
                s: Sign::Pos,
            },
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Self {
                n: self.n / other.n,
                s: Sign::Neg,
            },
        }
    }
}

impl Div<usize> for OffSet {
    type Output = OffSet;

    fn div(self, other: usize) -> Self::Output {
        match self.s {
            Sign::Pos => Self {
                n: self.n / other,
                s: Sign::Pos,
            },
            Sign::Neg => Self {
                n: self.n / other,
                s: Sign::Neg,
            },
        }
    }
}

impl Rem for OffSet {
    type Output = OffSet;

    fn rem(self, other: Self) -> Self::Output {
        match (self.s, other.s) {
            (Sign::Pos, Sign::Pos) => Self {
                n: self.n % other.n,
                s: Sign::Pos,
            },
            (Sign::Neg, Sign::Neg) => Self {
                n: self.n % other.n,
                s: Sign::Pos,
            },
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Self {
                n: self.n % other.n,
                s: Sign::Neg,
            },
        }
    }
}

impl OffSet {
    fn offset(dest: usize, src: usize) -> Self {
        if src > dest {
            Self {
                n: src - dest,
                s: Sign::Neg,
            }
        } else {
            Self {
                n: dest - src,
                s: Sign::Pos,
            }
        }
    }

    fn as_usize(self) -> usize {
        match self.s {
            Sign::Pos => self.n,
            Sign::Neg => panic!("ahhhhhhhhh"),
        }
    }
}

use num::Integer;

fn calc_sight(asteroids: &[Vec<Object>], a: (usize, usize), b: (usize, usize)) -> bool {
    if a == b {
        true
    } else {
        let (xa, ya) = a;
        let (xb, yb) = b;
        let (xm, ym) = (OffSet::offset(xb, xa), OffSet::offset(yb, ya));
        let gcd = if xm.n == 0 || ym.n == 0 {
            max(xm.n, ym.n)
        } else {
            xm.n.gcd(&ym.n)
        };
        let (dx, dy) = (xm / gcd, ym / gcd);
        (1..gcd).any(|n| {
            let y = (dy * n + ya).as_usize();
            let x = (dx * n + xa).as_usize();
            asteroids[y][x].is_asteriod()
        })
    }
}

fn calc_one(asteroids: &[Vec<Object>]) -> Option<((usize, usize), usize)> {
    asteroids
        .iter()
        .enumerate()
        .flat_map(|(ya, row)| {
            row.iter().enumerate().filter_map(move |(xa, object)| {
                if object.is_asteriod() {
                    let count = asteroids
                        .iter()
                        .enumerate()
                        .flat_map(|(yb, row)| {
                            row.iter().enumerate().filter(move |(xb, object)| {
                                if object.is_asteriod() {
                                    !calc_sight(&asteroids, (xa, ya), (*xb, yb))
                                } else {
                                    false
                                }
                            })
                        })
                        .count();
                    Some(((xa, ya), count))
                } else {
                    None
                }
            })
        })
        .max_by_key(|(_, count)| *count)
}

#[derive(Debug, Clone, Copy)]
struct Asteriod {
    m: (OffSet, OffSet),
    i: (usize, usize),
}

use std::cmp::Ordering;

impl Ord for Asteriod {
    fn cmp(&self, other: &Self) -> Ordering {
        let da = Dir::dir(&self.m);
        let db = Dir::dir(&other.m);
        if da as u8 != db as u8 {
            (da as u8).cmp(&(db as u8))
        } else {
            match da {
                Dir::Up => self.m.1.n.cmp(&other.m.1.n),
                Dir::Right => self.m.0.n.cmp(&other.m.0.n),
                Dir::Down => self.m.1.n.cmp(&other.m.1.n),
                Dir::Left => self.m.0.n.cmp(&other.m.0.n),
                Dir::Center => Ordering::Equal,
                _ => {
                    let a = self.m.0 * other.m.1;
                    let b = other.m.0 * self.m.1;
                    a.cmp(&b).reverse()
                }
            }
        }
    }
}

impl PartialOrd for Asteriod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Asteriod {
    fn eq(&self, other: &Self) -> bool {
        if let Ordering::Equal = self.cmp(other) {
            true
        } else {
            false
        }
    }
}

impl Eq for Asteriod {}

impl Ord for OffSet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.s, other.s) {
            (Sign::Neg, Sign::Pos) => Ordering::Less,
            (Sign::Pos, Sign::Neg) => Ordering::Greater,
            (Sign::Neg, Sign::Neg) => self.n.cmp(&other.n).reverse(),
            (Sign::Pos, Sign::Pos) => self.n.cmp(&other.n),
        }
    }
}

impl PartialOrd for OffSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OffSet {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s && self.n == other.n
    }
}

impl Eq for OffSet {}

fn calc_two(asteroids: &[Vec<Object>], (xa, ya): (usize, usize)) -> Vec<(usize, usize)> {
    let mut asteroids = asteroids.to_vec();
    asteroids[ya][xa] = Object::Nothing;
    let mut to_destroy: Vec<_> = asteroids
        .iter()
        .enumerate()
        .flat_map(|(yb, row)| {
            row.iter().enumerate().filter_map(move |(xb, object)| {
                if object.is_asteriod() {
                    Some(Asteriod {
                        m: (OffSet::offset(xb, xa), OffSet::offset(yb, ya)),
                        i: (xb, yb),
                    })
                } else {
                    None
                }
            })
        })
        .collect();
    to_destroy.sort();

    /*
    let mut debug: Vec<Vec<String>> = asteroids
        .iter()
        .map(|row| {
            row.iter()
                .map(|object| {
                    if object.is_asteriod() {
                        "###".to_string()
                    } else {
                        "...".to_string()
                    }
                })
                .collect()
        })
        .collect();
    for (i, x) in to_destroy.iter().enumerate() {
        debug[x.i.1][x.i.0] = format!("{:03}", i);
    }
    for i in debug {
        for j in i {
            print!(" {} ", j);
        }
        println!("");
    }*/

    let mut result = Vec::new();
    while to_destroy.len() > 0 {
        let destroyed: Vec<_> = to_destroy
            .drain_filter(|asteroid| !calc_sight(&asteroids, (xa, ya), asteroid.i))
            .collect();
        for i in destroyed {
            asteroids[i.i.1][i.i.0] = Object::Nothing;
            result.push(i.i);
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let asteroids = bench("parse_inputs", || -> Result<_, Box<dyn Error>> {
        let mut buffer = String::new();
        std::io::stdin().lock().read_to_string(&mut buffer)?;
        Ok(asteroids(&buffer).unwrap().1)
    })?;
    let one = bench("calc_one", || calc_one(&asteroids).unwrap());
    let two = bench("calc_two", || calc_two(&asteroids, one.0)[199]);
    println!("Answer One: {:?}", one);
    println!("Answer Two: {:?}", (two.0 * 100 + two.1));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::error::Error;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MyError {
        #[error("Value should be equal")]
        NotEqual(((usize, usize), usize), ((usize, usize), usize)),
    }

    #[test]
    fn one() -> Result<(), Box<dyn Error>> {
        let input = r#"
            .#..#
            .....
            #####
            ....#
            ...##
        "#;
        test(input, ((3, 4), 8))
    }

    #[test]
    fn two() -> Result<(), Box<dyn Error>> {
        let input = r#"
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
        "#;
        test(input, ((5, 8), 33))
    }

    #[test]
    fn three() -> Result<(), Box<dyn Error>> {
        let input = r#"
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.
        "#;
        test(input, ((1, 2), 35))
    }

    #[test]
    fn four() -> Result<(), Box<dyn Error>> {
        let input = r#"
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..
        "#;
        test(input, ((6, 3), 41))
    }

    #[test]
    fn five() -> Result<(), Box<dyn Error>> {
        let input = r#"
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
        "#;
        test(input, ((11, 13), 210))
    }

    fn test(input: &str, expect: ((usize, usize), usize)) -> Result<(), Box<dyn Error>> {
        let asteroids = bench("parse_inputs", || -> Result<_, Box<dyn Error>> {
            Ok(asteroids(&input).unwrap().1)
        })?;
        let result = calc_one(&asteroids).unwrap();
        if result != expect {
            Err(Box::new(MyError::NotEqual(result, expect)))
        } else {
            Ok(())
        }
    }
}
