use std::{collections::HashMap, hash::Hash};

pub struct Permuter<T> {
    start: Vec<T>,
    last: Option<Vec<T>>,
    end: bool,
    permutation: Vec<T>,
}

impl<'a, T: Clone + Ord + Eq + PartialEq + Hash> Permuter<T> {
    /// create an iterator over the permutations of vec
    pub fn new(vec: Vec<T>) -> Self {
        Self {
            start: vec.clone(),
            last: None,
            end: false,
            permutation: vec,
        }
    }

    pub fn is_permutation_of(&self, other: Vec<T>) -> bool {
        if self.permutation.len() != other.len() {
            return false;
        }

        let mut set1 = HashMap::new();
        for x in other.iter() {
            if let Some(count) = set1.get_mut(x) {
                *count += 1;
            } else {
                set1.insert(x.clone(), 1);
            }
        }

        let mut set2 = HashMap::new();
        for x in self.permutation.iter() {
            if let Some(count) = set2.get_mut(x) {
                *count += 1;
            } else {
                set2.insert(x.clone(), 1);
            }
        }

        set1 == set2
    }

    fn reverse(&mut self, start: usize) {
        let mut i = start;
        let mut j = self.permutation.len() - 1;
        while i < j {
            self.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    fn reverse_substr(&mut self, start: usize, end: usize) {
        let mut i = start;
        let mut j = end;
        while i < j {
            self.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let temp = self.permutation[i].clone();
        self.permutation[i] = self.permutation[j].clone();
        self.permutation[j] = temp;
    }
}

impl<T: Clone + Ord + Eq + PartialEq + Hash> DoubleEndedIterator for Permuter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let last = self.permutation.clone();
        self.last = Some(last.clone());

        if self.end {
            return None;
        }

        let n = self.permutation.len() as i32 - 1;

        // find largest index i such that perm[i-1] > str[i]
        let mut i = n;
        while i > 0 && self.permutation[i as usize - 1] <= self.permutation[i as usize] {
            i -= 1;
        }

        if i > 0 {
            let mut j = i - 1;

            while (j + 1 <= n)
                && self.permutation[j as usize + 1] < self.permutation[i as usize - 1]
            {
                j += 1;
            }
            self.swap(i as usize - 1, j as usize);
        }
        self.reverse_substr(i as usize, n as usize);

        if self.permutation == self.start {
            self.end = true;
        }

        return Some(last);
    }
}

impl<T: Clone + Ord + Eq + PartialEq + Hash> Iterator for Permuter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.permutation.clone();
        self.last = Some(last.clone());

        if self.end {
            return None;
        }

        let mut i: i32 = self.permutation.len() as i32 - 2;

        while i >= 0 && self.permutation[i as usize + 1] <= self.permutation[i as usize] {
            i -= 1;
        }
        if i >= 0 {
            let mut j: i32 = self.permutation.len() as i32 - 1;
            while self.permutation[j as usize] <= self.permutation[i as usize] {
                j -= 1;
            }
            self.swap(i as usize, j as usize);
        }
        self.reverse((i + 1) as usize);

        if self.permutation == self.start {
            self.end = true;
        }

        return Some(last);
    }
}
