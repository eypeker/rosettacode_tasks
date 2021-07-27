use druid::Data;
#[derive(Copy, Clone, Data)]
pub struct Point{
    x:u8,
    y:u8
}



impl Point {
    pub fn new(x:u8, y:u8) -> Point{
        match Point::are_valid(x,y){
            true => Point{x,y},
            false => {
                match x < 4 {
                    true => Point{x,y:3},
                    false => Point{x:3,y}
                }
            },
        }
    }

    pub fn are_valid(x:u8, y:u8) -> bool{
        x < 4 && y < 4
   }

    pub fn is_valid(&self) -> bool{
        Point::are_valid(self.x, self.y)
    }
    pub fn get_tuple(&self) -> (u8,u8){
        return (self.x, self.y)
    }

    pub fn is_neighbour(&self, point: &Point)-> bool {
        self.x == point.x && (self.y  + 1 == point.y || self.y - 1 ==point.y) 
        || 
        (self.y == point.y && (self.x + 1 == point.x || self.x - 1 == point.x))
    }


    pub fn set_point(& mut self, x:u8, y:u8){
        self.x = x & 0x03;
        self.y = y & 0x03;
    }

    pub fn x(&self ) -> usize{
        usize::from(self.x)
    }

    pub fn y(&self) -> usize{
        usize::from(self.y)
    }

    pub fn move_pos(&self, direction:u32) -> Point{
        match direction {
            0 => match self.x {3 => *self, _ => Point::new(self.x + 1, self.y)}, //moveright
            1 => match self.x {0 => *self, _ => Point::new(self.x - 1, self.y)}, //moveleft
            2 => match self.y {3 => *self, _ => Point::new(self.x, self.y + 1)}, //movedown
            3 => match self.y {0 => *self, _ => Point::new(self.x, self.y - 1)},  //moveup
            _ => *self
        }
    }

}

impl PartialEq for Point{
    fn eq(&self, other:&Point) -> bool{
        self.x == other.x && self.y == other.y
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}