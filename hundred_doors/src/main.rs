mod door;



fn main() {
    let mut hw = door::Hallway::new(100);
    hw.alternate_all_doors();
    hw.print();

}
