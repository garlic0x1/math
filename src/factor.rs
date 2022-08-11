use std::collections::HashMap;

/// create an occurence map of prime factors
pub fn fact_map(n: u32) -> HashMap<u32, u32> {
    let facts = prime_factors(n);
    let mut factmap = HashMap::new();
    for fact in facts {
        if let Some(count) = factmap.get_mut(&fact) {
            *count += 1;
        } else {
            factmap.insert(fact, 1);
        }
    }
    factmap
}

/// returns greatest common factor
pub fn gcf(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}

pub fn smallest_prime(n: u32) -> u32 {
    if n % 2 == 0 {
        return 2;
    }

    if n % 3 == 0 {
        return 3;
    }

    let mut i = 5;
    let mut step = 2;

    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += step;
        step = 6 - step;
    }

    n
}

pub fn smallest_prime_sieve(n: u32, primes: &Vec<u32>) -> u32 {
    if primes.len() == 0 {
        return smallest_prime(n);
    }

    for prime in primes.iter() {
        if n % prime == 0 {
            return *prime;
        }
    }

    let mut i = primes.last().unwrap() + 2;

    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += 2;
    }
    return n;
}

pub fn prime_factors(n: u32) -> Vec<u32> {
    let mut facts = Vec::new();
    let smallest = smallest_prime(n);
    if smallest != n {
        facts.push(smallest);
        facts.extend(prime_factors(n / smallest));
    } else {
        facts.push(smallest);
    }
    facts
}

pub fn largest_prime(n: u32) -> u32 {
    let smallest = smallest_prime(n);
    if smallest == n {
        n
    } else {
        largest_prime(n / smallest)
    }
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    smallest_prime(n) == n
}

/// calculate phi(n) using loops
pub fn totient_traditional(n: u32) -> u32 {
    let mut n = n;
    let mut result = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }

        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    return result;
}

/// calculate phi(n) recursively
pub fn totient(n: u32) -> u32 {
    totient_recursive(n, 2, n)
}

fn totient_recursive(n: u32, i: u32, res: u32) -> u32 {
    if n >= i * i {
        if n % i == 0 {
            totient_recursive(pow_fac(n, i), i + 1, res - res / i)
        } else {
            totient_recursive(n, i + 1, res)
        }
    } else if n > 1 {
        res - res / n
    } else {
        res
    }
}

fn pow_fac(n: u32, i: u32) -> u32 {
    if n % i == 0 {
        pow_fac(n / i, i)
    } else {
        n
    }
}
