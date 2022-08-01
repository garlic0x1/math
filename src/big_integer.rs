use anyhow::{bail as yeet, Result};

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
            s.push_str(&format!("{}{}", s, i));
        }
        return s;
    }

    pub fn raw_vec(&self) -> &Vec<u8> {
        &self.array
    }

    pub fn multiply(&mut self, factor: u8) {
        let mut carry: u8 = 0;
        for i in 0..self.array.len() {
            carry = self.multiply_place(i, carry, factor);
        }
        if carry != 0 {
            if carry / 10 < 1 {
                self.array.push(carry);
            } else {
                self.array.push(carry % 10);
                self.array.push(carry / 10);
            }
        }
    }

    fn multiply_place(&mut self, place: usize, carry: u8, factor: u8) -> u8 {
        let product = (self.array.get(place).unwrap() * factor) + carry;
        self.array[place as usize] = product % 10;
        return product / 10;
    }

    pub fn add(&mut self, add: &Self) {
        let mut i = 0;
        let mut carry = 0;
        loop {
            if let Some((sum, car)) = self.add_place(i, add, carry) {
                self.array[i] = sum;
                carry = car;
                i += 1;
            } else {
                if carry != 0 {
                    self.array.push(carry);
                }
                break;
            }
        }
    }

    fn add_place(&mut self, place: usize, add: &Self, carry: u8) -> Option<(u8, u8)> {
        let a: u8;
        let b: u8;
        let mut ending = false;
        if let Some(x) = self.array.get(place) {
            a = *x;
        } else {
            a = 0;
            ending = true;
        }
        if let Some(x) = add.array.get(place) {
            b = *x;
            if ending {
                self.array.push(0);
            }
        } else {
            if ending {
                return None;
            } else {
                b = 0;
            }
        }
        let sum = carry + a + b;
        Some((sum % 10, sum / 10))
    }
}
