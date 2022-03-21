

type Number = u64;

fn get_number() -> Number{
    use std::io::{stdin};
    let mut m = String::new();
    loop{
        m.clear();
        stdin().read_line(&mut m).expect("Did not expect something else than a Number");
        let trimmed = m.trim();
        println!("{}",trimmed);
        match trimmed.parse::<Number>(){
            Ok(i) => return i,
            Err(_) => ()
        }
    }
}


fn get_permutations(n:Number) -> Vec<Vec<Number>>{
    let mut mainvector:Vec<Number> = vec!();
    for _ in 1..(n+1) {
        mainvector.push(1);
    }
    let vecvec = permutations(&mainvector,2);
    return vecvec;
    
}

fn permutations(root:&Vec<Number>,not_defined_yet:Number)->Vec<Vec<Number>>{
    let mut rets:Vec<Vec<Number>> = vec![root.clone()];
    if root.len() <=1 {return rets};

    for i in not_defined_yet..(root.len()+1) as Number{
        let workingvec = (&root[(i) as usize..]).to_vec();
        let mut subperms = permutations(&workingvec,i);
        for p in subperms.iter_mut(){
            p.insert(0,i);
        }
        rets.append(& mut subperms)
    }
    return rets;
}

fn amount_permutations(vecleng:Number,not_defined_yet:Number)->Number{
    let mut count = 1;
    for l in not_defined_yet..(vecleng+1) {
        count += amount_permutations(vecleng-l, l);
    }
    return count;
}

fn get_amount_permutations(n:Number) -> Number{
    return amount_permutations(n,2);
}

fn main() {
    loop{
        println!("Bitte gib eine Nummer ein: ");
        let num = get_number();
        if num == 0 {break;}
        println!("You entered {}",num);  
        use std::time::SystemTime;
        let start = SystemTime::now();
        //let x = get_permutations(num);
        let x = get_amount_permutations(num);
        let end = SystemTime::now();
        let since_the_epoch = end
        .duration_since(start)
        .expect("Time went backwards").as_millis();
        println!("{:?}", since_the_epoch);
        println!("Amount Names: {}",x);
    }
}


