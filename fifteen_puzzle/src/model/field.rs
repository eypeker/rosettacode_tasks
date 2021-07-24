
enum Tile {
        Num(u32),
        Emp,
    }

use Tile::{Num,Emp};

use super::point::Point;

pub struct Tilegrid{
    field:[[Tile;4];4],
    empty: Point,
}

impl Tilegrid{
    pub fn new() -> Tilegrid{
        let f: [[Tile;4];4] = [[Num(1),Num(2),Num(3),Num(4)],
            [Num(5),Num(6),Num(7),Num(8)],
            [Num(9),Num(10),Num(11),Num(12)],
            [Num(13),Num(14),Num(15),Emp]];
            
        Tilegrid{
            field:f,
            empty:Point::new(3,3).unwrap(),
        }
    }

    pub fn move_tile(&mut self, pos:Point) -> Result<Point,Point>{
        if pos.is_valid() && pos.is_neighbour(&self.empty){
            self.field[self.empty.x()][self.empty.y()] = match self.field[pos.x()][ pos.y()]{
                Num(k) => Num(k),
                _ => Num(0),
            };
            self.field[pos.x()][pos.y()] = Tile::Emp;
            self.empty = pos;
            Ok(self.empty)
        }else{
            Err(self.empty)
        }
    }

    pub fn get_field(&self) -> [[Option<u32>;4];4]{
        let mut arrvec : [[Option<u32>;4];4]= [[None;4];4];
        for (i, row) in self.field.iter().enumerate(){
            for (j, cell) in row.iter().enumerate(){
                arrvec[i][j] = match *cell { 
                    Num(k) => Some(k),
                    Emp => None
                }
            }
        }
        arrvec
    }

    pub fn get_empty(&self) -> Point{
        self.empty
    }

    
    
}
