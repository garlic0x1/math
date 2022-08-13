/// generate a sieve of divisor counts
/// (zero index is unused, i == n)
pub fn div_sieve(n: u64) -> Vec<u64> {
    let mut sieve = vec![0u64; (n + 1) as usize];
    div_sieve_rec(n, 1, &mut sieve);
    sieve
}

fn div_sieve_rec(n: u64, i: u64, sieve: &mut Vec<u64>) {
    if n >= i {
        fill_mults(n, i, 1, sieve);
        div_sieve_rec(n, i + 1, sieve)
    }
}

fn fill_mults(n: u64, i: u64, fact: u64, sieve: &mut Vec<u64>) {
    if i * fact <= n {
        sieve[(i * fact) as usize] += 1;
        fill_mults(n, i, fact + 1, sieve);
    }
}

/// perform sieve up to and including n, true is prime, false is composite
pub fn eratosthenes(n: u64) -> Vec<bool> {
    let mut sieve = vec![true; (n + 1) as usize];
    erat_rec(n, 2, &mut sieve);
    sieve
}

fn erat_rec(n: u64, i: u64, sieve: &mut Vec<bool>) {
    if i * i <= n {
        if sieve[i as usize] {
            cross_comps(n, i, i * 2, sieve);
        }
        erat_rec(n, i + 1, sieve)
    }
}

fn cross_comps(n: u64, i: u64, fact: u64, sieve: &mut Vec<bool>) {
    if fact <= n {
        sieve[fact as usize] = false;
        cross_comps(n, i, fact + i, sieve)
    }
}

/// perform sieve to limit, true is prime, false is composite
pub fn eratosthenes_looping(limit: u64) -> Vec<bool> {
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

    // alternate method
    eratosthenes(limit)
        .iter()
        .enumerate()
        .filter(|(i, &p)| *i > 1 && p)
        .for_each(|(i, _)| primes.push(i as u64));

    // let sieve = eratosthenes(limit);

    // for i in 2..limit + 1 {
    //     if sieve[i as usize] == true {
    //         primes.push(i);
    //     }
    // }

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
