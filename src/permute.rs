pub struct Permuter<T> {
    start: Vec<T>,
    last: Option<Vec<T>>,
    end: bool,
    permutation: Vec<T>,
}

impl<T: Clone + Ord + Eq + PartialEq> Permuter<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self {
            start: vec.clone(),
            last: Some(vec.clone()),
            end: false,
            permutation: vec,
        }
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

    fn swap(&mut self, i: usize, j: usize) {
        let temp = self.permutation[i].clone();
        self.permutation[i] = self.permutation[j].clone();
        self.permutation[j] = temp;
    }
}

impl<T: Clone + Ord + Eq + PartialEq> Iterator for Permuter<T> {
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
