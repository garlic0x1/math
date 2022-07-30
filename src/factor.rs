pub fn prime_factors(n: u64) -> Vec<u64> {
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

pub fn largest_prime(n: u64) -> u64 {
    if n % 2 == 0 {
        return largest_prime(n / 2);
    }

    if n % 3 == 0 {
        return largest_prime(n / 3);
    }

    let mut i = 5;
    let mut step = 2;

    while i * i <= n {
        if n % i == 0 {
            return largest_prime(n / i);
        }
        i += step;
        step = 6 - step;
    }
    return n;
}

pub fn smallest_prime(n: u64) -> u64 {
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
    return n;
}

pub fn is_prime(n: u64) -> bool {
    if n % 3 == 0 || n % 2 == 0 {
        return false;
    }

    let mut i = 5;
    let mut step = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += step;
        step = 6 - step;
    }
    true
}
