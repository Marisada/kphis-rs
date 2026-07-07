// source: https://github.com/arastoof/AngloShift
// json files: https://github.com/hyperreality/American-British-English-Translator/tree/master/data

use std::{collections::HashMap, sync::LazyLock};

use crate::util::first_char_uppercase;

pub static TRANSLATOR: LazyLock<Translator> = LazyLock::new(|| Translator::new());

pub struct Translator {
    data: HashMap<String, String>,
}

impl Translator {
    fn new() -> Self {
        let british_to_american_raw = include_str!("../assets/british_spellings.json");
        let data: HashMap<String, String> = serde_json::from_str(&british_to_american_raw).unwrap();
        Self { data }
    }

    pub fn translate(&self, text: &str) -> String {
        let mut converted_text = String::new();
        let mut current_word = String::new();

        for c in text.chars() {
            if c.is_alphabetic() {
                current_word.push(c);
            } else {
                // We've hit a non-alphabetic character, so the previous sequence was a word (or part of one)
                if !current_word.is_empty() {
                    // Try to convert the word
                    let lowercased_word = current_word.to_lowercase(); // For case-insensitive lookup
                    if let Some(replacement) = self.data.get(&lowercased_word) {
                        // If a replacement is found, use it.
                        let replacement = &apply_case_to_replacement(&current_word, replacement);
                        converted_text.push_str(replacement);
                    } else {
                        // No replacement found, append the original word
                        converted_text.push_str(&current_word);
                    }
                    current_word.clear(); // Reset for the next word
                }
                converted_text.push(c); // Add the non-alphabetic character
            }
        }

        // Handle the last word if the text ends with one
        if !current_word.is_empty() {
            let lowercased_word = current_word.to_lowercase();
            if let Some(replacement) = self.data.get(&lowercased_word) {
                let replacement = &apply_case_to_replacement(&current_word, replacement);
                converted_text.push_str(replacement);
            } else {
                converted_text.push_str(&current_word);
            }
        }

        converted_text
    }
}

fn apply_case_to_replacement(original_word: &str, replacement: &str) -> String {
    if original_word.chars().all(|x| x.is_uppercase()) {
        // If original is ALL CAPS, make replacement ALL CAPS
        replacement.to_uppercase()
    } else if original_word.chars().next().map_or(false, |c| c.is_uppercase()) {
        // If original starts with uppercase, make replacement start with uppercase
        first_char_uppercase(replacement)
    } else {
        // Otherwise, return replacement as is (assuming original was lowercase or mixed)
        replacement.to_owned()
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(TRANSLATOR.translate("Will be Diarrhoea"), String::from("Will be Diarrhea"));
        assert_eq!(TRANSLATOR.translate(" DIARRHOEA AGAIN"), String::from(" DIARRHEA AGAIN"));
    }
}
