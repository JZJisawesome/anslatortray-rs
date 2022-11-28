/* translate_strings.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions for translating multiple sentences.
 *
*/

/* Imports */

use crate::translate_words::translate_word_with_style_reuse_buffers;
use crate::translate_words::translate_word_with_style_reuse_buffers_ascii;

/* Functions */

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_with_style()`].
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
///assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
///```
pub fn translate(english: &str) -> String {
    return translate_way(english);
}

///TODO description, tests, and examples
pub fn translate_ascii(english: &str) -> String {
    return translate_way_ascii(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style)!
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_with_style()`].
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
///assert_eq!(translate_way("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_way("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate_way(english: &str) -> String {
    return translate_with_style(english, "ay", "way");
}

///TODO description, tests, and examples
pub fn translate_way_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "way");
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_with_style()`].
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
///assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_yay("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_yay("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areyay-ifficultday-arenyay't-eythay?");
///```
pub fn translate_yay(english: &str) -> String {
    return translate_with_style(english, "ay", "yay");
}

///TODO description, tests, and examples
pub fn translate_yay_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "yay");
}

///TODO description, tests, and examples
pub fn translate_hay(english: &str) -> String {
    return translate_with_style(english, "ay", "hay");
}

///TODO description, tests, and examples
pub fn translate_hay_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "hay");
}

///Translates a multi-word string (including punctuation) into Ferb Latin!
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_ferb;
///
///assert_eq!(translate_ferb("Hello world from the coolest Ferb Latin translator!"), "Elloherb orldwerb omfrerb etherb oolestcerb Erbferb Atinlerb anslatortrerb!");
///
///assert_eq!(translate_ferb("This library can translate any English text. It can even handle multiple sentences!"),
///    "Istherb ibrarylerb ancerb anslatetrerb anyferb Englishferb extterb. Itferb ancerb evenferb andleherb ultiplemerb entencesserb!"
///);
///
///assert_eq!(translate_ferb("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlerb's ytrerb omeserb edgeferb asescerb. Attherb isferb aferb ontractioncerb, asferb ellwerb asferb aferb ordwerb erewherb etherb onlyferb owelverb isferb yferb. Eatnerb, allferb attherb orkswerb!"
///);
///assert_eq!(translate_ferb("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzerb"
///);
///assert_eq!(translate_ferb("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!"
///);
///
///assert_eq!(translate_ferb("Hello-world"), "Elloherb-orldwerb");
///assert_eq!(translate_ferb("Hyphens-are-difficult-aren't-they?"), "Yphensherb-areferb-ifficultderb-arenferb't-eytherb?");
///```
pub fn translate_ferb(english: &str) -> String {
    return translate_with_style(english, "erb", "ferb");
}

///TODO description, tests, and examples
pub fn translate_ferb_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "erb", "ferb");
}

///Translates a multi-word string (including punctuation) into a custom-styled play language!
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///NOTE: The suffixes must be entirely lower-case or weird results may occur.
///
///# Examples
///
///```
///use anslatortray::translate_with_style;
///
///let suffix = "ancy";
///let special_case_suffix = "fancy";
///assert_eq!(translate_with_style("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
///    "Ellohancy orldwancy omfrancy ethancy oolestcancy Igpancy Atinlancy anslatortrancy!"
///);
///
///assert_eq!(translate_with_style("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
///    "Isthancy ibrarylancy ancancy anslatetrancy anyfancy Englishfancy exttancy. Itfancy ancancy evenfancy andlehancy ultiplemancy entencessancy!"
///);
///
///assert_eq!(translate_with_style("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
///    "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
///    special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
///    " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
///    special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
///);
///
///assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
///    "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
///);
///
///assert_eq!(translate_with_style("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
///    "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
///);
///
///assert_eq!(translate_with_style("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);
///
///assert_eq!(translate_with_style("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
///    "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
///);
///```
pub fn translate_with_style(english: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    if english.is_empty() {
        return String::new();
    }

    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    eprintln!("{} -> {}, {} -> {}", suffix_lower, suffix_upper, special_case_suffix_lower, special_case_suffix_upper);

    let mut pig_latin_string = String::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    let mut current_word = String::with_capacity(64);//Longer than all English words to avoid unneeded allocations (plus leaving room for leading and trailing extra characters)
    let mut contraction_suffix = String::with_capacity(64);
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffers for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    for character in english.chars().peekable() {
        if in_word {
            if character.is_alphabetic() {
                //Save the character to translate once the word ends; we also keep apostrophes so that translate_word_with_style can handle contractions
                if in_contraction_suffix {
                    contraction_suffix.push(character);
                } else {
                    current_word.push(character);
                }
            } else if character == '\'' {
                in_contraction_suffix = true;
                contraction_suffix.push(character);
            } else {
                //The word ended, so translate the chararacters we've saved up until this point!
                in_word = false;
                translate_word_with_style_reuse_buffers (
                    current_word.as_str(),
                    suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
                    &mut pig_latin_string, &mut starting_consonants_buffer
                );

                //Push the contraction
                in_contraction_suffix = false;
                pig_latin_string.push_str(&contraction_suffix);
                contraction_suffix.truncate(0);//Faster than creating a new string

                //Append the symbol/whitespace we just got after the translated word
                pig_latin_string.push(character);
            }
        } else {//We are not currently in a word
            if character.is_alphabetic() {
                //If we see a letter, we are in a word, so save the character for now so we can translate the word later
                in_word = true;
                current_word.truncate(0);//Faster than creating a new string
                current_word.push(character);
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(character);
            }
        }
    }
    //If we ended on a word, we translate it and push it to the end of the string
    if in_word {
        translate_word_with_style_reuse_buffers (
            current_word.as_str(),
            suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return pig_latin_string;
}

///TODO description, tests, and examples
pub fn translate_with_style_ascii(english: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    if english.is_empty() {
        return String::new();
    }

    //TODO switch to fully operating on u8 slices/arrays/Vecs internally (converting from a string, then to a string at the end) in anslatortray 0.5.0

    let mut pig_latin_string = String::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long

    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    //Flags used to remember if we're currently processing a word, contraction, contraction suffix or neither
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffer for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    //Indexes for improved performance (avoid copying characters to use as the english_word argument for translate_word_with_style_reuse_buffers)
    //However, this assumes each character is one byte, so this only works with ASCII strings
    let mut slice_start_index: usize = 0;//Inclusive
    let mut slice_end_index: usize = 0;//Exclusive

    for character in english.chars() {
        if in_word {
            if in_contraction_suffix {
                if character.is_alphabetic() {
                    //We never translate the contraction suffix of a word, so just copy remaining letters as-is
                } else {
                    //The contraction ended, and so too does the word
                    //We still want to copy the non-letter to the output though
                    in_contraction_suffix = false;
                    in_word = false;
                }

                pig_latin_string.push(character);//Copy the character
                slice_start_index += 1;//Keep the slice start index up to speed for later use
            } else {
                if character.is_alphabetic() {
                    //This character is part of the word, so increment the slice_end_index to include it in the slice
                    slice_end_index += 1;
                } else {
                    //The word or first part of the contraction ended, so translate the word we've identified up until this point!
                    let word_slice: &str = &english[slice_start_index..slice_end_index];
                    translate_word_with_style_reuse_buffers_ascii (
                        word_slice,
                        suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
                        &mut pig_latin_string, &mut starting_consonants_buffer
                    );

                    //Bring the slice_start_index to the end since we've finished the word and need it ready for the next one
                    slice_start_index = slice_end_index + 1;

                    //Append the symbol/whitespace we just got after the translated word
                    pig_latin_string.push(character);

                    //If the symbol/whitespace we just got is an apostrophe, then this is a contraction suffix
                    if character == '\'' {
                        in_contraction_suffix = true;
                    } else {
                        in_word = false;//This wasn't a contraction, so we're done with the word
                    }
                }
            }
        } else {
            if character.is_alphabetic() {
                //If we see a letter, we are in a word, so set the slice_end_index to the character after the slice_start_index
                in_word = true;
                slice_end_index = slice_start_index + 1;
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(character);
                slice_start_index += 1;
            }
        }
    }
    //If we ended on a word (but not on a contraction suffix), we translate it and push it to the end of the string
    if in_word && !in_contraction_suffix {
        let word_slice: &str = &english[slice_start_index..slice_end_index];
        translate_word_with_style_reuse_buffers_ascii (
            word_slice,
            suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return pig_latin_string;
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_with_style() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
                "Elloh".to_string() + suffix + " orldw" + suffix + " omfr" + suffix + " eth" + suffix + " oolestc" + suffix + " Igp" + suffix + " Atinl" + suffix + " anslatortr" + suffix + "!"
            );

            assert_eq!(translate_with_style("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
                "Isth".to_string() + suffix + " ibraryl" + suffix + " anc" + suffix + " anslatetr" + suffix + " any" + special_case_suffix + " English" + special_case_suffix + " extt" + suffix +
                ". It" + special_case_suffix + " anc" + suffix + " even" + special_case_suffix + " andleh" + suffix + " ultiplem" + suffix + " entencess" + suffix + "!"
            );
        }
    }

    #[test]
    fn test_translate_with_style_edgecases() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
                "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
                special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
                " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
                special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
            );

            assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ" + suffix
            );

            assert_eq!(translate_with_style("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
                "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
            );

            assert_eq!(translate_with_style("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);

            assert_eq!(translate_with_style("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
                "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
            );
        }
    }

    #[test]
    fn test_translate_with_style_uppercase() {
        let suffix_special_case_suffix_lower_upper_tuples = [
            ("ay", "way", "AY", "WAY"), ("ay", "yay", "AY", "YAY"), ("ay", "hay", "AY", "HAY"), ("erb", "ferb", "ERB", "FERB"),
            ("ancy", "fancy", "ANCY", "FANCY"), ("orange", "porange", "ORANGE", "PORANGE"),
            ("anana", "banana", "ANANA", "BANANA"), ("atin", "latin", "ATIN", "LATIN"), ("ust", "rust", "UST", "RUST"),
        ];

        for pair in suffix_special_case_suffix_lower_upper_tuples {
            let suffix_lower = pair.0;
            let special_case_suffix_lower = pair.1;
            let suffix_upper = pair.2;
            let special_case_suffix_upper = pair.3;

            assert_eq!(translate_with_style("HELLO WORLD!", suffix_lower, special_case_suffix_lower),
                "ELLOH".to_string() + suffix_upper + " ORLDW" + suffix_upper + "!"
            );

            assert_eq!(translate_with_style("ISN't THIS COOL?", suffix_lower, special_case_suffix_lower),
                "ISN".to_string() + special_case_suffix_upper + "'t ISTH" + suffix_upper + " OOLC" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style("What ABOUT a MIX?", suffix_lower, special_case_suffix_lower),
                "Atwh".to_string() + suffix_lower + " ABOUT" + special_case_suffix_upper + " a" + special_case_suffix_lower + " IXM" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style("Luke, I am your father!", suffix_lower, special_case_suffix_lower),//We don't want to capitalize single-letter words
                "Ukel".to_string() + suffix_lower + ", I" + special_case_suffix_lower+ " am" + special_case_suffix_lower + " oury" + suffix_lower + " atherf" + suffix_lower + "!"
            );
        }
    }

    #[test]
    fn test_translate_ferb_uppercase() {

    }

    #[test]
    fn test_translate_with_style_ascii() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style_ascii("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
                "Elloh".to_string() + suffix + " orldw" + suffix + " omfr" + suffix + " eth" + suffix + " oolestc" + suffix + " Igp" + suffix + " Atinl" + suffix + " anslatortr" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
                "Isth".to_string() + suffix + " ibraryl" + suffix + " anc" + suffix + " anslatetr" + suffix + " any" + special_case_suffix + " English" + special_case_suffix + " extt" + suffix +
                ". It" + special_case_suffix + " anc" + suffix + " even" + special_case_suffix + " andleh" + suffix + " ultiplem" + suffix + " entencess" + suffix + "!"
            );
        }
    }

    #[test]
    fn test_translate_with_style_ascii_edgecases() {
        let suffix_special_case_suffix_pairs = [
            ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
        ];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
                "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
                special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
                " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
                special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
            );

            assert_eq!(translate_with_style_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
                "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);

            assert_eq!(translate_with_style_ascii("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
                "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
            );
        }
    }

    #[test]
    fn test_translate_with_style_ascii_uppercase() {
        let suffix_special_case_suffix_lower_upper_tuples = [
            ("ay", "way", "AY", "WAY"), ("ay", "yay", "AY", "YAY"), ("ay", "hay", "AY", "HAY"), ("erb", "ferb", "ERB", "FERB"),
            ("ancy", "fancy", "ANCY", "FANCY"), ("orange", "porange", "ORANGE", "PORANGE"),
            ("anana", "banana", "ANANA", "BANANA"), ("atin", "latin", "ATIN", "LATIN"), ("ust", "rust", "UST", "RUST"),
        ];

        for pair in suffix_special_case_suffix_lower_upper_tuples {
            let suffix_lower = pair.0;
            let special_case_suffix_lower = pair.1;
            let suffix_upper = pair.2;
            let special_case_suffix_upper = pair.3;

            assert_eq!(translate_with_style_ascii("HELLO WORLD!", suffix_lower, special_case_suffix_lower),
                "ELLOH".to_string() + suffix_upper + " ORLDW" + suffix_upper + "!"
            );

            assert_eq!(translate_with_style_ascii("ISN't THIS COOL?", suffix_lower, special_case_suffix_lower),
                "ISN".to_string() + special_case_suffix_upper + "'t ISTH" + suffix_upper + " OOLC" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style_ascii("What ABOUT a MIX?", suffix_lower, special_case_suffix_lower),
                "Atwh".to_string() + suffix_lower + " ABOUT" + special_case_suffix_upper + " a" + special_case_suffix_lower + " IXM" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style_ascii("Luke, I am your father!", suffix_lower, special_case_suffix_lower),//We don't want to capitalize single-letter words
                "Ukel".to_string() + suffix_lower + ", I" + special_case_suffix_lower+ " am" + special_case_suffix_lower + " oury" + suffix_lower + " atherf" + suffix_lower + "!"
            );
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
    fn translate_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_yay_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_yay("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_ferb_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_ferb("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }

    #[bench]
    fn translate_yay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_yay("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }

    #[bench]
    fn translate_ferb_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_ferb("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }
}
