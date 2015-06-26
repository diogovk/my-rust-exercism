use std::collections::HashMap;

/// Returns a HashMap with the the number of times each word appears in text
pub fn word_count(text: &str) -> HashMap<String, u32> {
  let mut map_counts: HashMap<String, u32> = HashMap::new();
  let not_alphanumeric = |c: char| !c.is_alphanumeric();
  for x in text.split(not_alphanumeric).filter(|s| !s.is_empty()) {
      let entry = map_counts.entry(x.to_lowercase().to_string()).or_insert(0);
      *entry += 1;
  }
  map_counts
}

