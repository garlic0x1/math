pub mod primes;
pub mod sieves;
pub mod totient;

#[cfg(test)]
mod tests {
    use crate::misc_functions::factorial_map;

    use super::primes::*;
    use super::sieves::*;
    use super::totient::*;

    #[test]
    fn primes_test() {
        assert_eq!(fact_map(362880), factorial_map(9));
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
        // commented out since i switched to u32
        // assert_eq!(largest_prime(600_851_475_143), 6857);
        assert_eq!(is_prime(2_147_483_647), true);
    }

    #[test]
    fn totient_test() {
        assert_eq!(totient(9), 6);
    }

    #[test]
    fn sieves_test() {
        assert_eq!(composites(6), vec![4, 6]);
        assert_eq!(composites(12), vec![4, 6, 8, 9, 10, 12]);
        assert_eq!(primes(12), vec![2, 3, 5, 7, 11]);
        assert_eq!(primes(23), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(eratosthenes(50), eratosthenes_looping(50));

        assert_eq!(div_sieve(9).last().unwrap(), &3);
        assert_eq!(div_sieve(12).last().unwrap(), &6);
        assert_eq!(div_sieve(14).last().unwrap(), &4);
        assert_eq!(div_sieve(15).last().unwrap(), &4);
    }
}
