use druid::{Data, Lens};
#[derive(Clone,Data)]
enum Tile {
        Num(u32),
        Emp,
    }

use Tile::{Num,Emp};

use super::point::Point;
#[derive(Clone, Data, Lens)]
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
            empty:Point::new(3,3),
        }
    }

    pub fn move_tile(&mut self, pos:Point) -> Result<Point,Point>{
        if pos.is_valid() && pos.is_neighbour(&self.empty){
            self.field[self.empty.x()][self.empty.y()] = match self.field[pos.x()][ pos.y()]{
                Num(k) => Num(k),
                _ => Emp,
            };
            self.field[pos.x()][pos.y()] = Tile::Emp;
            self.empty = pos;
            Ok(self.empty)
        }else{
            Err(self.empty)
        }
    }

    pub fn get_field(&self) -> [[u32;4];4]{
        let mut arrvec : [[u32;4];4]= [[0;4];4];
        for (i, row) in self.field.iter().enumerate(){
            for (j, cell) in row.iter().enumerate(){
                arrvec[i][j] = match *cell { 
                    Num(k) => k,
                    Emp => 0
                }
            }
        }
        arrvec
    }

    pub fn get_empty(&self) -> Point{
        self.empty
    } 
}


impl ToString for Tilegrid{
    fn to_string(&self) -> std::string::String{
        let vs = self.get_field();
        format!("{0} {1} {2} {3} \n{4} {5} {6} {7} \n{8} {9} {10} {11} \n{12} {13} {14} {15} \n",
        vs[0][0], vs[0][1], vs[0][2], vs[0][3], vs[1][0], vs[1][1], vs[1][2], vs[1][3], vs[2][0], vs[2][1], vs[2][2], vs[2][3], vs[3][0], vs[3][1], vs[3][2], vs[3][3])
    }
}