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

/* Functions */

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_word_with_style()`].
///
///Equivalent to [`translate_way()`].
///
///# Examples
///
///```
///use anslatortray::translate;
///
///assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate(english: &str) -> String {
    return translate_way(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style)!
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_word_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_way;
///
///assert_eq!(translate_way("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_way("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_way("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate_way("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate_way("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate_way(english: &str) -> String {
    return translate_with_style(english, "ay", "way");
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_word_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_yay;
///
///assert_eq!(translate_yay("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_yay("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_yay("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!"
///);
///
///assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate_yay(english: &str) -> String {
    return translate_with_style(english, "ay", "yay");
}

///Translates a multi-word string (including punctuation) into a custom-styled play language!
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///# Examples
///
///```
/////TODO
///```
pub fn translate_with_style(english: &str, suffix: &str, special_case_suffix: &str) -> String {
    if english.is_empty() {
        return "".to_string();
    }

    let mut pig_latin_string: String = "".to_string();

    let mut first_iteration: bool = true;
    for word in english.split(&[' ', '\t', '\n']) {
        if !first_iteration { pig_latin_string.push(' '); }//Seperate words by spaces regardless of how they were seperated before
        pig_latin_string.push_str(translate_word_with_style(word, suffix, special_case_suffix).as_str());
        first_iteration = false;
    }

    return pig_latin_string;
}

///Translates a single word or contraction string into Pig Latin!
///
///Can have leading and trailing punctuation or whitespace.
///It generally does a pretty good job with valid english words and contractions,
///and leaves symbols and spaces mostly unchanged.
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_word_with_style()`].
///
///Equivalent to [`translate_word_way()`].
///
///This is a helper function used by the [`translate()`] family of functions, but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///use anslatortray::translate_word;
///
///assert_eq!(translate_word("Hello"), "Ellohay");
///assert_eq!(translate_word("World!"), "Orldway!");
///
///assert_eq!(translate_word("This"), "Isthay");
///assert_eq!(translate_word("is"), "isway");
///assert_eq!(translate_word("a"), "away");
///assert_eq!(translate_word("test"), "esttay");
///assert_eq!(translate_word("of"), "ofway");
///assert_eq!(translate_word("the"), "ethay");
///assert_eq!(translate_word("function"), "unctionfay");
///assert_eq!(translate_word("translate_"), "anslatetray_");
///assert_eq!(translate_word("word."), "ordway.");
///
///assert_eq!(translate_word("I"), "Iway");
///assert_eq!(translate_word("Love"), "Ovelay");
///assert_eq!(translate_word("Pig"), "Igpay");
///assert_eq!(translate_word("Latin!"), "Atinlay!");
///
///assert_eq!(translate_word("You"), "Ouyay");//Y isn't a vowel here
///assert_eq!(translate_word("should"), "ouldshay");
///assert_eq!(translate_word("try"), "ytray");//Y is a vowel here
///assert_eq!(translate_word("yougurt,"), "ougurtyay,");//Y isn't a vowel here
///assert_eq!(translate_word("it's"), "itway's");//Contraction
///assert_eq!(translate_word("quite"), "uiteqay");//Awful to pronounce, but correct
///assert_eq!(translate_word("nice!"), "icenay!");
///
///assert_eq!(translate_word(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word("12345678"), "12345678");//A number
///assert_eq!(translate_word("100 pizzas"), "100 izzaspay");//A number before a word
///assert_eq!(translate_word("over 9000"), "overway 9000");//A number after a word
///```
pub fn translate_word(english_word: &str) -> String {
    return translate_word_way(english_word);
}

///Translates a single word or contraction string into Pig Latin (way-style)!
///
///Can have leading and trailing punctuation or whitespace.
///It generally does a pretty good job with valid english words and contractions,
///and leaves symbols and spaces mostly unchanged.
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_word_with_style()`].
///
///This is a helper function used by the [`translate()`] family of functions, but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///use anslatortray::translate_word_way;
///
///assert_eq!(translate_word_way("Hello"), "Ellohay");
///assert_eq!(translate_word_way("World!"), "Orldway!");
///
///assert_eq!(translate_word_way("This"), "Isthay");
///assert_eq!(translate_word_way("is"), "isway");
///assert_eq!(translate_word_way("a"), "away");
///assert_eq!(translate_word_way("test"), "esttay");
///assert_eq!(translate_word_way("of"), "ofway");
///assert_eq!(translate_word_way("the"), "ethay");
///assert_eq!(translate_word_way("function"), "unctionfay");
///assert_eq!(translate_word_way("translate_"), "anslatetray_");
///assert_eq!(translate_word_way("word."), "ordway.");
///
///assert_eq!(translate_word_way("I"), "Iway");
///assert_eq!(translate_word_way("Love"), "Ovelay");
///assert_eq!(translate_word_way("Pig"), "Igpay");
///assert_eq!(translate_word_way("Latin!"), "Atinlay!");
///
///assert_eq!(translate_word_way("You"), "Ouyay");//Y isn't a vowel here
///assert_eq!(translate_word_way("should"), "ouldshay");
///assert_eq!(translate_word_way("try"), "ytray");//Y is a vowel here
///assert_eq!(translate_word_way("yougurt,"), "ougurtyay,");//Y isn't a vowel here
///assert_eq!(translate_word_way("it's"), "itway's");//Contraction
///assert_eq!(translate_word_way("quite"), "uiteqay");//Awful to pronounce, but correct
///assert_eq!(translate_word_way("nice!"), "icenay!");
///
///assert_eq!(translate_word_way(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word_way(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word_way("12345678"), "12345678");//A number
///assert_eq!(translate_word_way("100 pizzas"), "100 izzaspay");//A number before a word
///assert_eq!(translate_word_way("over 9000"), "overway 9000");//A number after a word
///```
pub fn translate_word_way(english_word: &str) -> String {
    return translate_word_with_style(english_word, "ay", "way");
}

///Translates a single word or contraction string into Pig Latin (yay-style)!
///
///Can have leading and trailing punctuation or whitespace.
///It generally does a pretty good job with valid english words and contractions,
///and leaves symbols and spaces mostly unchanged.
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_word_with_style()`].
///
///This is a helper function used by the [`translate()`] family of functions, but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///use anslatortray::translate_word_yay;
///
///assert_eq!(translate_word_yay("Hello"), "Ellohay");
///assert_eq!(translate_word_yay("World!"), "Orldway!");
///
///assert_eq!(translate_word_yay("This"), "Isthay");
///assert_eq!(translate_word_yay("is"), "isyay");
///assert_eq!(translate_word_yay("a"), "ayay");
///assert_eq!(translate_word_yay("test"), "esttay");
///assert_eq!(translate_word_yay("of"), "ofyay");
///assert_eq!(translate_word_yay("the"), "ethay");
///assert_eq!(translate_word_yay("function"), "unctionfay");
///assert_eq!(translate_word_yay("translate_"), "anslatetray_");
///assert_eq!(translate_word_yay("word."), "ordway.");
///
///assert_eq!(translate_word_yay("I"), "Iyay");
///assert_eq!(translate_word_yay("Love"), "Ovelay");
///assert_eq!(translate_word_yay("Pig"), "Igpay");
///assert_eq!(translate_word_yay("Latin!"), "Atinlay!");
///
///assert_eq!(translate_word_yay("You"), "Ouyay");//Y isn't a vowel here
///assert_eq!(translate_word_yay("should"), "ouldshay");
///assert_eq!(translate_word_yay("try"), "ytray");//Y is a vowel here
///assert_eq!(translate_word_yay("yougurt,"), "ougurtyay,");//Y isn't a vowel here
///assert_eq!(translate_word_yay("it's"), "ityay's");//Contraction
///assert_eq!(translate_word_yay("quite"), "uiteqay");//Awful to pronounce, but correct
///assert_eq!(translate_word_yay("nice!"), "icenay!");
///
///assert_eq!(translate_word_yay(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word_yay(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word_yay("12345678"), "12345678");//A number
///assert_eq!(translate_word_yay("100 pizzas"), "100 izzaspay");//A number before a word
///assert_eq!(translate_word_yay("over 9000"), "overyay 9000");//A number after a word
///```
pub fn translate_word_yay(english_word: &str) -> String {
    return translate_word_with_style(english_word, "ay", "yay");
}

///Translates a single word or contraction string into a custom-styled play language!
///
///Pass the word you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///# Examples
///
///```
/////TODO
///```
pub fn translate_word_with_style(english_word: &str, suffix: &str, special_case_suffix: &str) -> String {
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

    //Copy starting consonants and add the suffix, or add the special_case_suffix depending on the circumstances
    if first_letter_was_vowel {
        pig_latin_word.push_str(special_case_suffix);
    } else {
        pig_latin_word.push_str(&starting_consonants);
        pig_latin_word.push_str(suffix);
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

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_and_translate_way() {
        assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
            "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
        );
    }

    #[test]
    fn test_translate_and_translate_way_edgecases() {
        assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
        );
        assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
        );
        assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
        );
    }

    #[test]
    fn test_translate_yay() {
        assert_eq!(translate_yay("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate_yay("This library can translate any English text. It can even handle multiple sentences!"),
            "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!"
        );
    }

    #[test]
    fn test_translate_yay_edgecases() {
        assert_eq!(translate_yay("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!"
        );
        assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
        );
        assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
        );
    }

    #[test]
    fn test_translate_word_and_translate_word_way() {
        assert_eq!(translate_word("Hello"), "Ellohay");
        assert_eq!(translate_word("World!"), "Orldway!");

        assert_eq!(translate_word("This"), "Isthay");
        assert_eq!(translate_word("is"), "isway");
        assert_eq!(translate_word("a"), "away");
        assert_eq!(translate_word("test"), "esttay");
        assert_eq!(translate_word("of"), "ofway");
        assert_eq!(translate_word("the"), "ethay");
        assert_eq!(translate_word("function"), "unctionfay");
        assert_eq!(translate_word("translate_"), "anslatetray_");
        assert_eq!(translate_word("word."), "ordway.");

        assert_eq!(translate_word("I"), "Iway");
        assert_eq!(translate_word("Love"), "Ovelay");
        assert_eq!(translate_word("Pig"), "Igpay");
        assert_eq!(translate_word("Latin!"), "Atinlay!");

        assert_eq!(translate_word("You"), "Ouyay");//Y isn't a vowel here
        assert_eq!(translate_word("should"), "ouldshay");
        assert_eq!(translate_word("try"), "ytray");//Y is a vowel here
        assert_eq!(translate_word("yougurt,"), "ougurtyay,");//Y isn't a vowel here
        assert_eq!(translate_word("it's"), "itway's");//Contraction
        assert_eq!(translate_word("quite"), "uiteqay");//Awful to pronounce, but correct
        assert_eq!(translate_word("nice!"), "icenay!");

        assert_eq!(translate_word(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
        assert_eq!(translate_word(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
        assert_eq!(translate_word("12345678"), "12345678");//A number
        assert_eq!(translate_word("100 pizzas"), "100 izzaspay");//A number before a word
        assert_eq!(translate_word("over 9000"), "overway 9000");//A number after a word
    }

    #[test]
    fn test_translate_word_yay() {
        assert_eq!(translate_word_yay("Hello"), "Ellohay");
        assert_eq!(translate_word_yay("World!"), "Orldway!");

        assert_eq!(translate_word_yay("This"), "Isthay");
        assert_eq!(translate_word_yay("is"), "isyay");
        assert_eq!(translate_word_yay("a"), "ayay");
        assert_eq!(translate_word_yay("test"), "esttay");
        assert_eq!(translate_word_yay("of"), "ofyay");
        assert_eq!(translate_word_yay("the"), "ethay");
        assert_eq!(translate_word_yay("function"), "unctionfay");
        assert_eq!(translate_word_yay("translate_"), "anslatetray_");
        assert_eq!(translate_word_yay("word."), "ordway.");

        assert_eq!(translate_word_yay("I"), "Iyay");
        assert_eq!(translate_word_yay("Love"), "Ovelay");
        assert_eq!(translate_word_yay("Pig"), "Igpay");
        assert_eq!(translate_word_yay("Latin!"), "Atinlay!");

        assert_eq!(translate_word_yay("You"), "Ouyay");//Y isn't a vowel here
        assert_eq!(translate_word_yay("should"), "ouldshay");
        assert_eq!(translate_word_yay("try"), "ytray");//Y is a vowel here
        assert_eq!(translate_word_yay("yougurt,"), "ougurtyay,");//Y isn't a vowel here
        assert_eq!(translate_word_yay("it's"), "ityay's");//Contraction
        assert_eq!(translate_word_yay("quite"), "uiteqay");//Awful to pronounce, but correct
        assert_eq!(translate_word_yay("nice!"), "icenay!");

        assert_eq!(translate_word_yay(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
        assert_eq!(translate_word_yay(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordway|\":>?~`\\][';/.,\t\n");//Symbols around a word
        assert_eq!(translate_word_yay("12345678"), "12345678");//A number
        assert_eq!(translate_word_yay("100 pizzas"), "100 izzaspay");//A number before a word
        assert_eq!(translate_word_yay("over 9000"), "overyay 9000");//A number after a word
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
