use std::{
    cmp::Ordering,
    collections::HashMap,
    hash::{Hash, Hasher},
};

use anyhow::{bail, Result};

#[derive(Eq, Clone)]
pub struct BigInteger {
    // little endian
    sign: bool,
    array: Vec<u8>,
    // optional digit cap
    cap: Option<usize>,
}

impl Hash for BigInteger {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state);
        self.cap.hash(state);
        self.sign.hash(state);
    }
}

impl Ord for BigInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        self.array.cmp(&other.array)
    }
}

impl PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array && self.sign == other.sign
    }
}

impl BigInteger {
    pub fn from_vec(array: Vec<u8>, cap: Option<usize>) -> Self {
        Self {
            sign: true,
            array,
            cap,
        }
    }

    pub fn from_facts(factmap: HashMap<u64, u64>, cap: Option<usize>) -> Self {
        let mut s = Self::from_u32(1, cap);

        factmap.iter().for_each(|(n, c)| {
            let mut bn = BigInteger::from_u64(*n, None);
            bn.power(*c as u32);
            s.multiply(&bn);
        });

        s
    }

    pub fn from_string(s: &str, cap: Option<usize>) -> Result<Self> {
        let array: Vec<u8> = Vec::new();
        let mut sign = true;

        if let Some(first_char) = s.chars().nth(0) {
            sign = first_char != '-';
        }

        // convert from big endian to little endian
        let mut reversed: String = s.chars().rev().collect();

        // remove the sign
        if !sign {
            reversed.pop();
        }

        let mut s = Self { sign, array, cap };

        for c in reversed.chars() {
            if let Some(digit) = c.to_digit(10) {
                s.push_big(digit as u8);
            } else {
                bail!("must consist of digits and optional sign");
            }
        }

        Ok(s)
    }

    pub fn from_u64(n: u64, cap: Option<usize>) -> Self {
        let mut arr: Vec<u8> = Vec::new();
        let mut place: u64 = 1;
        let mut c = 0;
        while place <= n {
            if let Some(cap) = cap {
                if c >= cap {
                    break;
                }
            }
            let val = (n % (place * 10)) / place;
            arr.push(val as u8);
            place *= 10;
            c += 1;
        }
        return Self {
            sign: true,
            array: arr,
            cap,
        };
    }
    pub fn from_u32(n: u32, cap: Option<usize>) -> Self {
        let mut arr: Vec<u8> = Vec::new();
        let mut place: u32 = 1;
        let mut c = 0;
        while place <= n {
            if let Some(cap) = cap {
                if c >= cap {
                    break;
                }
            }
            let val = (n % (place * 10)) / place;
            arr.push(val as u8);
            place *= 10;
            c += 1;
        }
        return Self {
            sign: true,
            array: arr,
            cap,
        };
    }

    pub fn to_u32(&self) -> Result<u32, bool> {
        let mut place = 1;
        let mut sum: u32 = 0;
        for i in self.array.iter() {
            let mut stop: bool;
            (sum, stop) = sum.overflowing_add((*i as u32) * place);
            if stop {
                return Err(stop);
            }
            (place, stop) = place.overflowing_mul(10);
            if stop {
                return Err(stop);
            }
        }
        Ok(sum)
    }

    pub fn to_u64(&self) -> Result<u64, bool> {
        let mut place = 1;
        let mut sum: u64 = 0;
        for i in self.array.iter() {
            let mut stop: bool;
            (sum, stop) = sum.overflowing_add((*i as u64) * place);
            if stop {
                return Err(stop);
            }
            (place, stop) = place.overflowing_mul(10);
            if stop {
                return Err(stop);
            }
        }
        Ok(sum)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if !self.sign {
            s.push('-');
        }
        for i in self.array.iter().rev() {
            s.push_str(&format!("{i}"));
        }
        return s;
    }

    pub fn raw_vec(&self) -> &Vec<u8> {
        &self.array
    }

    pub fn multiply(&mut self, other: &Self) {
        let mut sum = Self::from_u32(0, self.cap);

        for (place, digit) in other.array.iter().enumerate() {
            let mut a = self.clone();
            a.multiply_digit(*digit).unwrap();
            a.pow_10(place);
            sum.add(&a);
        }

        self.array = sum.raw_vec().clone();
    }

    pub fn pow_10(&mut self, n: usize) {
        let mut vec = vec![0; n];
        vec.extend(self.array.clone());
        if let Some(cap) = self.cap {
            while vec.len() > cap as usize {
                vec.pop();
            }
        }
        self.array = vec;
    }

    pub fn power(&mut self, n: u32) {
        match n {
            0 => self.array = vec![0],
            1 => return,
            n => {
                let fact = self.clone();
                for _ in 1..n {
                    self.multiply(&fact);
                }
            }
        }
    }

    /// must be a single digit factor
    pub fn multiply_digit(&mut self, factor: u8) -> Result<()> {
        if factor > 9 {
            bail!("must multiply by single digit factor");
        }

        // fold over the array, multiplying and carrying
        let carry = self
            .array
            .clone()
            .iter()
            .enumerate()
            .fold(0, |carry, (place, digit)| {
                let product = digit * factor + carry;
                self.array[place] = product % 10;
                return product / 10;
            });

        // we can have either one or two value carries this way.
        if carry != 0 {
            if carry / 10 < 1 {
                self.push_big(carry);
            } else {
                self.push_big(carry % 10);
                self.push_big(carry / 10);
            }
        }

        Ok(())
    }

    /// appends to little endian, false if max length
    fn push_big(&mut self, digit: u8) -> bool {
        if let Some(cap) = self.cap {
            if self.array.len() < cap as usize {
                self.array.push(digit);
                true
            } else {
                false
            }
        } else {
            self.array.push(digit);
            true
        }
    }

    pub fn add(&mut self, other: &Self) {
        // make sure self is not shorter than other, else recursively swap
        if self.array.len() < other.array.len() {
            let mut s = other.clone();
            s.add(&self);
            self.array = s.array;
            return;
        }

        // fold over numbers with carry
        let carry = self
            .array
            .iter_mut()
            .zip(other.array.iter().chain(std::iter::repeat(&0)))
            .fold(0, |carry, (this, that)| {
                // carry the sum of column div 10
                let c = (carry + *this + that) / 10;
                // result is the sum of column mod 10
                *this = (carry + *this + that) % 10;
                return c;
            });

        if carry != 0 {
            self.push_big(carry);
        }
    }
}
