use std::io::{BufRead, stdin};
use std::ops::{AddAssign, Mul};

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn rotate(self, degs: i32) -> Self {
        let mut res = self;
        if degs < 0 {
            for _ in (degs / 90)..0 {
                res = res.rotate_ccw();
            }
        } else {
            for _ in 0..(degs / 90) {
                res = res.rotate_cw();
            }
        }
        res
    }
    fn rotate_ccw(self) -> Self {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
    fn rotate_cw(self) -> Self {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Direction {
    fn ind(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
    fn from_ind(i: i32) -> Direction {
        match i.rem_euclid(4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Math is broken"),
        }
    }
    fn left(&self, degs: i32) -> Self {
        Direction::from_ind(self.ind() - (degs / 90))
    }
    fn right(&self, degs: i32) -> Self {
        Direction::from_ind(self.ind() + (degs / 90))
    }
    fn forward(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: 1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::South => Point { x: 0, y: -1 },
            Direction::West => Point { x: -1, y: 0 },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ShipState {
    d: Direction,
    p: Point,
}

impl ShipState {
    fn command(mut self, command: &str, value: i32) -> Self {
        match command {
            "N" => self.p += Point { x: 0, y: value },
            "E" => self.p += Point { x: value, y: 0 },
            "S" => self.p += Point { x: 0, y: -value },
            "W" => self.p += Point { x: -value, y: 0 },
            "L" => self.d = self.d.left(value),
            "R" => self.d = self.d.right(value),
            "F" => self.p += self.d.forward() * value,
            _ => panic!("Invalid command"),
        }
        self
    }
}

#[derive(Debug)]
struct ShipStateWaypoint {
    ship: Point,
    waypoint: Point,
}

impl ShipStateWaypoint {
    fn command(mut self, command: &str, value: i32) -> Self {
        match command {
            "N" => self.waypoint += Point { x: 0, y: value },
            "E" => self.waypoint += Point { x: value, y: 0 },
            "S" => self.waypoint += Point { x: 0, y: -value },
            "W" => self.waypoint += Point { x: -value, y: 0 },
            "L" => self.waypoint = self.waypoint.rotate(-value),
            "R" => self.waypoint = self.waypoint.rotate(value),
            "F" => self.ship += self.waypoint * value,
            _ => panic!("Invalid command"),
        }
        self
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let final_pos = lines.iter().fold(
        ShipState {
            d: Direction::East,
            p: Point { x: 0, y: 0 },
        },
        |s, l| s.command(&l[..1], l[1..].parse().unwrap()),
    );
    println!("{:?} {}", final_pos, final_pos.p.x.abs() + final_pos.p.y.abs());

    let final_pos2 = lines.iter().fold(
        ShipStateWaypoint {
            ship: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        },
        |s, l| s.command(&l[..1], l[1..].parse().unwrap()),
    );
    println!("{:?} {}", final_pos2, final_pos2.ship.x.abs() + final_pos2.ship.y.abs())
}
