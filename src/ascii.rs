/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::helpers::{is_vowel_ascii, is_y_ascii, word_is_uppercase_ascii, push_slice_to_vector};

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

pub fn translate(english: &[u8]) -> Vec::<u8> {
    return translate_way(english);
}

pub fn translate_way(english: &[u8]) -> Vec::<u8> {
    return translate_with_style(english, b"ay", b"way");
}

pub fn translate_with_style(english: &[u8], suffix_lower: &[u8], special_case_suffix_lower: &[u8]) -> Vec::<u8> {
    if english.is_empty() {
        return Vec::<u8>::new();
    }

    //TODO switch to fully operating on u8 slices/arrays/Vecs internally (converting from a string, then to a string at the end) in anslatortray 0.5.0

    let mut pig_latin_string = Vec::<u8>::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long

    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = Vec::<u8>::with_capacity(suffix_lower.len());
    for letter in suffix_lower.iter() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = Vec::<u8>::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.iter() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    //Flags used to remember if we're currently processing a word, contraction, contraction suffix or neither
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffer for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    //Indexes for improved performance (avoid copying characters to use as the english_word argument for translate_word_with_style_reuse_buffers)
    //However, this assumes each character is one byte, so this only works with ASCII strings
    let mut slice_start_index: usize = 0;//Inclusive
    let mut slice_end_index: usize = 0;//Exclusive

    for character in english.iter() {
        if in_word {
            if in_contraction_suffix {
                if character.is_ascii_alphabetic() {
                    //We never translate the contraction suffix of a word, so just copy remaining letters as-is
                } else {
                    //The contraction ended, and so too does the word
                    //We still want to copy the non-letter to the output though
                    in_contraction_suffix = false;
                    in_word = false;
                }

                pig_latin_string.push(*character);//Copy the character
                slice_start_index += 1;//Keep the slice start index up to speed for later use
            } else {
                if character.is_ascii_alphabetic() {
                    //This character is part of the word, so increment the slice_end_index to include it in the slice
                    slice_end_index += 1;
                } else {
                    //The word or first part of the contraction ended, so translate the word we've identified up until this point!
                    let word_slice: &[u8] = &english[slice_start_index..slice_end_index];
                    translate_word_with_style_reuse_buffers (
                        word_slice,
                        suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
                        &mut pig_latin_string, &mut starting_consonants_buffer
                    );

                    //Bring the slice_start_index to the end since we've finished the word and need it ready for the next one
                    slice_start_index = slice_end_index + 1;

                    //Append the symbol/whitespace we just got after the translated word
                    pig_latin_string.push(*character);

                    //If the symbol/whitespace we just got is an apostrophe, then this is a contraction suffix
                    if *character == b'\'' {
                        in_contraction_suffix = true;
                    } else {
                        in_word = false;//This wasn't a contraction, so we're done with the word
                    }
                }
            }
        } else {
            if character.is_ascii_alphabetic() {
                //If we see a letter, we are in a word, so set the slice_end_index to the character after the slice_start_index
                in_word = true;
                slice_end_index = slice_start_index + 1;
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(*character);
                slice_start_index += 1;
            }
        }
    }
    //If we ended on a word (but not on a contraction suffix), we translate it and push it to the end of the string
    if in_word && !in_contraction_suffix {
        let word_slice: &[u8] = &english[slice_start_index..slice_end_index];
        translate_word_with_style_reuse_buffers (
            word_slice,
            suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return pig_latin_string;
}

/*
pub(super) fn translate_word_with_style_reuse_buffers_better_perhaps <
    const SUFFIX_LEN: usize, const SPECIAL_CASE_SUFFIX_LEN: usize
> (
    english_word: &[u8],//Assumes this word is not empty
    suffix_lower: &[u8; SUFFIX_LEN], special_case_suffix_lower: &[u8; SPECIAL_CASE_SUFFIX_LEN], suffix_upper: &[u8; SUFFIX_LEN], special_case_suffix_upper: &[u8; SPECIAL_CASE_SUFFIX_LEN],
    buffer_to_append_to: &mut Vec<u8>, starting_consonants: &mut Vec<u8>
) {

}
*/

pub(super) fn translate_word_with_style_reuse_buffers (
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
        let first_char_was_upper = english_word[0].is_ascii_uppercase();
        starting_consonants.push(if word_uppercase { english_word[0] } else { english_word[0].to_ascii_lowercase() });

        //Grab all of the starting consonants, and push the first vowel we enounter to buffer_to_append_to
        while index < english_word.len() {
            let character: u8 = english_word[index];
            if is_vowel_ascii(character) || is_y_ascii(character) {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
                //The vowel is the first letter of the word; we want it match the capitalization of the first letter of the original word
                if first_char_was_upper {
                    buffer_to_append_to.push(character.to_ascii_uppercase());
                } else {
                    buffer_to_append_to.push(character.to_ascii_lowercase());
                }
                index += 1;
                break;
            } else {
                starting_consonants.push(character);
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

    //TODO test uppercase words

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

    //TODO add generic versions

    fn translate_word_with_style(english_word: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
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
        translate_word_with_style_reuse_buffers (
            english_word.as_bytes(),
            suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &suffix_upper.as_bytes(), &special_case_suffix_upper.as_bytes(),
            &mut pig_latin_word, &mut starting_consonants_buffer
        );
        return std::str::from_utf8(pig_latin_word.as_slice()).unwrap().to_string();
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn way_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"way", b"AY", b"WAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn yay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"yay", b"AY", b"YAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn hay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"hay", b"AY", b"HAY",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn ferb_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix
        let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"erb", b"ferb", b"ERB", b"FERB",
                &mut pig_latin_word, &mut starting_consonants_buffer
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }
}
