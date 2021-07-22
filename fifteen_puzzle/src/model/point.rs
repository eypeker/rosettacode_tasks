#[derive(Copy, Clone)]
pub struct Point{
    x:u8,
    y:u8
}



impl Point {
    pub fn new(x:u8, y:u8) -> Result<Point,String>{
        match Point::are_valid(x,y){
            true=>Ok(Point{x,y}),
            false=>Ok(Point{x:3,y:3}),
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
        self.x == point.x && (self.y  +1 == point.y || self.y - 1 ==point.y) 
        || 
        (self.y == point.y && (self.x + 1 == point.x || self.x - 1 ==point.x))
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

}