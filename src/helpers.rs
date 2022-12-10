/* helpers.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains helper functions
 *
*/

/* Functions */

//Returns whether a letter is a vowel or not.
pub(crate) fn is_vowel(letter: char) -> bool {
    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return true; }
        _ => { return false; }
    }
}

//Returns whether a letter is a vowel or not.
pub(crate) fn is_vowel_ascii(letter: u8) -> bool {
    match letter.to_ascii_lowercase() {
        b'a' | b'e' | b'i' | b'o' | b'u' => { return true; }
        _ => { return false; }
    }
}

//Returns whether a letter is y or not.
pub(crate) fn is_y(letter: char) -> bool {
    return letter.to_ascii_lowercase() == 'y';
}

//Returns whether a letter is y or not.
pub(crate) fn is_y_ascii(letter: u8) -> bool {
    return letter.to_ascii_lowercase() == b'y';
}

//Returns whether an entire word is upper case or not
pub(crate) fn word_is_uppercase(english_word: &str) -> bool {
    //We can't get the last character without iterating through the whole string since this is UTF-8
    //So the best we can do is exit out early if we encounter a lower-case character (we can't use the huristic in word_is_uppercase_ascii)
    for letter in english_word.chars() {
        if letter.is_ascii_lowercase() {
            return false;
        }
    }

    return true;
}

//Returns whether an entire word is upper case or not (the word must only contain ASCII characters)
pub(crate) fn word_is_uppercase_ascii(english_word_bytes: &[u8]) -> bool {
    //Asume length is non-zero
    //Heuristic: If the last letter of the word is uppercase, likely the whole word is uppercase
    return (english_word_bytes[english_word_bytes.len() - 1] as char).is_ascii_uppercase();
}

//Clones each element of a slice and push()es it to a vector
pub(crate) fn push_slice_to_vector<T: Clone>(vec: &mut Vec<T>, slice: &[T]) {
    for element in slice {
        vec.push(element.clone());
    }
}

//Capitalizes an ASCII/byte string at compile time
#[cfg(feature = "nightly-features-generics")]//Not unstable on it's own, but only needed by things enabled by nightly-features
pub(crate) const fn capitalize_ascii<const LEN: usize>(word: &[u8; LEN]) -> [u8; LEN] {
    let mut capitalized = [0u8; LEN];
    let mut i: usize = 0;
    while i < LEN {
        capitalized[i] = (word[i] as char).to_ascii_uppercase() as u8;
        i += 1;
    }
    return capitalized;
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        for letter in "aeiouAEIOU".chars() {
            assert!(is_vowel(letter));
        }

        for letter in "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars() {
            assert!(!is_vowel(letter));
        }

        for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(!is_vowel(not_letter));
        }
    }

    #[test]
    fn test_is_y() {
        for letter in "yY".chars() {
            assert!(is_y(letter));
        }

        for letter in "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".chars() {
            assert!(!is_y(letter));
        }

        for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(!is_y(not_letter));
        }
    }

    #[test]
    fn test_word_is_uppercase() {
        assert!(word_is_uppercase("HELLO"));
        assert!(word_is_uppercase("WORLD"));

        assert!(word_is_uppercase("I"));
        assert!(!word_is_uppercase("would"));
        assert!(!word_is_uppercase("like"));
        assert!(!word_is_uppercase("a"));
        assert!(!word_is_uppercase("pizza"));

        assert!(!word_is_uppercase("Sussus"));
        assert!(!word_is_uppercase("Amogus"));
    }

    #[test]
    fn test_word_is_uppercase_ascii() {
        assert!(word_is_uppercase_ascii(b"HELLO"));
        assert!(word_is_uppercase_ascii(b"WORLD"));

        assert!(word_is_uppercase_ascii(b"I"));
        assert!(!word_is_uppercase_ascii(b"would"));
        assert!(!word_is_uppercase_ascii(b"like"));
        assert!(!word_is_uppercase_ascii(b"a"));
        assert!(!word_is_uppercase_ascii(b"pizza"));

        assert!(!word_is_uppercase_ascii(b"Sussus"));
        assert!(!word_is_uppercase_ascii(b"Amogus"));
    }
}
