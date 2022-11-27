//!Anslatortray for Rust
//!
//!Welcome to the Anslatortray Documentation!
//!
//!A simple Rust library to translate from English to Pig Latin.
//!
//!
//!<a href="https://en.wikipedia.org/wiki/Pig_Latin">Wikipedia's definition of Pig Latin</a> is "a language game or argot in which words in English are altered, usually by adding a fabricated suffix or by moving the onset or initial consonant or consonant cluster of a word to the end of the word and adding a vocalic syllable to create such a suffix."
//!
//!Essentially, the word is reorganized in an effort to hide its true meaning, which can be lots of fun!
//!
//!The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily.
//!
//!You can translate multiple sentences, including numbers, punctuation, and spacing, with a single call to [`translate()`].
//!The function handles edge cases quite well (words without vowels, one-letter words, contractions, etc.), though there is always room for improvement.
//!
//!If you have suggestions for how the project could be improved, please visit the repository's issues page on <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> or contact me directly :)
//!
//!# Examples
//!
//!Try compiling this example code:
//!
//!```
//!use anslatortray::translate;
//!
//!//Prints "Ellohay orldway omfray ethay Anslatortray orfay Ustray!"
//!println!("{}", translate("Hello world from the Translator for Rust!"));
//!```
//!
//!# Useful Links
//!<a href="https://git.jekel.ca/JZJ/anslatortray-rs">Click here to visit the Anslatortray for Rust Git Repository!</a>.
//!
//!You can also visit the <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> mirrors to leave issues!
//!
//!Anslatortray for Rust is a spiritual sucessor of my original <a href="https://git.jekel.ca/JZJ/anslatortray">Anslatortray</a> (for C++).
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

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///# Examples
///
///```
///use anslatortray::{translate, VOWEL_START_STYLE};
///
///assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
///    if VOWEL_START_STYLE == "way" { "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!" }
///    else if VOWEL_START_STYLE == "yay" { "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!" }
///    else { panic!(); }
///);
///
///assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    if VOWEL_START_STYLE == "way" { "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!" }
///    else if VOWEL_START_STYLE == "yay" { "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!" }
///    else { panic!(); }
///);
///assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    if VOWEL_START_STYLE == "way" { "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay" }
///    else if VOWEL_START_STYLE == "yay" { "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay" }
///     else { panic!(); }
///    );
///    assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///    );
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
}

///The suffix appended to a word the starts with a vowel instead of the usual "ay"
///
///Commonly this is either "way" or "yay". I prefer the former, but you can choose
///between the two by specifying the feature "way" or "yay" respectively in Cargo.
///
///You can also choose from more exotic endings, like "werb" or "yerb" for Ferb latin, though
///you'll have to modify this file (src/lib.rs) to change this manually. This includes changes
///to the normal suffix in [`translate_word()`] and updates to the tests.
///
///If you'd like to see another ending available as a Cargo feature, contact me and I'll implement it
pub const VOWEL_START_STYLE: &str = "way";//TODO make this configurable via a Cargo feature

///Translates a single word or contraction string into Pig Latin!
///
///Can have leading and trailing punctuation or whitespace.
///It generally does a pretty good job with valid english words and contractions,
///and leaves symbols and spaces mostly unchanged.
///
///This is a helper function used by [`translate()`], but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///use anslatortray::{translate_word, VOWEL_START_STYLE};
///
///assert_eq!(translate_word("Hello"), "Ellohay");
///assert_eq!(translate_word("World!"), "Orldway!");
///
///assert_eq!(translate_word("This"), "Isthay");
///assert_eq!(translate_word("is"), "is".to_string() + &VOWEL_START_STYLE.to_string());
///assert_eq!(translate_word("a"), "a".to_string() + &VOWEL_START_STYLE.to_string());
///assert_eq!(translate_word("test"), "esttay".to_string());
///assert_eq!(translate_word("of"), "of".to_string() + &VOWEL_START_STYLE.to_string());
///assert_eq!(translate_word("the"), "ethay");
///assert_eq!(translate_word("function"), "unctionfay");
///assert_eq!(translate_word("translate_"), "anslatetray_");
///assert_eq!(translate_word("word."), "ordway.");
///
///assert_eq!(translate_word("I"), "I".to_string() + &VOWEL_START_STYLE.to_string());
///assert_eq!(translate_word("Love"), "Ovelay");
///assert_eq!(translate_word("Pig"), "Igpay");
///assert_eq!(translate_word("Latin!"), "Atinlay!");
///
///assert_eq!(translate_word("You"), "Ouyay");//Y isn't a vowel here
///assert_eq!(translate_word("should"), "ouldshay");
///assert_eq!(translate_word("try"), "ytray");//Y is a vowel here
///assert_eq!(translate_word("yougurt,"), "ougurtyay,");//Y isn't a vowel here
///assert_eq!(translate_word("it's"), "it".to_string() + &VOWEL_START_STYLE.to_string() + "'s");//Contraction
///assert_eq!(translate_word("quite"), "uiteqay");//Awful to pronounce, but correct
///assert_eq!(translate_word("nice!"), "icenay!");
///
///assert_eq!(translate_word(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word("12345678"), "12345678");//A number
///assert_eq!(translate_word("100 pizzas"), "100 izzaspay");//A number before a word
///assert_eq!(translate_word("over 9000"), "over".to_string() + &VOWEL_START_STYLE.to_string() + " 9000");//A number after a word
///```
pub fn translate_word(english_word: &str) -> String {
    if english_word.is_empty() {
        return "".to_string();
    }

    let mut pig_latin_word: String = "".to_string();
    let mut iterator = english_word.chars().peekable();

    //Copy leading symbols/whitespace until the first letter
    let first_letter: char;
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

    //TODO what if the word is all uppercase?

    //As a herustic, we consider Y to be a vowel when it is not at the start of the word
    //However, if any word is only one letter long, this takes priority and the word is treated like a vowel
    let first_letter_was_vowel: bool = {
        is_vowel(first_letter).unwrap()//Not including y
        || if let Some(character) = iterator.peek() { !character.is_alphabetic() } else { false }//Non-alphabetic character or word ends after the first letter
    };
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
    if first_letter_was_vowel {
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
}

///Returns whether a letter is a vowel or not.
///
///If the parameter is a letter, returns Some(true) if it is a vowel, and Some(false) otherwise.
///If the parameter isn't a letter, it will return None
///
///This is a helper function used by [`translate_word()`], but
///it is publically exposed as potential users may find this useful.
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
///for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,".chars() {
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

///Returns whether a letter is y or not.
///
///If the parameter is a letter, returns Some(true) if it is y, and Some(false) otherwise.
///If the parameter isn't a letter, it will return None
///
///This is a helper function used by [`translate_word()`], but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///for letter in "yY".chars() {
///    assert!(anslatortray::is_y(letter).unwrap());
///}
///
///for letter in "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".chars() {
///    assert!(!anslatortray::is_y(letter).unwrap());
///}
///
///for not_letter in " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".chars() {
///    assert!(matches!(anslatortray::is_y(not_letter), None));
///}
///```
pub fn is_y(letter: char) -> Option<bool> {
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
            if VOWEL_START_STYLE == "way" { "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!" }
            else if VOWEL_START_STYLE == "yay" { "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!" }
            else { panic!(); }
        );
        assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            if VOWEL_START_STYLE == "way" { "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay" }
            else if VOWEL_START_STYLE == "yay" { "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay" }
            else { panic!(); }
        );
        assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
        );
    }

    #[test]
    fn test_translate_word() {
        assert_eq!(translate_word("Hello"), "Ellohay");
        assert_eq!(translate_word("World!"), "Orldway!");

        assert_eq!(translate_word("This"), "Isthay");
        assert_eq!(translate_word("is"), "is".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("a"), "a".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("test"), "esttay".to_string());
        assert_eq!(translate_word("of"), "of".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("the"), "ethay");
        assert_eq!(translate_word("function"), "unctionfay");
        assert_eq!(translate_word("translate_"), "anslatetray_");
        assert_eq!(translate_word("word."), "ordway.");

        assert_eq!(translate_word("I"), "I".to_string() + &VOWEL_START_STYLE.to_string());
        assert_eq!(translate_word("Love"), "Ovelay");
        assert_eq!(translate_word("Pig"), "Igpay");
        assert_eq!(translate_word("Latin!"), "Atinlay!");

        assert_eq!(translate_word("You"), "Ouyay");//Y isn't a vowel here
        assert_eq!(translate_word("should"), "ouldshay");
        assert_eq!(translate_word("try"), "ytray");//Y is a vowel here
        assert_eq!(translate_word("yougurt,"), "ougurtyay,");//Y isn't a vowel here
        assert_eq!(translate_word("it's"), "it".to_string() + &VOWEL_START_STYLE.to_string() + "'s");//Contraction
        assert_eq!(translate_word("quite"), "uiteqay");//Awful to pronounce, but correct
        assert_eq!(translate_word("nice!"), "icenay!");

        assert_eq!(translate_word(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
        assert_eq!(translate_word(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
        assert_eq!(translate_word("12345678"), "12345678");//A number
        assert_eq!(translate_word("100 pizzas"), "100 izzaspay");//A number before a word
        assert_eq!(translate_word("over 9000"), "over".to_string() + &VOWEL_START_STYLE.to_string() + " 9000");//A number after a word
    }

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
