/* translate_words.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions to translate individual words (used by translate_strings.rs)
 *
*/

/* Imports */

use crate::helpers::{is_vowel, is_y, word_is_uppercase, word_is_uppercase_ascii};

/* Functions */

pub(crate) fn translate_word_with_style_reuse_buffers (
    english_word: &str,
    suffix_lower: &str, special_case_suffix_lower: &str, suffix_upper: &str, special_case_suffix_upper: &str,
    buffer_to_append_to: &mut String, starting_consonants: &mut String
) {
    if english_word.is_empty() {
        return;
    }

    if english_word.len() == 1 {
        buffer_to_append_to.push_str(english_word);
        buffer_to_append_to.push_str(special_case_suffix_lower);
        return;
    }

    let mut iterator = english_word.chars().peekable();

    //Check the first letter
    let first_letter: char = iterator.next().unwrap();

    //Check if the word is uppercase
    let word_uppercase = word_is_uppercase(&english_word);

    //As a herustic, we consider Y to be a vowel when it is not at the start of the word
    //However, if any word is only one letter long, this takes priority and the word is treated like a vowel
    let first_letter_was_vowel: bool = {
        is_vowel(first_letter).unwrap()//Not including y
        || if let Some(character) = iterator.peek() { !character.is_alphabetic() } else { true }//Non-alphabetic character after the first letter, or the word ends after the first letter
    };
    starting_consonants.truncate(0);

    if first_letter_was_vowel {
        buffer_to_append_to.push(first_letter);
    } else {
        let first_char_was_upper = first_letter.is_ascii_uppercase();
        starting_consonants.push(if word_uppercase { first_letter } else { first_letter.to_ascii_lowercase() });

        //Grab all of the starting consonants, and push the first vowel we enounter to buffer_to_append_to
        loop {
            match iterator.next() {
                None => { break; },//The word has no vowels, but it is a herustic to pass it on so that ex. the acroynm binary code decimal or bcd becomes bcdway, etc.
                Some(character) => {
                    if character.is_alphabetic() {
                        if is_vowel(character).unwrap() || is_y(character).unwrap() {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
                            //The vowel is the first letter of the word; we want it match the capitalization of the first letter of the original word
                            if first_char_was_upper {
                                buffer_to_append_to.push(character.to_ascii_uppercase());
                            } else {
                                buffer_to_append_to.push(character.to_ascii_lowercase());
                            }
                            break;
                        } else {
                            starting_consonants.push(character);
                        }
                    } else {//The word ended without vowels or we met an apostrophe
                        break;//It is a herustic to pass it on so that ex. the letter y becomes yway, the word a becomes away, etc.
                    }
                }
            }
        }
    }

    //Copy all of the remaining letters up to the end of the word
    loop {
        match iterator.next() {
            None => { break; },//End of the word
            Some(character) => { buffer_to_append_to.push(character); }
        }
    }

    //Copy starting consonants and add the suffix, or add the special_case_suffix depending on the circumstances
    if first_letter_was_vowel {
        if word_uppercase {
            buffer_to_append_to.push_str(special_case_suffix_upper);
        } else {
            buffer_to_append_to.push_str(special_case_suffix_lower);
        }
    } else {
        buffer_to_append_to.push_str(&starting_consonants);
        if word_uppercase {
            buffer_to_append_to.push_str(suffix_upper);
        } else {
            buffer_to_append_to.push_str(suffix_lower);
        }
    }
}

pub(crate) fn translate_word_with_style_reuse_buffers_ascii (
    english_word: &str,
    suffix_lower: &str, special_case_suffix_lower: &str, suffix_upper: &str, special_case_suffix_upper: &str,
    buffer_to_append_to: &mut String, starting_consonants: &mut String
) {
    //TODO make optimizations since we can assume the string is UTF8 safe
    translate_word_with_style_reuse_buffers (
        english_word,
        suffix_lower, special_case_suffix_lower, suffix_upper, special_case_suffix_upper,
        buffer_to_append_to, starting_consonants
    );
}

/* Tests */

#[cfg(test)]
fn translate_word_with_style(english_word: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    let mut pig_latin_word = String::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
    let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word
    translate_word_with_style_reuse_buffers (
        english_word,
        suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
        &mut pig_latin_word, &mut starting_consonants_buffer
    );
    return pig_latin_word;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_word_with_style() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_word_with_style("Hello", suffix, special_case_suffix), "Elloh".to_string() + suffix);
            assert_eq!(translate_word_with_style("World", suffix, special_case_suffix), "Orldw".to_string() + suffix);

            assert_eq!(translate_word_with_style("This", suffix, special_case_suffix), "Isth".to_string() + suffix);
            assert_eq!(translate_word_with_style("is", suffix, special_case_suffix), "is".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("a", suffix, special_case_suffix), "a".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("test", suffix, special_case_suffix), "estt".to_string() + suffix);
            assert_eq!(translate_word_with_style("of", suffix, special_case_suffix), "of".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("the", suffix, special_case_suffix), "eth".to_string() + suffix);
            assert_eq!(translate_word_with_style("function", suffix, special_case_suffix), "unctionf".to_string() + suffix);
            assert_eq!(translate_word_with_style("translate", suffix, special_case_suffix), "anslatetr".to_string() + suffix);
            assert_eq!(translate_word_with_style("word", suffix, special_case_suffix), "ordw".to_string() + suffix);

            assert_eq!(translate_word_with_style("I", suffix, special_case_suffix), "I".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("Love", suffix, special_case_suffix), "Ovel".to_string() + suffix);
            assert_eq!(translate_word_with_style("Pig", suffix, special_case_suffix), "Igp".to_string() + suffix);
            assert_eq!(translate_word_with_style("Latin", suffix, special_case_suffix), "Atinl".to_string() + suffix);

            assert_eq!(translate_word_with_style("You", suffix, special_case_suffix), "Ouy".to_string() + suffix);//Y isn't a vowel here
            assert_eq!(translate_word_with_style("should", suffix, special_case_suffix), "ouldsh".to_string() + suffix);
            assert_eq!(translate_word_with_style("try", suffix, special_case_suffix), "ytr".to_string() + suffix);//Y is a vowel here
            assert_eq!(translate_word_with_style("yougurt", suffix, special_case_suffix), "ougurty".to_string() + suffix);//Y isn't a vowel here
            //assert_eq!(translate_word_with_style("it's", suffix, special_case_suffix), "it".to_string() + special_case_suffix + "'s");//Contraction
            assert_eq!(translate_word_with_style("quite", suffix, special_case_suffix), "uiteq".to_string() + suffix);//Awful to pronounce, but correct
            assert_eq!(translate_word_with_style("nice", suffix, special_case_suffix), "icen".to_string() + suffix);
        }
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features", cfg(test))]
#[cfg(feature = "nightly-features")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn translate_word_the_word_translator(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_word_with_style("translator", "ay", "way");
        });
    }

    #[bench]
    fn translate_word_yay_the_word_translator(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_word_with_style("translator", "ay", "yay");
        });
    }

    #[bench]
    fn translate_word_ferb_the_word_translator(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_word_with_style("translator", "erb", "ferb");
        });
    }
}
