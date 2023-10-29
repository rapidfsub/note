use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for letter in input.graphemes(true).rev() {
        result.push_str(letter);
    }
    result
}
