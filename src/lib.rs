pub mod big_integer;
pub mod factor;
pub mod permute;
pub mod sieve;
pub mod special_properties;

#[cfg(test)]
mod tests {
    use crate::big_integer::*;
    use crate::factor::*;
    use crate::permute::Permuter;
    use crate::sieve::*;
    use crate::special_properties::*;

    #[test]
    fn factor_test() {
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
        assert_eq!(largest_prime(600851475143), 6857);
    }

    #[test]
    fn sieves() {
        assert_eq!(composites(6), vec![4, 6]);
        assert_eq!(composites(12), vec![4, 6, 8, 9, 10, 12]);
        assert_eq!(primes(12), vec![2, 3, 5, 7, 11]);
        assert_eq!(primes(23), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    }

    #[test]
    fn permutations() {
        let permuter = Permuter::new(vec!['a', 'b', 'c']);
        let perms: Vec<Vec<char>> = permuter.collect();
        assert_eq!(format!("{:?}", perms), "[['a', 'b', 'c'], ['a', 'c', 'b'], ['b', 'a', 'c'], ['b', 'c', 'a'], ['c', 'a', 'b'], ['c', 'b', 'a']]");
        let permuter = Permuter::new(vec!['b', 'c']);
        let perms: Vec<Vec<char>> = permuter.collect();
        assert_eq!(format!("{:?}", perms), "[['b', 'c'], ['c', 'b']]");
    }

    #[test]
    fn big_nums() {
        let mut num1 = BigInteger::from_u32(1234);
        let num2 = BigInteger::from_string("4321").unwrap();
        num1.add(&num2);
        assert_eq!(5555, num1.to_u32().unwrap());
    }

    #[test]
    fn special_props() {
        assert_eq!(is_palindromic(43400434), true);
    }
}
