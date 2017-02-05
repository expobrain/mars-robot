use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use robot::{Instructions,};
use engine::{Point, Size, Vector, Movements};


#[derive(Debug)]
pub struct Setup {
    pub size: Size,
    pub instructions: Vec<Instructions>
}


fn parse_size(buffer: &str) -> Size {
    let values = buffer
        .lines()
        .take(1)
        .collect::<String>()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u8>>();

    Size::new(values[0], values[1])
}


fn parse_instructions(buffer: &str) -> Vec<Instructions> {
    let mut instructions = Vec::new();
    let mut lines = buffer.lines().skip(1);

    while let Some(line) = lines.next() {
        // Read starting coords and the direction
        let parts = line.trim().split_whitespace().collect::<Vec<&str>>();

        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();

        let start = Point::new(x, y);
        let direction: Vector = parts[2].parse().unwrap();

        // Read movements
        let movements = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Movements>().unwrap())
            .collect::<Vec<Movements>>();

        // Skip empty line
        let _ = lines.next();

        // Add instruction
        let instruction = Instructions::new(start, direction, movements);

        instructions.push(instruction);
    }

    instructions
}


impl Setup {
    fn new(size: Size, instructions: Vec<Instructions>) -> Self {
        Setup{
            size: size,
            instructions: instructions
        }
    }

    pub fn read_from_file(filename: &str) -> Self {
        // Open file
        let path = Path::new(filename);
        let mut buffer = String::new();

        let _ = match File::open(&path) {
            Ok(mut f) => f.read_to_string(&mut buffer),
            Err(e) => panic!("{}", e)
        };

        // Read settings
        let size = parse_size(&buffer);
        let instructions = parse_instructions(&buffer);

        // Return setup
        Setup::new(size, instructions)
    }
}
