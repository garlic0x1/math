/// perform sieve to limit, true is prime, false is composite
pub fn eratosthenes(limit: u64) -> Vec<bool> {
    let mut sieve = vec![true; limit as usize + 1];

    let mut i = 2;
    while i * i <= limit {
        if sieve[i as usize] == true {
            let mut mult: u64 = i * 2;
            while mult <= limit {
                sieve[mult as usize] = false;
                mult += i;
            }
        }
        i += 1;
    }

    sieve
}

/// vec of primes up to and including limit
pub fn primes(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();

    let sieve = eratosthenes(limit);

    for i in 2..limit + 1 {
        if sieve[i as usize] == true {
            primes.push(i);
        }
    }

    primes
}

/// vec of composites up to and including limit
pub fn composites(limit: u64) -> Vec<u64> {
    let mut composites = Vec::new();

    let sieve = eratosthenes(limit);

    for i in 2..limit + 1 {
        if sieve[i as usize] == false {
            composites.push(i);
        }
    }

    composites
}
