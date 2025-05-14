use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();
    let mut process: Vec<_> = word.to_lowercase().chars().collect();

    for w in possible_anagrams {
        if *w.to_lowercase() != word.to_lowercase() {
            process.sort_unstable();
            let mut compare: Vec<_> = w.to_lowercase().chars().collect();
            compare.sort_unstable();

            if compare == process {
                output.insert(*w);
            }

            process.clear();
            process.extend(word.to_lowercase().chars());
        }
    }

    output
}
