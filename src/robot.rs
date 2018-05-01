use std::fmt;

use engine::{Movements, Point, Vector};
use terrain::Terrain;

#[derive(Debug, Clone)]
pub struct Instructions {
    pub start: Point,
    pub direction: Vector,
    pub movements: Vec<Movements>,
}

#[derive(Debug)]
pub struct Robot {
    pos: Point,
    direction: Vector,
    lost: bool,
}

impl Instructions {
    pub fn new(start: Point, direction: Vector, movements: Vec<Movements>) -> Self {
        Instructions {
            start,
            direction,
            movements,
        }
    }
}

impl Robot {
    pub fn new(pos: Point, direction: Vector) -> Self {
        Robot {
            pos,
            direction,
            lost: false,
        }
    }

    pub fn update(&mut self, terrain: &mut Terrain, movement: Movements) -> Result<(), &str> {
        let mut pos = self.pos;
        let direction = self.direction.rotate(movement);

        if movement == Movements::Forward && !terrain.has_scent(pos, direction) {
            pos += direction;
        }

        if terrain.includes(pos) {
            self.direction = direction;
            self.pos = pos;

            Ok(())
        } else {
            self.lost = true;
            terrain.set_scent(self.pos, direction);

            Err("Robot lost")
        }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.lost {
            write!(f, "{} {} {} LOST", self.pos.x, self.pos.y, self.direction)
        } else {
            write!(f, "{} {} {}", self.pos.x, self.pos.y, self.direction)
        }
    }
}
