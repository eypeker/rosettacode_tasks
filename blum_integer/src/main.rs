fn is_prime(num: &u32) -> bool {
    for i in 2..(*num / 2) {
        if *num % i == 0 {
            return false;
        };
    }
    return true;
}

fn is_prime_factor(primes: &Vec<u32>, num: &u32) -> bool {
    for p in primes.iter() {
        if num % *p == 0 {
            let n = *num / *p;
            for q in primes.iter() {
                if *q == n {
                    return true;
                } else if *q > n {
                    return false;
                }
            }
        }
    }
    return false;
}

fn main() {
    let primes = (1..10000)
        .map(|i| {
            return 4 * i + 3;
        })
        .filter(|n| is_prime(n))
        .collect::<Vec<u32>>(); 
    let result = (1..).filter(|i| is_prime_factor(&primes, i)).nth(26828);

    println!("{:?}", result);
}
