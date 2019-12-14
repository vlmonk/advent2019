use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fs;
use std::ops::Add;
use num::Integer;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct Moon {
    px: i32,
    py: i32,
    pz: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

fn parse_el(input: regex::Match) -> Option<i32> {
    input.as_str().parse::<i32>().ok()
}

impl Moon {
    pub fn new(px: i32, py: i32, pz: i32, vx: i32, vy: i32, vz: i32) -> Self {
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)").unwrap();
        }

        let caps = RE.captures(input).ok_or("invalid input")?;
        let px = caps.get(1).and_then(parse_el).ok_or("invalid input")?;
        let py = caps.get(2).and_then(parse_el).ok_or("invalid input")?;
        let pz = caps.get(3).and_then(parse_el).ok_or("invalid input")?;

        Ok(Moon::new(px, py, pz, 0, 0, 0))
    }

    pub fn add_vel(&mut self, vel: &Vel) {
        self.vx += vel.0;
        self.vy += vel.1;
        self.vz += vel.2;
    }

    pub fn tick(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
        self.pz += self.vz;
    }

    pub fn energy(&self) -> i32 {
        self.pot() * self.kin()
    }

    fn pot(&self) -> i32 {
        self.px.abs() + self.py.abs() + self.pz.abs()
    }

    fn kin(&self) -> i32 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.px, self.py, self.pz, self.vx, self.vy, self.vz
        )
    }
}

#[derive(Debug)]
struct Vel(i32, i32, i32);

impl Vel {
    fn for_moon(a: &Moon, b: &Moon) -> Self {
        let dx = (b.px - a.px).signum();
        let dy = (b.py - a.py).signum();
        let dz = (b.pz - a.pz).signum();

        Self(dx, dy, dz)
    }

    fn new() -> Self {
        Self(0, 0, 0)
    }
}

impl Add for Vel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    pub fn parse(input: &str) -> Result<Self> {
        let moons = input
            .lines()
            .map(|line| Moon::parse(line))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { moons })
    }

    fn step(&mut self) {
        let vel = self
            .moons
            .iter()
            .map(|i| {
                self.moons
                    .iter()
                    .map(move |j| Vel::for_moon(i, j))
                    .fold(Vel::new(), |a, b| a + b)
            })
            .collect::<Vec<_>>();

        for (i, moon) in vel.iter().zip(self.moons.iter_mut()) {
            moon.add_vel(i);
            moon.tick()
        }
    }

    fn energy(&self) -> i32 {
        self.moons.iter().map(|m| m.energy()).sum()
    }

    fn ax(&self) -> Vec<AxisMoon> {
        self.moons
            .iter()
            .map(|moon| AxisMoon::new(moon.px, moon.vx))
            .collect()
    }

    fn ay(&self) -> Vec<AxisMoon> {
        self.moons
            .iter()
            .map(|moon| AxisMoon::new(moon.py, moon.vy))
            .collect()
    }
    fn az(&self) -> Vec<AxisMoon> {
        self.moons
            .iter()
            .map(|moon| AxisMoon::new(moon.pz, moon.vz))
            .collect()
    }
}

impl fmt::Display for System {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for moon in self.moons.iter() {
            write!(fmt, "{}\n", moon)?
        }

        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct AxisMoon {
    p: i32,
    v: i32,
}

impl AxisMoon {
    pub fn new(p: i32, v: i32) -> Self {
        Self { p, v }
    }
}

impl fmt::Display for AxisMoon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<pos={:3}, vel={:3}>", self.p, self.v)
    }
}

struct AxisSolver {
    system: Vec<AxisMoon>,
}

impl AxisSolver {
    fn new(system: Vec<AxisMoon>) -> Self {
        Self { system }
    }

    fn solve(&mut self) -> usize {
        let mut i = 0;
        let keep = self.system.clone();

        for i in &self.system {
            print!("{} ", i)
        }

        println!("");

        loop {
            i += 1;
            self.step();

            if self.system == keep {
                println!("Found on step {}", i);
                return i;
            }
        }
    }

    fn step(&mut self) {
        let add: Vec<i32> = self
            .system
            .iter()
            .map(|i| self.system.iter().map(|j| (j.p - i.p).signum()).sum())
            .collect();

        for (a, b) in self.system.iter_mut().zip(add.iter()) {
            a.v += b
        }

        for moon in self.system.iter_mut() {
            moon.p += moon.v
        }
    }
}

struct LoopSolver {
    ax: AxisSolver,
    ay: AxisSolver,
    az: AxisSolver,
}

impl LoopSolver {
    pub fn new(system: &System) -> Self {
        Self {
            ax: AxisSolver::new(system.ax()),
            ay: AxisSolver::new(system.ay()),
            az: AxisSolver::new(system.az()),
        }
    }

    pub fn solve(&mut self) -> usize {
        let x = self.ax.solve();
        let y = self.ay.solve();
        let z = self.az.solve();

        dbg!(x, y, z);
        x.lcm(&y).lcm(&z)
    }
}

fn main() -> Result<()> {
    let raw = fs::read_to_string("input.txt")?;

    let mut system = System::parse(&raw)?;
    for _ in 0..1000 {
        system.step();
    }
    let task_a = system.energy();

    let system = System::parse(&raw)?;
    let task_b = LoopSolver::new(&system).solve();

    println!("Task I: {}", task_a);
    println!("Task I: {}", task_b);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_moon() {
        assert_eq!(
            Moon::parse("<x=-1, y=0, z=2>\n").unwrap(),
            Moon::new(-1, 0, 2, 0, 0, 0)
        );

        assert_eq!(
            Moon::parse("<x=-100, y=99, z=2>\n").unwrap(),
            Moon::new(-100, 99, 2, 0, 0, 0)
        );
    }

    #[test]
    fn test_loop_sover() {
        let input = r#"<x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>"#;

        let system = System::parse(input).unwrap();
        let result = LoopSolver::new(&system).solve();
        assert_eq!(2772, result);
    }

    #[test]
    fn test_loop_sover2() {
        let input = r#"<x=-8, y=-10, z=0>
            <x=5, y=5, z=10>
            <x=2, y=-7, z=3>
            <x=9, y=-8, z=-3>"#;

        let system = System::parse(input).unwrap();
        let result = LoopSolver::new(&system).solve();
        assert_eq!(4686774924, result);
    }
}
