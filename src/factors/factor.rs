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
