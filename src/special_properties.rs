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
