use unicode_reverse::reverse_grapheme_clusters_in_place as reverse_string;

pub fn reverse(input: &str) -> String {
    let mut output = String::from(input);
    reverse_string(&mut output);
    output
}
