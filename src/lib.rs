//!Anslatortray for Rust
//!
//!Welcome to the Anslatortray Documentation!
//!
//!A simple, Rust library to translate from English to Pig Latin.
//!
//!
//!<a href="https://en.wikipedia.org/wiki/Pig_Latin">Wikipedia's definition of Pig Latin</a> is "a language game or argot in which words in English are altered, usually by adding a fabricated suffix or by moving the onset or initial consonant or consonant cluster of a word to the end of the word and adding a vocalic syllable to create such a suffix."
//!
//!Essentially, the word is reorganized in an effort to hide its true meaning, which can be lots of fun!
//!The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily.
//!
//!# Examples
//!
//!Try compiling this example code:
//!
//!```
//!//TODO example code
//!```
//!
//!# Useful Links
//!<a href="https://git.jekel.ca/JZJ/anslatortray-rs">Click here to visit the Anslatortray for Rust Git Repository!</a>.
//!
//!Anslatortray for Rust is a spiritual sucessor of the original <a href="https://git.jekel.ca/JZJ/anslatortray">Anslatortray</a> (for C++).
//!
//!# Anslatortray Code and Documentation Licence
//!MIT License
//!
//!Copyright (c) 2022 John Jekel
//!
//!Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//!
//!The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//!
//!THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

///The suffix appended to a word the starts with a vowel instead of the usual "ay"
///
///Commonly this is either "way" or "yay". I prefer the former, but you can choose
///between the two by specifying the feature "way" or "yay" respectively in Cargo.
pub const VOWEL_START_STYLE: &str = "way";//TODO make this configurable via a Cargo feature

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Does not handle some edge cases however.
///Use this over [`anslatortray::translate()`] only if you really need a ton of speed
///
///# Examples
///
///```
///assert_eq!(anslatortray::translate_dumb("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(anslatortray::translate_dumb("This library can translate any English text. It can even handle multiple sentences!"),
///    if anslatortray::VOWEL_START_STYLE == "way" { "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!" }
///    else if anslatortray::VOWEL_START_STYLE == "yay" { "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!" }
///    else { panic!(); }
///);
///```
pub fn translate_dumb(english: &str) -> String {
    if english.is_empty() {
        return "".to_string();
    }

    let mut pig_latin_string: String = "".to_string();
    let mut current_word: String = "".to_string();
    let mut in_word: bool = false;

    for character in english.chars() {
        if in_word {
            if character.is_alphabetic() {
                current_word.push(character);
            } else {
                in_word = false;
                pig_latin_string.push_str(translate_word_dumb(current_word.as_str()).unwrap().as_str());
                pig_latin_string.push(character);
            }
        } else {
            if character.is_alphabetic() {
                in_word = true;
                current_word = character.to_string();
            } else {
                pig_latin_string.push(character);
            }
        }
    }

    return pig_latin_string;
}

///Translates a single word string into Pig Latin!
///
///Must only contain letters; no punctuation or spaces.
///Also requires at minimum one vowel, and must be non-empty.
///
///If all of these are satasfied, then this returns Some(String) containing the translated word.
///Otherwise it returns None
///
//Use this over [`anslatortray::translate_word()`] if you really need a ton of speed
///
///# Examples
///
///```
///assert_eq!(anslatortray::translate_word_dumb("Hello").unwrap(), "Ellohay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("World").unwrap(), "Orldway".to_string());
///assert_eq!(anslatortray::translate_word_dumb("This").unwrap(), "Isthay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("is").unwrap(), "is".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word_dumb("a").unwrap(), "a".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word_dumb("test").unwrap(), "esttay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("of").unwrap(), "of".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word_dumb("the").unwrap(), "ethay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("function").unwrap(), "unctionfay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("translate").unwrap(), "anslatetray".to_string());
///assert_eq!(anslatortray::translate_word_dumb("word").unwrap(), "ordway".to_string());
///assert_eq!(anslatortray::translate_word_dumb("I").unwrap(), "I".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word_dumb("Love").unwrap(), "Ovelay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("Pig").unwrap(), "Igpay".to_string());
///assert_eq!(anslatortray::translate_word_dumb("Latin").unwrap(), "Atinlay".to_string());
///assert!(matches!(anslatortray::translate_word_dumb("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"), None));//No vowels
///assert!(matches!(anslatortray::translate_word_dumb(""), None));//No letters at all
///assert!(matches!(anslatortray::translate_word_dumb("Multiple Words"), None));
///assert!(matches!(anslatortray::translate_word_dumb("wordwithpunctuation!"), None));
///assert!(matches!(anslatortray::translate_word_dumb(" "), None));//Single space/punctuation/symbol
///assert!(matches!(anslatortray::translate_word_dumb(" !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n"), None));//Lots of symbols
///assert!(matches!(anslatortray::translate_word_dumb("You're"), None));//Does not handle contractions
///assert!(matches!(anslatortray::translate_word_dumb("Try"), None));//Does not consider y to be a vowel, ever
///```
pub fn translate_word_dumb(english_word: &str) -> Option<String> {
    if english_word.is_empty() {
        return None;
    }

    let mut iterator = english_word.chars();
    let first_char: char = iterator.next().unwrap();

    if !first_char.is_alphabetic() {
        return None;
    }

    if is_vowel(first_char).unwrap() {
        let mut pig_latin_string = english_word.to_string();
        pig_latin_string.push_str(VOWEL_START_STYLE);
        return Some(pig_latin_string);
    }

    let mut first_char_was_upper = first_char.is_ascii_uppercase();

    let mut vowel_encountered: bool = false;
    let mut starting_part_of_word: String = first_char.to_ascii_lowercase().to_string();
    let mut pig_latin_word: String = "".to_string();
    for letter in iterator {
        if !letter.is_alphabetic() {
            return None;
        }

        if vowel_encountered {
            pig_latin_word.push(letter);
        } else {
            if is_vowel(letter).unwrap() {
                if first_char_was_upper {
                    pig_latin_word.push(letter.to_ascii_uppercase())
                } else {
                    pig_latin_word.push(letter.to_ascii_lowercase())
                }
                vowel_encountered = true;
            } else {
                starting_part_of_word.push(letter.to_ascii_lowercase());
            }
        }
    }

    if !vowel_encountered {
        return None;
    }

    pig_latin_word.push_str(&starting_part_of_word);
    pig_latin_word.push_str("ay");

    return Some(pig_latin_word);
}

///Returns whether a letter is a vowel or not.
///
///If the parameter is a letter, returns Some(true) if it is a vowel, and Some(false) otherwise.
///If the parameter isn't a letter, it will return None
///
///# Examples
///
///```
///for letter in "aeiouAEIOU".chars() {
///    assert!(anslatortray::is_vowel(letter).unwrap());
///}
///
///for letter in "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars() {
///    assert!(!anslatortray::is_vowel(letter).unwrap());
///}
///
///for not_letter in " !@#$%^&*()_+{}|\":>?~`\\][';/.,".chars() {
///    assert!(matches!(anslatortray::is_vowel(not_letter), None));
///}
///```
pub fn is_vowel(letter: char) -> Option<bool> {
    if !letter.is_alphabetic() {
        return None;
    }

    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return Some(true); }
        _ => { return Some(false); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_dumb() {
        assert_eq!(translate_dumb("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate_dumb("This library can translate any English text. It can even handle multiple sentences!"),
            if VOWEL_START_STYLE == "way" { "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!" }
            else if VOWEL_START_STYLE == "yay" { "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!" }
            else { panic!(); }
        );
    }

    #[test]
    fn test_translate() {
        todo!();
        /*assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            if VOWEL_START_STYLE == "way" { "Etlay's ytray omesya edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!" }
            else if VOWEL_START_STYLE == "yay" { "Etlay's ytray omesya edgeway asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allway atthay orksway!" }
            else { panic!(); }
        );
        assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"),
            if VOWEL_START_STYLE == "way" { "Atwhay ifway away ordway ashay onay owelvay, ikelay isthay: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ" }
            else if VOWEL_START_STYLE == "yay" { "Atwhay ifyay ayay ordway ashay onay owelvay, ikelay isthay: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ" }
            else { panic!(); }
        );
        */
    }

    #[test]
    fn test_translate_word_dumb() {
        assert_eq!(translate_word_dumb("Hello").unwrap(), "Ellohay");
        assert_eq!(translate_word_dumb("World").unwrap(), "Orldway");
        assert_eq!(translate_word_dumb("This").unwrap(), "Isthay");
        assert_eq!(translate_word_dumb("is").unwrap(), "is".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word_dumb("a").unwrap(), "a".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word_dumb("test").unwrap(), "esttay".to_string());
        assert_eq!(translate_word_dumb("of").unwrap(), "of".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word_dumb("the").unwrap(), "ethay");
        assert_eq!(translate_word_dumb("function").unwrap(), "unctionfay");
        assert_eq!(translate_word_dumb("translate").unwrap(), "anslatetray");
        assert_eq!(translate_word_dumb("word").unwrap(), "ordway");
        assert_eq!(translate_word_dumb("I").unwrap(), "I".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word_dumb("Love").unwrap(), "Ovelay");
        assert_eq!(translate_word_dumb("Pig").unwrap(), "Igpay");
        assert_eq!(translate_word_dumb("Latin").unwrap(), "Atinlay");
        assert!(matches!(translate_word_dumb("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"), None));//No vowels
        assert!(matches!(translate_word_dumb(""), None));//No letters at all
        assert!(matches!(translate_word_dumb("Multiple Words"), None));
        assert!(matches!(translate_word_dumb("wordwithpunctuation!"), None));
        assert!(matches!(translate_word_dumb(" "), None));//Single space/punctuation
        assert!(matches!(translate_word_dumb(" !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n"), None));//Lots of symbols
        assert!(matches!(translate_word_dumb("You're"), None));//Does not handle contractions
        assert!(matches!(translate_word_dumb("Try"), None));//Does not consider y to be a vowel, ever
    }

    #[test]
    fn test_is_vowel() {
        for letter in "aeiouAEIOU".chars() {
            assert!(is_vowel(letter).unwrap());
        }

        for letter in "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars() {
            assert!(!is_vowel(letter).unwrap());
        }

        for not_letter in " !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(matches!(is_vowel(not_letter), None));
        }
    }
}
