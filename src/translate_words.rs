/* translate_words.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions to translate individual words (used by translate_strings.rs)
 *
*/

/* Imports */

use crate::helpers::{is_vowel, is_vowel_ascii, is_y, word_is_uppercase, word_is_uppercase_ascii, push_slice_to_vector};

/* Functions */

pub(crate) fn translate_word_with_style_reuse_buffers (
    english_word: &str,//Assumes this word is not empty
    suffix_lower: &str, special_case_suffix_lower: &str, suffix_upper: &str, special_case_suffix_upper: &str,
    buffer_to_append_to: &mut String, starting_consonants: &mut String
) {
    if english_word.len() == 1 {
        buffer_to_append_to.push_str(english_word);
        buffer_to_append_to.push_str(special_case_suffix_lower);
        return;
    }

    let mut iterator = english_word.chars();

    //Check the first letter
    let first_letter: char = iterator.next().unwrap();

    //Check if the word is uppercase
    let word_uppercase = word_is_uppercase(&english_word);

    //As a herustic, we consider Y to be a vowel when it is not at the start of the word
    let first_letter_was_vowel: bool = is_vowel(first_letter);//Not including y

    //Clear the starting_consonants buffer we were given
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
                    if is_vowel(character) || is_y(character) {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
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
    english_word: &[u8],//Assumes this word is not empty
    suffix_lower: &[u8], special_case_suffix_lower: &[u8], suffix_upper: &[u8], special_case_suffix_upper: &[u8],
    buffer_to_append_to: &mut Vec<u8>, starting_consonants: &mut Vec<u8>
) {
    if english_word.len() == 1 {
        push_slice_to_vector(buffer_to_append_to, english_word);
        push_slice_to_vector(buffer_to_append_to, special_case_suffix_lower);
        return;
    }

    //TODO more ascii optimizations

    //Set the starting index (the first character is assumed to exist and is accessed directly in several spots)
    let mut index = 1;

    //Check if the word is uppercase
    let word_uppercase = word_is_uppercase_ascii(english_word);

    //As a herustic, we consider Y to be a vowel when it is not at the start of the word
    let first_letter_was_vowel: bool = is_vowel_ascii(english_word[0]);//Not including y

    //Clear the starting_consonants buffer we were given
    starting_consonants.truncate(0);

    if first_letter_was_vowel {
        buffer_to_append_to.push(english_word[0]);
    } else {
        let first_char_was_upper = (english_word[0] as char).is_ascii_uppercase();
        starting_consonants.push(if word_uppercase { english_word[0] } else { (english_word[0] as char).to_ascii_lowercase() as u8 });

        //Grab all of the starting consonants, and push the first vowel we enounter to buffer_to_append_to
        while index < english_word.len() {
            let character: char = english_word[index] as char;
            if is_vowel(character) || is_y(character) {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
                //The vowel is the first letter of the word; we want it match the capitalization of the first letter of the original word
                if first_char_was_upper {
                    buffer_to_append_to.push(character.to_ascii_uppercase() as u8);
                } else {
                    buffer_to_append_to.push(character.to_ascii_lowercase() as u8);
                }
                index += 1;
                break;
            } else {
                starting_consonants.push(character as u8);
                index += 1;
            }
        }
    }

    //Copy all of the remaining letters up to the end of the word
    while index < english_word.len() {
        buffer_to_append_to.push(english_word[index]);
        index += 1;
    }

    //Copy starting consonants and add the suffix, or add the special_case_suffix depending on the circumstances
    if first_letter_was_vowel {
        if word_uppercase {
            push_slice_to_vector(buffer_to_append_to, special_case_suffix_upper);
        } else {
            push_slice_to_vector(buffer_to_append_to, special_case_suffix_lower);
        }
    } else {
        push_slice_to_vector(buffer_to_append_to, starting_consonants.as_slice());
        if word_uppercase {
            push_slice_to_vector(buffer_to_append_to, suffix_upper);
        } else {
            push_slice_to_vector(buffer_to_append_to, suffix_lower);
        }
    }
}

/* Tests */

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

    #[test]
    fn test_translate_word_with_style_ascii() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_word_with_style_ascii("Hello", suffix, special_case_suffix), "Elloh".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("World", suffix, special_case_suffix), "Orldw".to_string() + suffix);

            assert_eq!(translate_word_with_style_ascii("This", suffix, special_case_suffix), "Isth".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("is", suffix, special_case_suffix), "is".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style_ascii("a", suffix, special_case_suffix), "a".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style_ascii("test", suffix, special_case_suffix), "estt".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("of", suffix, special_case_suffix), "of".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style_ascii("the", suffix, special_case_suffix), "eth".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("function", suffix, special_case_suffix), "unctionf".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("translate", suffix, special_case_suffix), "anslatetr".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("word", suffix, special_case_suffix), "ordw".to_string() + suffix);

            assert_eq!(translate_word_with_style_ascii("I", suffix, special_case_suffix), "I".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style_ascii("Love", suffix, special_case_suffix), "Ovel".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("Pig", suffix, special_case_suffix), "Igp".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("Latin", suffix, special_case_suffix), "Atinl".to_string() + suffix);

            assert_eq!(translate_word_with_style_ascii("You", suffix, special_case_suffix), "Ouy".to_string() + suffix);//Y isn't a vowel here
            assert_eq!(translate_word_with_style_ascii("should", suffix, special_case_suffix), "ouldsh".to_string() + suffix);
            assert_eq!(translate_word_with_style_ascii("try", suffix, special_case_suffix), "ytr".to_string() + suffix);//Y is a vowel here
            assert_eq!(translate_word_with_style_ascii("yougurt", suffix, special_case_suffix), "ougurty".to_string() + suffix);//Y isn't a vowel here
            //assert_eq!(translate_word_with_style_ascii("it's", suffix, special_case_suffix), "it".to_string() + special_case_suffix + "'s");//Contraction
            assert_eq!(translate_word_with_style_ascii("quite", suffix, special_case_suffix), "uiteq".to_string() + suffix);//Awful to pronounce, but correct
            assert_eq!(translate_word_with_style_ascii("nice", suffix, special_case_suffix), "icen".to_string() + suffix);
        }
    }

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

    fn translate_word_with_style_ascii(english_word: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
        let mut suffix_upper = String::with_capacity(suffix_lower.len());
        for letter in suffix_lower.chars() {
            suffix_upper.push(letter.to_ascii_uppercase());
        }
        let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
        for letter in special_case_suffix_lower.chars() {
            special_case_suffix_upper.push(letter.to_ascii_uppercase());
        }

        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word
        translate_word_with_style_reuse_buffers_ascii (
            english_word.as_bytes(),
            suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &suffix_upper.as_bytes(), &special_case_suffix_upper.as_bytes(),
            &mut pig_latin_word, &mut starting_consonants_buffer
        );
        return std::str::from_utf8(pig_latin_word.as_slice()).unwrap().to_string();
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
    fn way_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = String::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box("translator");

            translate_word_with_style_reuse_buffers (
                word,
                "ay", "way", "AY", "WAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", pig_latin_word);//To avoid optimizing things out
    }

    #[bench]
    fn yay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = String::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box("translator");

            translate_word_with_style_reuse_buffers (
                word,
                "ay", "yay", "AY", "YAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", pig_latin_word);//To avoid optimizing things out
    }

    #[bench]
    fn hay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = String::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box("translator");

            translate_word_with_style_reuse_buffers (
                word,
                "ay", "hay", "AY", "HAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", pig_latin_word);//To avoid optimizing things out
    }

    #[bench]
    fn ferb_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = String::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box("translator");

            translate_word_with_style_reuse_buffers (
                word,
                "erb", "ferb", "ERB", "FERB",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", pig_latin_word);//To avoid optimizing things out
    }

    #[bench]
    fn ascii_way_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers_ascii (
                word,
                b"ay", b"way", b"AY", b"WAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn ascii_yay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers_ascii (
                word,
                b"ay", b"yay", b"AY", b"YAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn ascii_hay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers_ascii (
                word,
                b"ay", b"hay", b"AY", b"HAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn ascii_ferb_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers_ascii (
                word,
                b"erb", b"ferb", b"ERB", b"FERB",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }
}
