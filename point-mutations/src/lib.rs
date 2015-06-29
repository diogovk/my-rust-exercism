
pub fn hamming_distance(dna1: &str, dna2: &str) -> u32 {
    let mut difference = 0;
    for (x, y) in dna1.chars().zip(dna2.chars()) {
        if x != y {
            difference += 1;
        }
    }
    return difference;
}
