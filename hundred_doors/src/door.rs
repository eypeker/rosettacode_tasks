pub struct Hallway{
    doors : Vec<Box<Door>>
}

enum Door {
    CLOSED,
    OPEN
}

impl Hallway{
    pub fn new(number:usize) -> Hallway{
        let mut drs = Vec::new();
        for _i in 0..number {
            drs.push(Box::new(Door::CLOSED));
        }
        Hallway{doors:drs}
    }

    pub fn alternate_doors(& mut self, n:usize){
        for _d in self.doors.iter_mut().skip(n-1).step_by(n){
            match **_d {
                Door::CLOSED =>  *_d = Box::new(Door::OPEN),
                Door::OPEN => *_d = Box::new(Door::CLOSED)
            }
        }
    }

    pub fn alternate_all_doors(& mut self){
        let len = self.doors.len()+1;
        for i in 1..len{
            self.alternate_doors(i);
        }

    }

    pub fn print(&self){
        for (i,d) in self.doors.iter().enumerate(){
            println!("Door {1} ist {0}\t",match **d {Door::CLOSED=>"zu", Door::OPEN=>"auf"},i+1);
        }
    }

}