
/// Returns the number of nucleotide differences in two DNAs of same size
pub fn hamming_distance(dna1: &str, dna2: &str) -> usize {
    dna1.chars().zip(dna2.chars()).filter(|&(x, y)| x != y).count()
}
