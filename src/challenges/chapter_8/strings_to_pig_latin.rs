pub fn to_pig_latin(text: &str) -> String {
    let mut result = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for word in text.split_whitespace() {
        let (_, ch) = word.chars().enumerate().next().expect("Somehow got a word that doesn't have a first letter!");
        if vowels.contains(&ch) {
            result.push_str(&format!("{}-hay ", word));
        } else {
            result.push_str(&format!("{}-{}ay ", &word[1..], &word[0..1]));
        }
    };

    return result;
}