pub mod big_fraction;
pub mod big_integer;
pub mod binary;
pub mod combinations;
pub mod factor;
pub mod functions;
pub mod permutations;
pub mod sieve;
pub mod special_properties;

#[cfg(test)]
mod tests {
    use crate::big_integer::*;
    use crate::binary::*;
    use crate::factor::*;
    use crate::functions::factorial;
    use crate::functions::factorial_map;
    use crate::permutations::Permuter;
    use crate::sieve::*;
    use crate::special_properties::*;

    #[test]
    fn factor_test() {
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
        assert_eq!(largest_prime(600851475143), 6857);
        assert_eq!(is_prime(2_147_483_647), true);
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
        let permuter = Permuter::new(vec![1, 2, 3]);
        let perms: Vec<Vec<u32>> = permuter.rev().collect();
        assert_eq!(
            format!("{:?}", perms),
            "[[1, 2, 3], [3, 2, 1], [3, 1, 2], [2, 3, 1], [2, 1, 3], [1, 3, 2]]"
        );
    }

    #[test]
    fn binary_nums() {
        let bin = Binary::from_u32(419);
        let s = bin.big_endian();
        assert_eq!(&s, "110100011");
        assert_eq!(bin.even(), false);
        let bin = Binary::from_u32(23);
        let s = bin.little_endian();
        assert_eq!(&s, "11101");
        assert_eq!(bin.even(), false);
        let mut bin = Binary::from_str("110100011").unwrap();
        bin.double();
        let n = bin.to_u32();
        assert_eq!(n, 838);
        assert_eq!(bin.even(), true);
    }

    #[test]
    fn big_nums() {
        let mut num1 = BigInteger::from_u32(1234);
        let num2 = BigInteger::from_string("4321").unwrap();
        num1.add(&num2);
        assert_eq!(5555, num1.to_u32().unwrap());
    }

    #[test]
    fn palindromes() {
        assert_eq!(is_palindromic_num(43400434), true);
        assert_eq!(is_palindromic_num(48400434), false);
        assert_eq!(is_palindromic(vec!["apple", "cherry", "blueberry"]), false);
        assert_eq!(is_palindromic(vec!["apple", "cherry", "apple"]), true);
    }

    #[test]
    fn misc_funcs() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(9), 362880);
        assert_eq!(fact_map(362880), factorial_map(9));
    }
}
