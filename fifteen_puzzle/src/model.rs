use self::field::Tilegrid;


pub mod field;
pub mod point;

pub fn init() -> field::Tilegrid{
    Tilegrid::new()
}

use rand::Rng;
fn mix(mut tg: &field::Tilegrid){
    
}

pub fn create_field() -> field::Tilegrid{
    let mut tg = init();
    mix(tg);
    tg
}