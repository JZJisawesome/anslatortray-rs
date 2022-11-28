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

//Returns whether a letter is y or not.
pub(crate) fn is_y(letter: char) -> bool {
    return letter.to_ascii_lowercase() == 'y';
}

//TODO testing
pub(crate) fn word_is_uppercase(english_word: &str) -> bool {
    for letter in english_word.chars() {
        if letter.is_ascii_lowercase() {
            return false;
        }
    }

    return true;
}

//TODO testing
pub(crate) fn word_is_uppercase_ascii(english_word: &str) -> bool {
    return word_is_uppercase(english_word);
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
}
