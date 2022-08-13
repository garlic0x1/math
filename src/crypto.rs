/// decrypt ascii text (as used in PE XOR problem)
pub fn xor_decimal_text(text: &Vec<u32>, key: &Vec<u32>) -> Vec<u32> {
    let mut vec = Vec::new();

    let mut i = 0;
    for dec in text {
        let k = key[i % key.len()];

        vec.push(*dec ^ k);

        i += 1;
    }

    vec
}
