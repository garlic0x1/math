pub fn eratosthenes(limit: u64) -> Vec<u64> {
    let mut sieve = vec![false; limit as usize + 1];

    let mut i = 2;
    while i * i <= limit {
        if sieve[i as usize] == false {
            let mut mult: u64 = i * 2;
            while mult <= limit {
                sieve[mult as usize] = true;
                mult += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::new();

    for i in 2..limit + 1 {
        if sieve[i as usize] == false {
            primes.push(i);
        }
    }

    primes
}
