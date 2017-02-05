use engine::{Point, Size, Vector};


#[derive(Debug, PartialEq)]
struct Lost(Vector);


#[derive(Debug)]
pub struct Terrain{
    size: Size,
    grid: Box<[Option<Lost>]>
}

impl Terrain {
    pub fn with_size(size: Size) -> Self {
        let capacity = size.w as usize * size.h as usize;
        let mut grid_vec = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            grid_vec.push(None);
        }

        Terrain{
            size: size,
            grid: grid_vec.into_boxed_slice()
        }
    }

    pub fn includes(&self, point: Point) -> bool {
        point.x >= 0
        && point.y >= 0
        && point.x as i16 <= self.size.w as i16
        && point.y as i16 <= self.size.h as i16
    }

    pub fn has_scent(&self, pos: Point, dir: Vector) -> bool {
        match self.includes(pos) {
            false => panic!("Point non inside the map"),
            true => {
                let index = self.index_from_pos(pos);

                self.grid[index] == Some(Lost(dir))
            }
        }
    }

    pub fn set_scent(&mut self, pos: Point, dir: Vector) {
        match self.includes(pos) {
            false => panic!("Point {:?} non inside the map"),
            true => {
                let index = self.index_from_pos(pos);

                self.grid[index] = Some(Lost(dir))
            }
        };
    }

    fn index_from_pos(&self, pos: Point) -> usize {
        (pos.y as usize * self.size.h as usize) + pos.x as usize
    }
}
