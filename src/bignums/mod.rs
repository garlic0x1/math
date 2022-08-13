pub mod big_fraction;
pub mod big_integer;
pub mod binary;

#[cfg(test)]
mod tests {
    // need to write tests
    // use super::big_fraction::*;
    use super::big_integer::*;
    use super::binary::*;

    #[test]
    fn binary_test() {
        let bin = Binary::from_u32(419);
        assert_eq!(&bin.big_endian(), "110100011");
        assert_eq!(bin.even(), false);
        let bin = Binary::from_u32(23);
        assert_eq!(&bin.little_endian(), "11101");
        assert_eq!(bin.even(), false);
        let mut bin = Binary::from_str("110100011").unwrap();
        bin.double();
        assert_eq!(bin.to_u32(), 838);
        assert_eq!(bin.even(), true);
    }

    #[test]
    fn decimal_test() {
        let mut num1 = BigInteger::from_u32(1234, None);
        let num2 = BigInteger::from_string("4321", None).unwrap();
        num1.add(&num2);

        assert_eq!(5555, num1.to_u32().unwrap());

        let mut num1 = BigInteger::from_u32(230, None);
        num1.multiply_digit(7).unwrap();

        assert_eq!(1610, num1.to_u32().unwrap());

        let mut num1 = BigInteger::from_u32(230, None);
        num1.pow_10(2);

        assert_eq!(23000, num1.to_u32().unwrap());

        let mut num1 = BigInteger::from_u32(9010, None);
        let num2 = BigInteger::from_u32(77, None);
        num1.multiply(&num2);

        assert_eq!(693770, num1.to_u32().unwrap());
    }

    #[test]
    fn fraction_test() {}
}
