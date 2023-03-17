use core::fmt;
use std::{io::{stdout,stdin}, ops::Deref};




fn main() {
    //let mut s = String::new();
    loop{
        println!("Bitte gib eine Zahl an, die zu einem Quardat gemacht werden kann");
        let num = match get_user_number() {
            Ok(x) => x,
            Err(s) => {
                println!("{}", s);
                continue;
            }
        };
        let mut matrix = create_matrix(num);
        insert_values(& mut matrix);
        print_matrix(& matrix, matrix.len(),matrix.len());
        
    }

}

fn get_user_number() -> Result<u16,String>{
    let mut buffer = String::new();
    std::io::stdin().read_line(& mut buffer).expect("Nicht lesbar für mich");
    match buffer.trim().parse::<u16>() {
        Ok(v) => Ok(v),
        Err(_) => return Err("Bitte nochmal wiederholen".to_string()),
    }
}



fn create_matrix(number: u16) -> Box<Vec<Vec<u16>>>{
    let matrix = Box::new(vec![vec![0; number as usize]; number as usize]);
    matrix
}


fn insert_values(matrix: & mut Box<Vec<Vec<u16>>>){
    let width = matrix.len();
    let height= width;
    let elements = width * height;
    let mut coords = Coordinate(0,0);
    let mut direction = Coordinate(1,-1);
    for e in 0..elements  as u16 {
        insert_value(matrix, &coords, e);
        match (&coords, &direction) {
            (Coordinate(0,y), Coordinate(-1,1)) if *y < (height as i32 -1) => direction = Coordinate(0,1), // nach unten links stößt auf linke wand, geht runter
            (Coordinate(0,y), Coordinate(0,1)) if *y <= (height as i32 -1) => direction = Coordinate(1,-1),
            (Coordinate(x, 0), Coordinate(1,-1)) if *x < (width as i32 -1) => direction = Coordinate(1,0),
            (Coordinate(x, 0), Coordinate(1,0)) if *x <= (width as i32 -1) => direction = Coordinate(-1,1),

            ((Coordinate(_, y), Coordinate(-1,1)) ) if *y == (height as i32 -1) => direction = Coordinate(1,0),
            (Coordinate(_,y), Coordinate(1,0)) if *y == (height as i32 -1) => direction = Coordinate(1,-1),
            (Coordinate(x, _), Coordinate(1,-1)) if *x == (width as i32 -1) => direction = Coordinate(0,1),
            (Coordinate(x, _), Coordinate(0, 1)) if *x == (width as i32 -1) => direction = Coordinate(-1,1),

            _ => {},
        }
        coords.0 = coords.0 + direction.0;
        coords.1 = coords.1 + direction.1;

    }
}

fn insert_value(matrix: & mut Box<Vec<Vec<u16>>>, coordinate: &Coordinate,value: u16){
    matrix.get_mut(coordinate.0 as usize).into_iter().for_each(|v| {
        v.get_mut(coordinate.1 as usize).into_iter().for_each(|w| *w = value);
    });
}


fn print_matrix(matrix: & Box<Vec<Vec<u16>>>,width:usize, height:usize){
    let mut s = String::new();
    for h in 0..height {
        for w in 0..width {
            matrix.get(w).into_iter().for_each(|ve| {
                ve.get(h).into_iter().for_each(|wert| {
                    s.push_str(&format!("{} ",wert));
                })
            });
        }
        s.push_str("\n");
    }
    print!("{}",s);
}


struct Coordinate(i32,i32);


impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C({}/{})", self.0, self.1)?;
        Ok(())
    }
}