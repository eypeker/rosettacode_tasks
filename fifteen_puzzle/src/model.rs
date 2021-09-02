pub use self::field::Tilegrid as Tg;


pub mod field;
pub mod point;

fn init() -> field::Tilegrid{
    Tg::new()
}

use rand::Rng;
fn mix(mut tg: Tg) -> field::Tilegrid{
    let mut rng = rand::thread_rng();
    let iterations = rng.gen_range(0..2);
    for _ in 0..iterations{
        let e = tg.get_empty();
        let dir = rng.gen_range(0..4); 
        let newpoint = e.move_pos(dir);
        if e != newpoint {
            tg.move_tile(newpoint) ;
        }
    }
    tg
}

pub fn create_field() -> field::Tilegrid{
    let tg = init();
    mix(tg)
}