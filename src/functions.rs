use crate::factor::*;
use std::collections::HashMap;

/// create a number that is the concatenation of two nums
pub fn numcat(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut dec = b;

    while dec > 0 {
        dec /= 10;
        x *= 10;
    }

    x + b
}

/// number of decimal digits in a number
pub fn numdigits(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut x = n;
    let mut c = 0;

    while x > 0 {
        x /= 10;
        c += 1;
    }

    c
}

/// get the nth place of a number
pub fn place(n: u64, i: usize) -> Option<u8> {
    if n == 0 {
        return Some(0);
    }
    let head = n / u64::pow(10, i as u32);
    if head > 0 {
        Some((head % 10) as u8)
    } else {
        None
    }
}

/// multiply a factor map with another
pub fn multiply_map(this: &mut HashMap<u64, u32>, that: &HashMap<u64, u32>) {
    for (k, v) in that.iter() {
        if let Some(v2) = this.get_mut(k) {
            *v2 += v;
        } else {
            this.insert(*k, *v);
        }
    }
}

/// create a factor map from a factorial
pub fn factorial_map(n: u64) -> HashMap<u64, u32> {
    let mut map = HashMap::new();
    if n > 1 {
        map = fact_map(n);
        let map2 = factorial_map(n - 1);

        multiply_map(&mut map, &map2);
    }
    map
}

pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
