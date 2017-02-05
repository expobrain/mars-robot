use std::str::FromStr;
use std::fmt;
use std::ops::AddAssign;

use terrain::{Terrain};
use robot::{Instructions, Robot};


#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Movements {
    Left,
    Right,
    Forward,
}


#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub w: u8,
    pub h: u8
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    x: i8,
    y: i8
}


impl FromStr for Movements {
    type Err = ();

    fn from_str(s: &str) -> Result<Movements, ()> {
        match s {
            "L" => Ok(Movements::Left),
            "R" => Ok(Movements::Right),
            "F" => Ok(Movements::Forward),
            _ => Err(()),
        }
    }
}


impl Vector {
    pub fn rotate(&self, mov: Movements) -> Vector {
        match mov {
            Movements::Left  => Vector{x: -self.y, y: self.x},
            Movements::Right => Vector{x: self.y, y: -self.x},
            _ => *self
        }
    }
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Vector {x:  0, y:  1} => "N",
            Vector {x:  1, y:  0} => "E",
            Vector {x:  0, y: -1} => "S",
            Vector {x: -1, y:  0} => "W",
            _ => panic!("Why we reach this point?")
        };

        write!(f, "{}", s)
    }
}


impl FromStr for Vector {
    type Err = ();

    fn from_str(s: &str) -> Result<Vector, ()> {
        match s {
            "N" => Ok(Vector{x:  0, y:  1}),
            "E" => Ok(Vector{x:  1, y:  0}),
            "S" => Ok(Vector{x:  0, y: -1}),
            "W" => Ok(Vector{x: -1, y:  0}),
            _ => Err(()),
        }
    }
}


impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Point{x: x, y: y}
    }
}

impl AddAssign<Vector> for Point {
        fn add_assign(&mut self, other: Vector) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Size {
    pub fn new(w: u8, h: u8) -> Self {
        Size{w: w, h: h}
    }
}


pub struct Engine {
    terrain: Terrain
}


impl Engine {
    pub fn with_size(size: &Size) -> Self {
        let terrain = Terrain::with_size(*size);

        Engine{terrain: terrain}
    }

    pub fn run_instructions(&mut self, instructions: &Instructions) {
        let mut robot = Robot::new(instructions.start, instructions.direction);

        for movement in instructions.movements.iter() {
            match robot.update(&mut self.terrain, *movement) {
                Err(_) => break,
                _ => {}
            };
        }

        println!("{}", robot);
    }
}
