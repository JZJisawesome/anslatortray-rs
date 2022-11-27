/* translate_words.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions to translate individual words (used by translate_strings.rs)
 *
*/

/* Imports */

use crate::helpers::{is_vowel, is_y};

/* Functions */

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
///This is a helper function used by the [`translate()`](crate::translate) family of functions, but
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
///This is a helper function used by the [`translate()`](crate::translate) family of functions, but
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
///This is a helper function used by the [`translate()`](crate::translate) family of functions, but
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

///TODO description, tests, and examples
fn translate_word_hay(english_word: &str) -> String {
    return translate_word_with_style(english_word, "ay", "hay");
}

///Translates a single word or contraction string into Ferb Latin!
///
///Can have leading and trailing punctuation or whitespace.
///It generally does a pretty good job with valid english words and contractions,
///and leaves symbols and spaces mostly unchanged.
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_word_with_style()`].
///
///This is a helper function used by the [`translate()`](crate::translate) family of functions, but
///it is publically exposed as potential users may find this useful.
///
///# Examples
///
///```
///use anslatortray::translate_word_ferb;
///
///assert_eq!(translate_word_ferb("Hello"), "Elloherb");
///assert_eq!(translate_word_ferb("World!"), "Orldwerb!");
///
///assert_eq!(translate_word_ferb("This"), "Istherb");
///assert_eq!(translate_word_ferb("is"), "isferb");
///assert_eq!(translate_word_ferb("a"), "aferb");
///assert_eq!(translate_word_ferb("test"), "estterb");
///assert_eq!(translate_word_ferb("of"), "offerb");
///assert_eq!(translate_word_ferb("the"), "etherb");
///assert_eq!(translate_word_ferb("function"), "unctionferb");
///assert_eq!(translate_word_ferb("translate_"), "anslatetrerb_");
///assert_eq!(translate_word_ferb("word."), "ordwerb.");
///
///assert_eq!(translate_word_ferb("I"), "Iferb");
///assert_eq!(translate_word_ferb("Love"), "Ovelerb");
///assert_eq!(translate_word_ferb("Pig"), "Igperb");
///assert_eq!(translate_word_ferb("Latin!"), "Atinlerb!");
///
///assert_eq!(translate_word_ferb("You"), "Ouyerb");//Y isn't a vowel here
///assert_eq!(translate_word_ferb("should"), "ouldsherb");
///assert_eq!(translate_word_ferb("try"), "ytrerb");//Y is a vowel here
///assert_eq!(translate_word_ferb("yougurt,"), "ougurtyerb,");//Y isn't a vowel here
///assert_eq!(translate_word_ferb("it's"), "itferb's");//Contraction
///assert_eq!(translate_word_ferb("quite"), "uiteqerb");//Awful to pronounce, but correct
///assert_eq!(translate_word_ferb("nice!"), "icenerb!");
///
///assert_eq!(translate_word_ferb(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word_ferb(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordwerb|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word_ferb("12345678"), "12345678");//A number
///assert_eq!(translate_word_ferb("100 pizzas"), "100 izzasperb");//A number before a word
///assert_eq!(translate_word_ferb("over 9000"), "overferb 9000");//A number after a word
///```
pub fn translate_word_ferb(english_word: &str) -> String {
    return translate_word_with_style(english_word, "erb", "ferb");
}

///Translates a single word or contraction string into a custom-styled play language!
///
///Pass the word you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///# Examples
///
///```
///use anslatortray::translate_word_with_style;
///
///let suffix = "ancy";
///let special_case_suffix = "fancy";
///
///assert_eq!(translate_word_with_style("Hello", suffix, special_case_suffix), "Ellohancy");
///assert_eq!(translate_word_with_style("World!", suffix, special_case_suffix), "Orldwancy!");
///
///assert_eq!(translate_word_with_style("This", suffix, special_case_suffix), "Isthancy");
///assert_eq!(translate_word_with_style("is", suffix, special_case_suffix), "isfancy");
///assert_eq!(translate_word_with_style("a", suffix, special_case_suffix), "afancy");
///assert_eq!(translate_word_with_style("test", suffix, special_case_suffix), "esttancy");
///assert_eq!(translate_word_with_style("of", suffix, special_case_suffix), "offancy");
///assert_eq!(translate_word_with_style("the", suffix, special_case_suffix), "ethancy");
///assert_eq!(translate_word_with_style("function", suffix, special_case_suffix), "unctionfancy");
///assert_eq!(translate_word_with_style("translate_", suffix, special_case_suffix), "anslatetrancy_");
///assert_eq!(translate_word_with_style("word.", suffix, special_case_suffix), "ordwancy.");
///
///assert_eq!(translate_word_with_style("I", suffix, special_case_suffix), "Ifancy");
///assert_eq!(translate_word_with_style("Love", suffix, special_case_suffix), "Ovelancy");
///assert_eq!(translate_word_with_style("Pig", suffix, special_case_suffix), "Igpancy");
///assert_eq!(translate_word_with_style("Latin!", suffix, special_case_suffix), "Atinlancy!");
///
///assert_eq!(translate_word_with_style("You", suffix, special_case_suffix), "Ouyancy");//Y isn't a vowel here
///assert_eq!(translate_word_with_style("should", suffix, special_case_suffix), "ouldshancy");
///assert_eq!(translate_word_with_style("try", suffix, special_case_suffix), "ytrancy");//Y is a vowel here
///assert_eq!(translate_word_with_style("yougurt,", suffix, special_case_suffix), "ougurtyancy,");//Y isn't a vowel here
///assert_eq!(translate_word_with_style("it's", suffix, special_case_suffix), "itfancy's");//Contraction
///assert_eq!(translate_word_with_style("quite", suffix, special_case_suffix), "uiteqancy");//Awful to pronounce, but correct
///assert_eq!(translate_word_with_style("nice!", suffix, special_case_suffix), "icenancy!");
///
///assert_eq!(translate_word_with_style(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n", suffix, special_case_suffix), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
///assert_eq!(translate_word_with_style(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n", suffix, special_case_suffix), " !@#$%^&*()_+={}ordwancy|\":>?~`\\][';/.,\t\n");//Symbols around a word
///assert_eq!(translate_word_with_style("12345678", suffix, special_case_suffix), "12345678");//A number
///assert_eq!(translate_word_with_style("100 pizzas", suffix, special_case_suffix), "100 izzaspancy");//A number before a word
///assert_eq!(translate_word_with_style("over 9000", suffix, special_case_suffix), "overfancy 9000");//A number after a word
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
        || if let Some(character) = iterator.peek() { !character.is_alphabetic() } else { true }//Non-alphabetic character after the first letter, or the word ends after the first letter
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

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_translate_word_ferb() {
        assert_eq!(translate_word_ferb("Hello"), "Elloherb");
        assert_eq!(translate_word_ferb("World!"), "Orldwerb!");

        assert_eq!(translate_word_ferb("This"), "Istherb");
        assert_eq!(translate_word_ferb("is"), "isferb");
        assert_eq!(translate_word_ferb("a"), "aferb");
        assert_eq!(translate_word_ferb("test"), "estterb");
        assert_eq!(translate_word_ferb("of"), "offerb");
        assert_eq!(translate_word_ferb("the"), "etherb");
        assert_eq!(translate_word_ferb("function"), "unctionferb");
        assert_eq!(translate_word_ferb("translate_"), "anslatetrerb_");
        assert_eq!(translate_word_ferb("word."), "ordwerb.");

        assert_eq!(translate_word_ferb("I"), "Iferb");
        assert_eq!(translate_word_ferb("Love"), "Ovelerb");
        assert_eq!(translate_word_ferb("Pig"), "Igperb");
        assert_eq!(translate_word_ferb("Latin!"), "Atinlerb!");

        assert_eq!(translate_word_ferb("You"), "Ouyerb");//Y isn't a vowel here
        assert_eq!(translate_word_ferb("should"), "ouldsherb");
        assert_eq!(translate_word_ferb("try"), "ytrerb");//Y is a vowel here
        assert_eq!(translate_word_ferb("yougurt,"), "ougurtyerb,");//Y isn't a vowel here
        assert_eq!(translate_word_ferb("it's"), "itferb's");//Contraction
        assert_eq!(translate_word_ferb("quite"), "uiteqerb");//Awful to pronounce, but correct
        assert_eq!(translate_word_ferb("nice!"), "icenerb!");

        assert_eq!(translate_word_ferb(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
        assert_eq!(translate_word_ferb(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n"), " !@#$%^&*()_+={}ordwerb|\":>?~`\\][';/.,\t\n");//Symbols around a word
        assert_eq!(translate_word_ferb("12345678"), "12345678");//A number
        assert_eq!(translate_word_ferb("100 pizzas"), "100 izzasperb");//A number before a word
        assert_eq!(translate_word_ferb("over 9000"), "overferb 9000");//A number after a word
    }

    #[test]
    fn test_translate_word_with_style() {
        let suffix_special_case_suffix_pairs = [("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_word_with_style("Hello", suffix, special_case_suffix), "Elloh".to_string() + suffix);
            assert_eq!(translate_word_with_style("World!", suffix, special_case_suffix), "Orldw".to_string() + suffix + "!");

            assert_eq!(translate_word_with_style("This", suffix, special_case_suffix), "Isth".to_string() + suffix);
            assert_eq!(translate_word_with_style("is", suffix, special_case_suffix), "is".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("a", suffix, special_case_suffix), "a".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("test", suffix, special_case_suffix), "estt".to_string() + suffix);
            assert_eq!(translate_word_with_style("of", suffix, special_case_suffix), "of".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("the", suffix, special_case_suffix), "eth".to_string() + suffix);
            assert_eq!(translate_word_with_style("function", suffix, special_case_suffix), "unctionf".to_string() + suffix);
            assert_eq!(translate_word_with_style("translate_", suffix, special_case_suffix), "anslatetr".to_string() + suffix + "_");
            assert_eq!(translate_word_with_style("word.", suffix, special_case_suffix), "ordw".to_string() + suffix + ".");

            assert_eq!(translate_word_with_style("I", suffix, special_case_suffix), "I".to_string() + special_case_suffix);
            assert_eq!(translate_word_with_style("Love", suffix, special_case_suffix), "Ovel".to_string() + suffix);
            assert_eq!(translate_word_with_style("Pig", suffix, special_case_suffix), "Igp".to_string() + suffix);
            assert_eq!(translate_word_with_style("Latin!", suffix, special_case_suffix), "Atinl".to_string() + suffix + "!");

            assert_eq!(translate_word_with_style("You", suffix, special_case_suffix), "Ouy".to_string() + suffix);//Y isn't a vowel here
            assert_eq!(translate_word_with_style("should", suffix, special_case_suffix), "ouldsh".to_string() + suffix);
            assert_eq!(translate_word_with_style("try", suffix, special_case_suffix), "ytr".to_string() + suffix);//Y is a vowel here
            assert_eq!(translate_word_with_style("yougurt,", suffix, special_case_suffix), "ougurty".to_string() + suffix + ",");//Y isn't a vowel here
            assert_eq!(translate_word_with_style("it's", suffix, special_case_suffix), "it".to_string() + special_case_suffix + "'s");//Contraction
            assert_eq!(translate_word_with_style("quite", suffix, special_case_suffix), "uiteq".to_string() + suffix);//Awful to pronounce, but correct
            assert_eq!(translate_word_with_style("nice!", suffix, special_case_suffix), "icen".to_string() + suffix + "!");

            assert_eq!(translate_word_with_style(" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n", suffix, special_case_suffix), " !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n");//Lots of symbols
            assert_eq!(translate_word_with_style(" !@#$%^&*()_+={}word|\":>?~`\\][';/.,\t\n", suffix, special_case_suffix), " !@#$%^&*()_+={}ordw".to_string() + suffix + "|\":>?~`\\][';/.,\t\n");//Symbols around a word
            assert_eq!(translate_word_with_style("12345678", suffix, special_case_suffix), "12345678");//A number
            assert_eq!(translate_word_with_style("100 pizzas", suffix, special_case_suffix), "100 izzasp".to_string() + suffix);//A number before a word
            assert_eq!(translate_word_with_style("over 9000", suffix, special_case_suffix), "over".to_string() + special_case_suffix + " 9000");//A number after a word
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
            return translate_word("translator");
        });
    }

    #[bench]
    fn translate_word_yay_the_word_translator(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_word_yay("translator");
        });
    }

    #[bench]
    fn translate_word_ferb_the_word_translator(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_word_ferb("translator");
        });
    }
}
