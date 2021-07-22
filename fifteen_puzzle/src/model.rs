use self::field::Tilegrid;


pub mod field;
pub mod point;

pub fn init()->field::Tilegrid{
    Tilegrid::new()
}