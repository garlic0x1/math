pub mod bignums;
pub mod combinations;
pub mod crypto;
pub mod digits;
pub mod factors;
pub mod misc_functions;
pub mod permutations;

#[cfg(test)]
mod tests {
    use crate::combinations::*;
    use crate::digits::*;
    use crate::misc_functions::*;
    use crate::permutations::*;

    #[test]
    fn digits() {
        // 234 in base 10
        let mut lend: LEndian = LEndian::new(234, 10);
        assert_eq!(lend.next(), Some(4));
        assert_eq!(lend.last(), Some(2));
        let mut lend: BEndian = BEndian::new(234, 10);
        assert_eq!(lend.next(), Some(2));
        assert_eq!(lend.last(), Some(4));
    }

    #[test]
    fn crypto() {
        assert_eq!(65 ^ 42, 107);
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
    fn combinations() {
        assert_eq!(n_combinations(5, 3).unwrap(), 10);
        assert_eq!(n_combinations(23, 10).unwrap(), 1144066);
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
        assert_eq!(numrepeats(22769), true);
        assert_eq!(numrepeats(126489), false);
        assert_eq!(numcat(124, 77), 12477);
        assert_eq!(place(1, 0), Some(1));
        assert_eq!(place(0, 0), Some(0));
        assert_eq!(place(124, 0), Some(4));
        assert_eq!(place(124, 2), Some(1));
        assert_eq!(place(2550, 0), Some(0));
        assert_eq!(place(2550, 2), Some(5));
        assert_eq!(place(2550, 5), None);
        assert_eq!(numdigits(25502), 5);
        assert_eq!(numdigits(0), 1);
        assert_eq!(numdigits(1), 1);
        // assert_eq!(numdigits(4865197302), 10);
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(9), 362880);
    }
}
