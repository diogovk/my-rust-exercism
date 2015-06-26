/// Receives a word and an array of candidates and returns a 
/// vector with the words that are an anagram to that word
/// # Examples
/// ```
/// let inputs = ["tan", "stand", "at"];
/// let outputs: Vec<&str> = vec!["tan"];
/// assert_eq!(anagram::anagrams_for("ant", &inputs), outputs);
/// ```
pub fn anagrams_for<'a>(word: &str, candidates: &'a [&str] ) -> Vec<&'a str> {
    let normalized_word = normalize(word);
    candidates.iter().filter(
        |&cand| *cand != word && normalized_word == normalize(cand) 
        ).cloned().collect()
}

fn normalize(word: &str) -> Vec<char> {
    let mut upper_word: Vec<_> = word.to_uppercase().chars().collect();
    upper_word.sort();
    upper_word
}

