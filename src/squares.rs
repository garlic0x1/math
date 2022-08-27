/// find the integral square root by babylonian method
pub fn int_sqrt(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        let mut a = n / 2;
        let mut b = n;
        while a < b {
            let c = (a + (n / a)) / 2;
            (a, b) = (c, a);
        }
        b
    }
}
