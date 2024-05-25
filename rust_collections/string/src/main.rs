use std::io::{self, Write};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars(); // Create an iterator over the characters
    let first_char = chars.next(); // Get the first character, if it exists
    match first_char {
        Some(c) if is_vowel(c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", chars.collect::<String>(), c),
        None => String::new(),
    }
}

 
fn main() {
    println!("Enter a string: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let pig_latin_words: Vec<String> = input
        .split_whitespace()
        .map(|word| to_pig_latin(word))
        .collect();
    let pig_latin_sentence = pig_latin_words.join(" ");

    println!("pig latin of {} - {}", input, pig_latin_sentence);

}
