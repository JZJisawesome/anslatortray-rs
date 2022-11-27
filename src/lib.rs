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
///# Examples
///
///```
///assert_eq!(anslatortray::translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(anslatortray::translate("This library can translate any English text. It can even handle multiple sentences!"),
///    if anslatortray::VOWEL_START_STYLE == "way" { "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!" }
///    else if anslatortray::VOWEL_START_STYLE == "yay" { "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!" }
///    else { panic!(); }
///);
///```
pub fn translate(english: &str) -> String {
    if english.is_empty() {
        return "".to_string();
    }

    let mut pig_latin_string: String = "".to_string();

    let mut first_iteration: bool = true;
    for word in english.split(&[' ', '\t', '\n']) {
        if !first_iteration { pig_latin_string.push(' '); }//Seperate words by spaces regardless of how they were seperated before
        pig_latin_string.push_str(translate_word(word).as_str());
        first_iteration = false;
    }

    return pig_latin_string;

    /*
    let mut pig_latin_string: String = "".to_string();
    let mut current_word: String = "".to_string();
    let mut in_word: bool = false;

    for character in english.chars() {
        if in_word {
            if character.is_alphabetic() {
                current_word.push(character);
            } else {
                in_word = false;
                pig_latin_string.push_str(translate_word(current_word.as_str()).unwrap().as_str());
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
    */
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
///assert_eq!(anslatortray::translate_word("Hello").unwrap(), "Ellohay".to_string());
///assert_eq!(anslatortray::translate_word("World").unwrap(), "Orldway".to_string());
///assert_eq!(anslatortray::translate_word("This").unwrap(), "Isthay".to_string());
///assert_eq!(anslatortray::translate_word("is").unwrap(), "is".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word("a").unwrap(), "a".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word("test").unwrap(), "esttay".to_string());
///assert_eq!(anslatortray::translate_word("of").unwrap(), "of".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word("the").unwrap(), "ethay".to_string());
///assert_eq!(anslatortray::translate_word("function").unwrap(), "unctionfay".to_string());
///assert_eq!(anslatortray::translate_word("translate").unwrap(), "anslatetray".to_string());
///assert_eq!(anslatortray::translate_word("word").unwrap(), "ordway".to_string());
///assert_eq!(anslatortray::translate_word("I").unwrap(), "I".to_string() + &anslatortray::VOWEL_START_STYLE.to_string());
///assert_eq!(anslatortray::translate_word("Love").unwrap(), "Ovelay".to_string());
///assert_eq!(anslatortray::translate_word("Pig").unwrap(), "Igpay".to_string());
///assert_eq!(anslatortray::translate_word("Latin").unwrap(), "Atinlay".to_string());
///assert!(matches!(anslatortray::translate_word("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"), None));//No vowels
///assert!(matches!(anslatortray::translate_word(""), None));//No letters at all
///assert!(matches!(anslatortray::translate_word("Multiple Words"), None));
///assert!(matches!(anslatortray::translate_word("wordwithpunctuation!"), None));
///assert!(matches!(anslatortray::translate_word(" "), None));//Single space/punctuation/symbol
///assert!(matches!(anslatortray::translate_word(" !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n"), None));//Lots of symbols
///assert!(matches!(anslatortray::translate_word("You're"), None));//Does not handle contractions
///assert!(matches!(anslatortray::translate_word("Try"), None));//Does not consider y to be a vowel, ever
///```
fn translate_word(english_word: &str) -> String {
    if english_word.is_empty() {
        return "".to_string();
    }

    let mut pig_latin_word: String = "".to_string();
    let mut iterator = english_word.chars();

    //Copy leading symbols/whitespace until the first letter
    let mut first_letter: char;
    loop {
        match iterator.next() {
            None => { return english_word.to_string(); },//There are only symbols/whitespace in the word
            Some(character) => {
                if character.is_alphabetic() {
                    first_letter = character;//We found the first character of the word/contraction
                    break;
                } else {
                    pig_latin_word.push(character);//Copy whitespace/symbol
                }
            }
        }
    }

    //TODO what if first character is an apostrophe? (like 'cause)
    //TODO what if the word is all uppercase?
    let first_letter_was_vowel: bool = is_vowel(first_letter).unwrap();//As a herustic, we consider Y to be a vowel when it is not at the start of the word
    let mut starting_consonants: String = "".to_string();

    if first_letter_was_vowel {
        pig_latin_word.push(first_letter);
    } else {
        let first_char_was_upper = first_letter.is_ascii_uppercase();
        starting_consonants.push(first_letter.to_ascii_lowercase());

        //Grab all of the starting consonants, and push the first vowel we enounter to pig_latin_word
        loop {
            match iterator.next() {
                None => { break; },//The word has no vowels, but it is a herustic to pass it on so that ex. the acroynm binary code decimal or bcd becomes bcdway, etc.
                Some(character) => {
                    if character.is_alphabetic() {
                        if is_vowel(character).unwrap() || is_y(character).unwrap() {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
                            //The vowel is the first letter of the word; we want it match the capitalization of the first letter of the original word
                            if first_char_was_upper {
                                pig_latin_word.push(character.to_ascii_uppercase());
                            } else {
                                pig_latin_word.push(character.to_ascii_lowercase());
                            }
                            break;
                        } else {
                            starting_consonants.push(character);
                        }
                        first_letter = character;//We found the first character of the word/contraction
                    } else {//The word ended without vowels or we met an apostrophe
                        break;//It is a herustic to pass it on so that ex. the letter y becomes yway, the word a becomes away, etc.
                    }
                }
            }
        }
    }

    //Copy all of the remaining letters up to the end of the word or up until we enounter the ' as part of a contraction
    let trailing_character: Option<char>;
    loop {
        match iterator.next() {
            None => {
                trailing_character = None;
                break;
            },//End of the word
            Some(character) => {
                if character.is_alphabetic() {
                    pig_latin_word.push(character);
                } else {
                    trailing_character = Some(character);
                    break;
                }
            }
        }
    }

    //Copy starting consonants and add ay, or add the VOWEL_START_STYLE depending on the circumstances
    if first_letter_was_vowel || (starting_consonants == "") {//Words that start with vowels or that are only a single letter get the VOWEL_START_STYLE
        pig_latin_word.push_str(VOWEL_START_STYLE);
    } else {
        pig_latin_word.push_str(&starting_consonants);
        pig_latin_word.push_str("ay");
    }

    //Re-add the trailing character we "accidentally" took in the previous loop (if we do in fact have one)
    if let Some(character) = trailing_character {
        pig_latin_word.push(character);
    }

    //Copy any remaining characters as-is
    loop {
        match iterator.next() {
            None => { break; },//End of the word
            Some(character) => { pig_latin_word.push(character); },
        }
    }

    return pig_latin_word;

    /*let mut first_letter: char;
    while let Some(character) = iterator.next() {
        if character.is_alphabetic() {
            first_letter = character;
            break;
        } else {
            pig_latin_word.push(character);
        }
    }
    if matches!(iterator.next(), None) {//No letters at all!
        return pig_latin_word;
    }

    if is_vowel(first_letter).unwrap() {//Not including y
        pig_latin_string.push(first_letter);
        pig_latin_string.push_str(VOWEL_START_STYLE);
        //TODO need to handle contractions
        return Some(pig_latin_string);
    }

    let mut starting_part_of_word: String = first_char.to_ascii_lowercase().to_string();

    let mut char_after_last_letter: char;
    while let Some(character) = iterator.next() {
        if character.is_alphabetic() {
            first_letter = character;
            break;
        } else {
            pig_latin_word.push(character);
        }
    }
    */

    //for letter in english_word.chars() {

    /*let mut iterator = english_word.chars();
    let first_char: char = iterator.next().unwrap();

    if !first_char.is_alphabetic() {
        return None;
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
    */
}

///Returns whether a letter is a vowel or not.
///
///If the parameter is a letter, returns Some(true) if it is a vowel, and Some(false) otherwise.
///If the parameter isn't a letter, it will return None
///
///# Examples
///
///```ignore
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
fn is_vowel(letter: char) -> Option<bool> {
    if !letter.is_alphabetic() {
        return None;
    }

    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return Some(true); }
        _ => { return Some(false); }
    }
}

///Returns whether a letter is y or not.
///
///If the parameter is a letter, returns Some(true) if it is y, and Some(false) otherwise.
///If the parameter isn't a letter, it will return None
///
///# Examples
///
///```ignore
///for letter in "yY".chars() {
///    assert!(anslatortray::is_y(letter).unwrap());
///}
///
///for letter in "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".chars() {
///    assert!(!anslatortray::is_y(letter).unwrap());
///}
///
///for not_letter in " !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n".chars() {
///    assert!(matches!(anslatortray::is_y(not_letter), None));
///}
///```
fn is_y(letter: char) -> Option<bool> {
    if !letter.is_alphabetic() {
        return None;
    }

    return Some(letter.to_ascii_lowercase() == 'y');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
            if VOWEL_START_STYLE == "way" { "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!" }
            else if VOWEL_START_STYLE == "yay" { "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!" }
            else { panic!(); }
        );
    }

    #[test]
    fn test_translate_edgecases() {
        assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            if VOWEL_START_STYLE == "way" { "Etlay's ytray omesya edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!" }
            else if VOWEL_START_STYLE == "yay" { "Etlay's ytray omesya edgeway asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allway atthay orksway!" }
            else { panic!(); }
        );
        assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"),
            if VOWEL_START_STYLE == "way" { "Atwhay ifway away ordway ashay onay owelvay, ikelay isthay: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ" }
            else if VOWEL_START_STYLE == "yay" { "Atwhay ifyay ayay ordway ashay onay owelvay, ikelay isthay: bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ" }
            else { panic!(); }
        );
    }

    #[test]
    fn test_translate_word() {
        assert_eq!(translate_word("Hello"), "Ellohay");
        assert_eq!(translate_word("World"), "Orldway");
        assert_eq!(translate_word("This"), "Isthay");
        assert_eq!(translate_word("is"), "is".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("a"), "a".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("test"), "esttay".to_string());
        assert_eq!(translate_word("of"), "of".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("the"), "ethay");
        assert_eq!(translate_word("function"), "unctionfay");
        assert_eq!(translate_word("translate"), "anslatetray");
        assert_eq!(translate_word("word"), "ordway");
        assert_eq!(translate_word("I"), "I".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("Love"), "Ovelay");
        assert_eq!(translate_word("Pig"), "Igpay");
        assert_eq!(translate_word("Latin"), "Atinlay");
        /*assert!(matches!(translate_word("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ"), None));//No vowels
        assert!(matches!(translate_word(""), None));//No letters at all
        assert!(matches!(translate_word("Multiple Words"), None));
        assert!(matches!(translate_word("wordwithpunctuation!"), None));
        assert!(matches!(translate_word(" "), None));//Single space/punctuation
        assert!(matches!(translate_word(" !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n"), None));//Lots of symbols
        assert!(matches!(translate_word("You're"), None));//Does not handle contractions
        assert!(matches!(translate_word("Try"), None));//Does not consider y to be a vowel, ever
        */
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

    #[test]
    fn test_is_y() {
        for letter in "yY".chars() {
            assert!(is_y(letter).unwrap());
        }

        for letter in "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".chars() {
            assert!(!is_y(letter).unwrap());
        }

        for not_letter in " !@#$%^&*()_+{}|\":>?~`\\][';/.,\t\n".chars() {
            assert!(matches!(is_y(not_letter), None));
        }
    }
}
