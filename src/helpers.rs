/* helpers.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains helper functions for translate_words.rs
 *
*/

/* Functions */

//Returns whether a letter is a vowel or not.
//
//If the parameter is a letter, returns Some(true) if it is a vowel, and Some(false) otherwise.
//If the parameter isn't a letter, it will return None
pub(crate) fn is_vowel(letter: char) -> Option<bool> {
    if !letter.is_alphabetic() {
        return None;
    }

    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return Some(true); }
        _ => { return Some(false); }
    }
}

//Returns whether a letter is y or not.
//
//If the parameter is a letter, returns Some(true) if it is y, and Some(false) otherwise.
//If the parameter isn't a letter, it will return None
pub(crate) fn is_y(letter: char) -> Option<bool> {
    if !letter.is_alphabetic() {
        return None;
    }

    return Some(letter.to_ascii_lowercase() == 'y');
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        for letter in "aeiouAEIOU".chars() {
            assert!(is_vowel(letter).unwrap());
        }

        for letter in "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars() {
            assert!(!is_vowel(letter).unwrap());
        }

        for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(matches!(is_vowel(not_letter), None));
        }
    }

    #[test]
    fn test_is_y() {
        for letter in "yY".chars() {
            assert!(is_y(letter).unwrap());
        }

        for letter in "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".chars() {
            assert!(!is_y(letter).unwrap());
        }

        for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(matches!(is_y(not_letter), None));
        }
    }
}
