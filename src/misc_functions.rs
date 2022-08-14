use crate::factors::primes::*;

use std::collections::{HashMap, HashSet};

/// greatest common factor, Euclidean algorithm
pub fn gcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}

/// return the digits of a number as Vec<u8> in little endian order
pub fn digits(n: u64) -> Vec<u8> {
    let mut arr: Vec<u8> = Vec::new();
    let mut place = 1;
    while place <= n {
        let val = (n % (place * 10)) / place;
        arr.push(val as u8);
        place *= 10;
    }

    arr
}

/// returns true if there are repeated digits
pub fn numrepeats(n: u64) -> bool {
    let mut map = HashSet::new();
    let mut place: u64 = 1;
    while place <= n {
        let val = (n % (place * 10)) / place;
        if !map.insert(val) {
            return true;
        }
        place *= 10;
    }
    false
}

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
pub fn multiply_map(this: &mut HashMap<u64, u64>, that: &HashMap<u64, u64>) {
    for (k, v) in that.iter() {
        if let Some(v2) = this.get_mut(k) {
            *v2 += v;
        } else {
            this.insert(*k, *v);
        }
    }
}

/// create a factor map from a factorial
pub fn factorial_map(n: u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    if n > 1 {
        map = fact_map(n);
        let map2 = factorial_map(n - 1);

        multiply_map(&mut map, &map2);
    }
    map
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// determine if an integer is palindromic
pub fn is_palindromic_num(n: u64) -> bool {
    let nf = n as f64;
    let max = nf.log10().floor() as u64 + 1;

    let mut i = 0;
    while i < max / 2 {
        let place1 = u64::pow(10, i as u32);
        let place2 = u64::pow(10, (max - (i + 1)) as u32);
        let digit1 = (n % (place1 * 10)) / place1;
        let digit2 = (n % (place2 * 10)) / place2;
        if digit1 != digit2 {
            return false;
        }
        i += 1;
    }
    return true;
}

/// determine if a vec is palindromic
pub fn is_palindromic<T: Eq>(arr: Vec<T>) -> bool {
    for a in 0..=(arr.len() / 2) {
        let b = arr.len() - (a + 1);

        if arr[a] != arr[b] {
            return false;
        }
    }
    true
}
