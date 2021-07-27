use self::field::Tilegrid;


pub mod field;
pub mod point;

fn init() -> field::Tilegrid{
    Tilegrid::new()
}

use rand::Rng;
fn mix(mut tg: field::Tilegrid) -> field::Tilegrid{
    let mut rng =rand::thread_rng();
    let iterations = rng.gen_range(50..200);
    for _ in 0..iterations{
        let e = tg.get_empty();
        let dir = rng.gen_range(0..4); 
        let newpoint = e.move_pos(dir);
        if e != newpoint {
            match tg.move_tile(newpoint) {
                Err(_) => println!("something went wrong with moving the tiles while mixing"),
                _ => ()
            };
        }
    }
    tg
}

pub fn create_field() -> field::Tilegrid{
    let tg = init();
    mix(tg)
}