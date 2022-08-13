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

pub fn pow_fac(n: u32, i: u32) -> u32 {
    if n % i == 0 {
        pow_fac(n / i, i)
    } else {
        n
    }
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

/// returns greatest common factor
pub fn gcf(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}
