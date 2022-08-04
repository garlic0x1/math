use anyhow::{bail as yeet, Result};

#[derive(Clone)]
pub struct BigInteger {
    // little endian
    sign: bool,
    array: Vec<u8>,
}

impl BigInteger {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            sign: true,
            array: vec,
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        let mut array: Vec<u8> = Vec::new();
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

        for c in reversed.chars() {
            if let Some(digit) = c.to_digit(10) {
                array.push(digit as u8);
            } else {
                yeet!("must consist of digits and optional sign");
            }
        }

        Ok(Self { sign, array })
    }

    pub fn from_u32(n: u32) -> Self {
        let mut arr: Vec<u8> = Vec::new();
        let mut place: u32 = 1;
        while place <= n {
            let val = (n % (place * 10)) / place;
            arr.push(val as u8);
            place *= 10;
        }
        return Self {
            sign: true,
            array: arr,
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

    /*
           234
        x   17

          1638
        + 2340

          3978
    */

    pub fn multiply(&mut self, other: &Self) {
        let mut sum = Self::from_u32(0);

        println!("{}", self.to_u32().unwrap());
        for (place, digit) in other.array.iter().enumerate() {
            let mut a = self.clone();
            println!("{digit} {place} {}", a.to_string());
            a.multiply_digit(*digit).unwrap();
            println!("{digit} {place} {}", a.to_string());
            a.pow_10(place);
            println!("{digit} {place} {}", a.to_string());
            sum.add(&a);
            println!("sum: {}", sum.to_u32().unwrap());
        }

        self.array = sum.raw_vec().clone();
    }

    pub fn pow_10(&mut self, n: usize) {
        let mut vec = vec![0; n];
        vec.extend(self.array.clone());
        self.array = vec;
    }

    /// must be a single digit factor
    pub fn multiply_digit(&mut self, factor: u8) -> Result<()> {
        if factor > 9 {
            yeet!("must multiply by single digit factor");
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
                self.array.push(carry);
            } else {
                self.array.push(carry % 10);
                self.array.push(carry / 10);
            }
        }
        Ok(())
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
            self.array.push(carry);
        }
    }
}
