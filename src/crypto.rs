/// decrypt ascii text (as used in PE XOR problem)
pub fn xor_decimal_text(text: &Vec<u32>, key: &Vec<u32>) -> Vec<u32> {
    text.iter()
        .enumerate()
        .map(|(i, n)| n ^ key[i % key.len()])
        .collect()
}
