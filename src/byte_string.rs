/* byte_string.rs
 * By: John Jekel
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Translation functions operating on &[u8] and Vec::<u8> (higher efficiency, but less user-friendly)
 *
*/

//!anslatortray Functions Operating on Byte-Strings
//!
//!As opposed to functions provided in the anslatortray crate's root, which operate on [`&str`] and [`String`], these functions operate on `&[u8]` and [`Vec<u8>`].
//!
//!In performance-sensitive applications, they can allow for some minor optimizations:
//!* One can reuse buffers for getting the result of a translation (as the functions accept a mutable reference to a [`Vec<u8>`] rather than returning data)
//!* One can avoid the penalty of converting to an [`&str`], translating to a [`String`], and having to convert back to raw bytes if one is working solely with byte-strings.
//!
//!Note that both ASCII and UTF-8 byte strings may be passed to these functions, and that valid ASCII/UTF-8 will be returned.
//!In the past there were "ASCII-only" functions that operated on [`String`]s, but these were removed.
//!These byte_string functions are NOT the sucessors of those functions.
//!Without exception, ALL functions in anslatortray accept both ASCII and UTF-8 text, regardless of whether they operate on byte-strings or char-strings.
//!The modern functions present are faster than the old ASCII ones anyways, even the ones in the crate's root that don't operate on byte-strings.

/* Imports */

use std::num::Wrapping;

/* Functions */

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_with_style()`].
///
///Equivalent to [`translate_way()`].
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate;
///
///let mut buffer = Vec::<u8>::new();
///
///translate(b"Hello world from the coolest Pig Latin translator!", &mut buffer);
///assert_eq!(&buffer, b"Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///buffer.truncate(0);
///translate(b"This library can translate any English text. It can even handle multiple sentences!", &mut buffer);
///assert_eq!(&buffer, b"Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!");
///
///buffer.truncate(0);
///translate(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", &mut buffer);
///assert_eq!(&buffer, b"Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!");
///
///buffer.truncate(0);
///translate(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", &mut buffer);
///assert_eq!(&buffer, b"Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay");
///
///buffer.truncate(0);
///translate(b"Cool, so the heuristics make pretty good guesses with what they're fed!", &mut buffer);
///assert_eq!(&buffer, b"Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!");
///
///buffer.truncate(0);
///translate(b"Hello-world", &mut buffer);
///assert_eq!(&buffer, b"Ellohay-orldway");
///
///buffer.truncate(0);
///translate(b"Hyphens-are-difficult-aren't-they?", &mut buffer);
///assert_eq!(&buffer, b"Yphenshay-areway-ifficultday-arenway't-eythay?");
///
///buffer.truncate(0);
///translate(b"The buffer isn't cleared by the translate function beforehand, ", &mut buffer);
///translate(b"so we can do something like this if we wish!", &mut buffer);
///assert_eq!(&buffer, b"Ethay ufferbay isnway't earedclay ybay ethay anslatetray unctionfay eforehandbay, osay eway ancay oday omethingsay ikelay isthay ifway eway ishway!");
///```
pub fn translate(english: &[u8], pig_latin_string: &mut Vec::<u8>) {
    translate_way(english, pig_latin_string);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style)!
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_with_style()`].
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate_way;
///
///let mut buffer = Vec::<u8>::new();
///
///translate_way(b"Hello world from the coolest Pig Latin translator!", &mut buffer);
///assert_eq!(&buffer, b"Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///buffer.truncate(0);
///translate_way(b"This library can translate any English text. It can even handle multiple sentences!", &mut buffer);
///assert_eq!(&buffer, b"Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!");
///
///buffer.truncate(0);
///translate_way(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", &mut buffer);
///assert_eq!(&buffer, b"Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!");
///
///buffer.truncate(0);
///translate_way(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", &mut buffer);
///assert_eq!(&buffer, b"Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay");
///
///buffer.truncate(0);
///translate_way(b"Cool, so the heuristics make pretty good guesses with what they're fed!", &mut buffer);
///assert_eq!(&buffer, b"Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!");
///
///buffer.truncate(0);
///translate_way(b"Hello-world", &mut buffer);
///assert_eq!(&buffer, b"Ellohay-orldway");
///
///buffer.truncate(0);
///translate_way(b"Hyphens-are-difficult-aren't-they?", &mut buffer);
///assert_eq!(&buffer, b"Yphenshay-areway-ifficultday-arenway't-eythay?");
///
///buffer.truncate(0);
///translate_way(b"The buffer isn't cleared by the translate function beforehand, ", &mut buffer);
///translate_way(b"so we can do something like this if we wish!", &mut buffer);
///assert_eq!(&buffer, b"Ethay ufferbay isnway't earedclay ybay ethay anslatetray unctionfay eforehandbay, osay eway ancay oday omethingsay ikelay isthay ifway eway ishway!");
///```
pub fn translate_way(english: &[u8], pig_latin_string: &mut Vec::<u8>) {
    translate_with_style_lower_and_upper_suffixes(english, b"ay", b"way", b"AY", b"WAY", pig_latin_string);
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_with_style()`].
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate_yay;
///
///let mut buffer = Vec::<u8>::new();
///
///translate_yay(b"Hello world from the coolest Pig Latin translator!", &mut buffer);
///assert_eq!(&buffer, b"Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///buffer.truncate(0);
///translate_yay(b"This library can translate any English text. It can even handle multiple sentences!", &mut buffer);
///assert_eq!(&buffer, b"Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!");
///
///buffer.truncate(0);
///translate_yay(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", &mut buffer);
///assert_eq!(&buffer, b"Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!");
///
///buffer.truncate(0);
///translate_yay(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", &mut buffer);
///assert_eq!(&buffer, b"Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay");
///
///buffer.truncate(0);
///translate_yay(b"Cool, so the heuristics make pretty good guesses with what they're fed!", &mut buffer);
///assert_eq!(&buffer, b"Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!");
///
///buffer.truncate(0);
///translate_yay(b"Hello-world", &mut buffer);
///assert_eq!(&buffer, b"Ellohay-orldway");
///
///buffer.truncate(0);
///translate_yay(b"Hyphens-are-difficult-aren't-they?", &mut buffer);
///assert_eq!(&buffer, b"Yphenshay-areyay-ifficultday-arenyay't-eythay?");
///
///buffer.truncate(0);
///translate_yay(b"The buffer isn't cleared by the translate function beforehand, ", &mut buffer);
///translate_yay(b"so we can do something like this if we wish!", &mut buffer);
///assert_eq!(&buffer, b"Ethay ufferbay isnyay't earedclay ybay ethay anslatetray unctionfay eforehandbay, osay eway ancay oday omethingsay ikelay isthay ifyay eway ishway!");
///```
pub fn translate_yay(english: &[u8], pig_latin_string: &mut Vec::<u8>) {
    translate_with_style_lower_and_upper_suffixes(english, b"ay", b"yay", b"AY", b"WAY", pig_latin_string);
}

///Translates a multi-word string (including punctuation) into Pig Latin (hay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "hay" respectively when calling [`translate_with_style()`].
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate_hay;
///
///let mut buffer = Vec::<u8>::new();
///
///translate_hay(b"Hello world from the coolest Pig Latin translator!", &mut buffer);
///assert_eq!(&buffer, b"Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///buffer.truncate(0);
///translate_hay(b"This library can translate any English text. It can even handle multiple sentences!", &mut buffer);
///assert_eq!(&buffer, b"Isthay ibrarylay ancay anslatetray anyhay Englishhay exttay. Ithay ancay evenhay andlehay ultiplemay entencessay!");
///
///buffer.truncate(0);
///translate_hay(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", &mut buffer);
///assert_eq!(&buffer, b"Etlay's ytray omesay edgehay asescay. Atthay ishay ahay ontractioncay, ashay ellway ashay ahay ordway erewhay ethay onlyhay owelvay ishay yhay. Eatnay, allhay atthay orksway!");
///
///buffer.truncate(0);
///translate_hay(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", &mut buffer);
///assert_eq!(&buffer, b"Atwhay ifhay ahay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay");
///
///buffer.truncate(0);
///translate_hay(b"Cool, so the heuristics make pretty good guesses with what they're fed!", &mut buffer);
///assert_eq!(&buffer, b"Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!");
///
///buffer.truncate(0);
///translate_hay(b"Hello-world", &mut buffer);
///assert_eq!(&buffer, b"Ellohay-orldway");
///
///buffer.truncate(0);
///translate_hay(b"Hyphens-are-difficult-aren't-they?", &mut buffer);
///assert_eq!(&buffer, b"Yphenshay-arehay-ifficultday-arenhay't-eythay?");
///
///buffer.truncate(0);
///translate_hay(b"The buffer isn't cleared by the translate function beforehand, ", &mut buffer);
///translate_hay(b"so we can do something like this if we wish!", &mut buffer);
///assert_eq!(&buffer, b"Ethay ufferbay isnhay't earedclay ybay ethay anslatetray unctionfay eforehandbay, osay eway ancay oday omethingsay ikelay isthay ifhay eway ishway!");
///```
pub fn translate_hay(english: &[u8], pig_latin_string: &mut Vec::<u8>) {
    translate_with_style_lower_and_upper_suffixes(english, b"ay", b"hay", b"AY", b"HAY", pig_latin_string);
}

///Translates a multi-word string (including punctuation) into Ferb Latin!
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_with_style()`].
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate_ferb;
///
///let mut buffer = Vec::<u8>::new();
///
///translate_ferb(b"Hello world from the coolest Pig Latin translator!", &mut buffer);
///assert_eq!(&buffer, b"Elloherb orldwerb omfrerb etherb oolestcerb Igperb Atinlerb anslatortrerb!");
///
///buffer.truncate(0);
///translate_ferb(b"This library can translate any English text. It can even handle multiple sentences!", &mut buffer);
///assert_eq!(&buffer, b"Istherb ibrarylerb ancerb anslatetrerb anyferb Englishferb extterb. Itferb ancerb evenferb andleherb ultiplemerb entencesserb!");
///
///buffer.truncate(0);
///translate_ferb(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", &mut buffer);
///assert_eq!(&buffer, b"Etlerb's ytrerb omeserb edgeferb asescerb. Attherb isferb aferb ontractioncerb, asferb ellwerb asferb aferb ordwerb erewherb etherb onlyferb owelverb isferb yferb. Eatnerb, allferb attherb orkswerb!");
///
///buffer.truncate(0);
///translate_ferb(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", &mut buffer);
///assert_eq!(&buffer, b"Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzerb");
///
///buffer.truncate(0);
///translate_ferb(b"Cool, so the heuristics make pretty good guesses with what they're fed!", &mut buffer);
///assert_eq!(&buffer, b"Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!");
///
///buffer.truncate(0);
///translate_ferb(b"Hello-world", &mut buffer);
///assert_eq!(&buffer, b"Elloherb-orldwerb");
///
///buffer.truncate(0);
///translate_ferb(b"Hyphens-are-difficult-aren't-they?", &mut buffer);
///assert_eq!(&buffer, b"Yphensherb-areferb-ifficultderb-arenferb't-eytherb?");
///
///buffer.truncate(0);
///translate_ferb(b"The buffer isn't cleared by the translate function beforehand, ", &mut buffer);
///translate_ferb(b"so we can do something like this if we wish!", &mut buffer);
///assert_eq!(&buffer, b"Etherb ufferberb isnferb't earedclerb yberb etherb anslatetrerb unctionferb eforehandberb, oserb ewerb ancerb oderb omethingserb ikelerb istherb ifferb ewerb ishwerb!");
///```
pub fn translate_ferb(english: &[u8], pig_latin_string: &mut Vec::<u8>) {
    translate_with_style_lower_and_upper_suffixes(english, b"erb", b"ferb", b"ERB", b"FERB", pig_latin_string);
}

///Translates a multi-word string (including punctuation) into a custom-styled play language!
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///Note: The suffixes must be entirely lower-case or weird results may occur.
///
///Note: The resulting translation is appended to the provided buffer, so one may wish to ensure it is cleared before each use or not depending on the application.
///
///# Examples
///
///```
///use anslatortray::byte_string::translate_with_style;
///
///let suffix = b"ancy";
///let special_case_suffix = b"fancy";
///
///let mut buffer = Vec::<u8>::new();
///
///translate_with_style(b"Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Ellohancy orldwancy omfrancy ethancy oolestcancy Igpancy Atinlancy anslatortrancy!");
///
///buffer.truncate(0);
///translate_with_style(b"This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Isthancy ibrarylancy ancancy anslatetrancy anyfancy Englishfancy exttancy. Itfancy ancancy evenfancy andlehancy ultiplemancy entencessancy!");
///
///buffer.truncate(0);
///translate_with_style(b"Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Etlancy's ytrancy omesancy edgefancy asescancy. Atthancy isfancy afancy ontractioncancy, asfancy ellwancy asfancy afancy ordwancy erewhancy ethancy onlyfancy owelvancy isfancy yfancy. Eatnancy, allfancy atthancy orkswancy!");
///
///buffer.truncate(0);
///translate_with_style(b"What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Atwhancy iffancy afancy ordwancy ashancy onancy owelsvancy, ikelancy isthancy: bcdfghjklmnpqrstvwxzancy");
///
///buffer.truncate(0);
///translate_with_style(b"Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Oolcancy, osancy ethancy euristicshancy akemancy ettyprancy oodgancy uessesgancy ithwancy atwhancy eythancy're edfancy!");
///
///buffer.truncate(0);
///translate_with_style(b"Hello-world", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Ellohancy-orldwancy");
///
///buffer.truncate(0);
///translate_with_style(b"Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Yphenshancy-arefancy-ifficultdancy-arenfancy't-eythancy?");
///
///buffer.truncate(0);
///translate_with_style(b"The buffer isn't cleared by the translate function beforehand, ", suffix, special_case_suffix, &mut buffer);
///translate_with_style(b"so we can do something like this if we wish!", suffix, special_case_suffix, &mut buffer);
///assert_eq!(&buffer, b"Ethancy ufferbancy isnfancy't earedclancy ybancy ethancy anslatetrancy unctionfancy eforehandbancy, osancy ewancy ancancy odancy omethingsancy ikelancy isthancy iffancy ewancy ishwancy!");
///```
pub fn translate_with_style(english: &[u8], suffix_lower: &[u8], special_case_suffix_lower: &[u8], pig_latin_string: &mut Vec::<u8>) {
    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = Vec::<u8>::with_capacity(suffix_lower.len());
    for letter in suffix_lower.iter() {
        suffix_upper.push(letter.to_ascii_uppercase());//NOTE: We can't use fast_to_ascii_uppercase in case the suffixes contain UTF-8 or non-letters
    }
    let mut special_case_suffix_upper = Vec::<u8>::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.iter() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());//NOTE: We can't use fast_to_ascii_uppercase in case the suffixes contain UTF-8 or non-letters
    }

    translate_with_style_lower_and_upper_suffixes(english, suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper, pig_latin_string);
}

//Avoids the overhead of having to convert suffixes to uppercase for the standard translation functions at runtime
pub(crate) fn translate_with_style_lower_and_upper_suffixes (
    english: &[u8],
    suffix_lower: &[u8], special_case_suffix_lower: &[u8], suffix_upper: &[u8], special_case_suffix_upper: &[u8],
    pig_latin_string: &mut Vec::<u8>
) {
    if english.is_empty() {
        return;
    }

    let mut global_index: usize = 0;
    loop {
        //Copies characters in-between words
        //TODO this could probably be optimized with vector instructions
        {
            let mut start_of_in_between_words_index: usize = global_index;//Inclusive
            loop {
                if english[global_index].is_ascii_alphabetic() {//Start of a word
                    break;
                }

                global_index += 1;
                if global_index == english.len() {
                    //Copy all of the characters so far (all that remain) and return
                    let remaining_characters_slice = &english[start_of_in_between_words_index..];
                    pig_latin_string.extend_from_slice(remaining_characters_slice);
                    return;
                }
            }
            //Copy the characters in-between words as-is
            let in_between_words_characters_slice = &english[start_of_in_between_words_index..global_index];
            pig_latin_string.extend_from_slice(in_between_words_characters_slice);

            //At this point, global_index contains the index to the start of the word to translate
        }

        //Translates the current word
        {
            let word_start_index = global_index;
            let first_letter = english[word_start_index];
            global_index += 1;

            if (global_index == english.len()) || (!english[global_index].is_ascii_alphabetic()) {//The word is only one letter long (special case)
                //Push the letter and add the lowercase special suffix (even if the letter is uppercase)
                pig_latin_string.push(first_letter);
                pig_latin_string.extend_from_slice(special_case_suffix_lower);
            } else if is_vowel(first_letter) {//The word is longer than a letter and starts with a vowel (special case)
                //As a heuristic, we consider Y to be a vowel when it is not at the start of the word

                //Get the slice containing the whole word
                let slice_to_search_for_end = &english[global_index..];
                let word_slice: &[u8];
                if let Some(found_end_of_word_index) = slice_to_search_for_end.iter().position(|&x| !x.is_ascii_alphabetic()) {//We found a non-letter that ends the word
                    global_index += found_end_of_word_index;
                    word_slice = &english[word_start_index..global_index];
                } else {//The string ended
                    global_index = english.len();
                    word_slice = slice_to_search_for_end;
                }

                //Translate the word and push it
                pig_latin_string.extend_from_slice(word_slice);
                if fast_is_ascii_uppercase(english[word_start_index + 1]) {//As a heuristic, we consider the word to be uppercase if the second letter is
                    pig_latin_string.extend_from_slice(special_case_suffix_upper);
                } else {//Word is entirely lowercase, or its first letter is uppercase only
                    pig_latin_string.extend_from_slice(special_case_suffix_lower);
                }
            } else {//The word is longer than a letter and doesn't start with a vowel
                //Find the first vowel; we assume the word actually has a vowel in it
                let first_vowel_index: usize;
                let slice_to_search_for_vowel = &english[global_index..];
                if let Some(first_vowel_of_word_index) = slice_to_search_for_vowel.iter().position(|&x| { is_vowel(x) || is_y(x) }) {//As a heuristic, we consider Y to be a vowel when it is not at the start of the word
                    global_index += first_vowel_of_word_index;
                } else {//This string ended and we never found a vowel
                    return;//Just give up
                }
                first_vowel_index = global_index;

                //Find the end of the word
                let word_end_index: usize;
                let slice_to_search_for_end = &english[global_index..];
                if let Some(end_of_word_index) = slice_to_search_for_end.iter().position(|&x| !x.is_ascii_alphabetic()) {//We found a non-letter that ends the word
                    global_index += end_of_word_index;
                } else {//The string ended
                    global_index = english.len();
                }
                word_end_index = global_index;

                //Translate the word
                //TODO improve code reuse here
                if fast_is_ascii_uppercase(first_letter) {//Check if the first letter is uppercase
                    if fast_is_ascii_uppercase(english[word_start_index + 1]) {//As a heuristic, we consider the word to be uppercase if the second letter is
                        //Push the vowel and all letters after it
                        let vowel_to_end_slice = &english[first_vowel_index..word_end_index];
                        pig_latin_string.extend_from_slice(vowel_to_end_slice);

                        //Push the starting consonants
                        let start_to_vowel_slice = &english[word_start_index..first_vowel_index];
                        pig_latin_string.extend_from_slice(start_to_vowel_slice);

                        //Push the normal suffix (uppercase)
                        pig_latin_string.extend_from_slice(suffix_upper);
                    } else {//Word starts with an uppercase letter, but is otherwise lowercase
                        //Push the vowel, matching the starting case of the original word
                        pig_latin_string.push(fast_to_ascii_uppercase(english[first_vowel_index]));

                        //Push all letters after the vowel
                        let after_vowel_slice = &english[(first_vowel_index + 1)..word_end_index];
                        pig_latin_string.extend_from_slice(after_vowel_slice);

                        //Push the first starting consonant, which should be lowercase now
                        pig_latin_string.push(fast_to_ascii_lowercase(english[word_start_index]));

                        //Push the remaining starting consonants
                        let after_start_to_vowel_slice = &english[(word_start_index + 1)..first_vowel_index];
                        pig_latin_string.extend_from_slice(after_start_to_vowel_slice);

                        //Push the normal suffix
                        pig_latin_string.extend_from_slice(suffix_lower);
                    }
                } else {//Word is entirely lowercase
                    //Push the vowel and all letters after it
                    let vowel_to_end_slice = &english[first_vowel_index..word_end_index];
                    pig_latin_string.extend_from_slice(vowel_to_end_slice);

                    //Push the starting consonants
                    let start_to_vowel_slice = &english[word_start_index..first_vowel_index];
                    pig_latin_string.extend_from_slice(start_to_vowel_slice);

                    //Push the normal suffix (lowercase)
                    pig_latin_string.extend_from_slice(suffix_lower);
                }
            }

            //Don't go on if we reached the end of the string during the word
            if global_index == english.len() {
                return;
            }

            //At this point, global_index contains the index to the next character to check
        }

        //Copies contraction suffixes, if present
        if english[global_index] == b'\'' {//TODO if this is true we can also skip the regular inter-word loop on the next iteration
            let mut start_of_contraction_suffix_index: usize = global_index;//Inclusive
            global_index += 1;//We skip over the apostrophe for the loop below, but we still want to copy it in the end
            loop {
                if global_index == english.len() {
                    //Copy all of the characters so far (all that remain) and return
                    let remaining_characters_slice = &english[start_of_contraction_suffix_index..];
                    pig_latin_string.extend_from_slice(remaining_characters_slice);
                    return;
                }

                if !english[global_index].is_ascii_alphabetic() {//End of the contraction suffix
                    break;
                }

                global_index += 1;
            }
            //Copy the contraction suffix as-is
            let contraction_suffix_slice = &english[start_of_contraction_suffix_index..global_index];
            pig_latin_string.extend_from_slice(contraction_suffix_slice);
        }
    }
}

//Avoids the overhead of having to convert suffixes to uppercase for the standard translation functions at runtime
pub(crate) fn translate_with_style_lower_and_upper_suffixes_old (
    english: &[u8],
    suffix_lower: &[u8], special_case_suffix_lower: &[u8], suffix_upper: &[u8], special_case_suffix_upper: &[u8],
    pig_latin_string: &mut Vec::<u8>
) {
    if english.is_empty() {
        return;
    }

    //TODO merge the word and the generic text function into one function to allow for optimizations with certain things
    //TODO do an SSE/AVX optimized version of this

    //Flags used to remember if we're currently processing a word, contraction, contraction suffix or neither
    //TODO can we avoid needing these flags and be more efficient?
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

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
                        suffix_lower, special_case_suffix_lower, suffix_upper, special_case_suffix_upper,
                        pig_latin_string
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
            suffix_lower, special_case_suffix_lower, suffix_upper, special_case_suffix_upper,
            pig_latin_string
        );
    }
}

//Translate a word (english_word MUST ONLY CONTAIN ASCII LETTERS, not numbers/symbols/etc or anything UTF-8)
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn translate_word_with_style_reuse_buffers (
    english_word: &[u8],//Assumes this word is not empty
    suffix_lower: &[u8], special_case_suffix_lower: &[u8], suffix_upper: &[u8], special_case_suffix_upper: &[u8],
    buffer_to_append_to: &mut Vec<u8>
) {
    //Assume the word is at least 1 letter
    debug_assert!(english_word.len() != 0);
    if english_word.len() == 0 {
        unsafe {
            std::hint::unreachable_unchecked();
        }
    }

    //Special case for 1-letter words
    if english_word.len() == 1 {//TODO annotate this branch as unlikely taken
        //TODO it may be better to chain these back to back in a single call so the vector gets a hint with how much it needs to resize for both at once
        //See https://stackoverflow.com/questions/71785682/calling-extend-from-slice-multiple-times
        buffer_to_append_to.extend_from_slice(english_word);
        buffer_to_append_to.extend_from_slice(special_case_suffix_lower);
        return;
    }

    //Check if the word is uppercase
    let word_uppercase = word_is_uppercase(english_word);

    //As a herustic, we consider Y to be a vowel when it is not at the start of the word
    if is_vowel(english_word[0]) {//Not including y//TODO annotate this branch as unlikely taken
        buffer_to_append_to.extend_from_slice(english_word);
        if word_uppercase {
            buffer_to_append_to.extend_from_slice(special_case_suffix_upper);
        } else {
            buffer_to_append_to.extend_from_slice(special_case_suffix_lower);
        }
        return;
    }

    //Find the index of the first vowel, skipping index 1 since that was handled above
    let mut index_of_first_vowel: usize = 1;
    while index_of_first_vowel < english_word.len() {
        let character: u8 = english_word[index_of_first_vowel];
        if is_vowel(character) || is_y(character) {//As a herustic, we consider Y to be a vowel when it is not at the start of the word
            break;
        }
        index_of_first_vowel += 1;
    }

    //Now that we know where the first vowel is and if the word is uppercase, we can construct the pig-latin word
    if index_of_first_vowel < english_word.len() {//We found a vowel//TODO mark this branch as likely taken
        //Push the first vowel to the new pig latin string. If the first letter was capitalized originally, match the case
        if fast_is_ascii_uppercase(english_word[0]) {
            buffer_to_append_to.push(fast_to_ascii_uppercase(english_word[index_of_first_vowel]));
        } else {
            buffer_to_append_to.push(english_word[index_of_first_vowel]);
        }

        //Copy the remaining letters in the word after the vowel
        buffer_to_append_to.extend_from_slice(&english_word[(index_of_first_vowel + 1)..]);

        //If the first letter (a consonant) was uppercase, it no longer needs to be (since the vowel above is now at the start and capitalized)
        //Unless, of course, the whole word is uppercase, in which case it should be left alone
        buffer_to_append_to.push(if word_uppercase { english_word[0] } else { fast_to_ascii_lowercase(english_word[0]) });

        //Copy the remaining starting consonants
        buffer_to_append_to.extend_from_slice(&english_word[1..index_of_first_vowel]);
    } else {//This word dosn't have a vowel
        //Just copy it as-is then
        buffer_to_append_to.extend_from_slice(english_word);
    }

    //Add the regular suffixes
    if word_uppercase {//TODO annotate this branch as unlikely taken
        buffer_to_append_to.extend_from_slice(suffix_upper);
    } else {
        buffer_to_append_to.extend_from_slice(suffix_lower);
    }
}

//Returns whether a letter is a vowel or not.
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn is_vowel(letter: u8) -> bool {
    match letter {
        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => { return true; }
        _ => { return false; }
    }
}

//Returns whether a letter is y or not.
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn is_y(letter: u8) -> bool {
    return (letter == b'y') || (letter == b'Y');
}

//Returns whether an entire word is upper case or not.
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn word_is_uppercase(english_word: &[u8]) -> bool {
    //Asume length is non-zero
    debug_assert!(english_word.len() != 0);
    if english_word.len() == 0 {
        unsafe {
            std::hint::unreachable_unchecked();
        }
    }

    //Heuristic: If the last letter of the word is uppercase, likely the whole word is uppercase
    return fast_is_ascii_uppercase(english_word[english_word.len() - 1]);
}

//NOTE the result is undefined if the character is not a letter
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn fast_is_ascii_uppercase(letter: u8) -> bool {
    return letter <= b'Z';
}

//NOTE the result is undefined if the character is not a letter
/*#[inline(always)]//Only used by the one function in this module, so this makes sense
fn fast_is_ascii_lowercase(letter: u8) -> bool {
    return letter >= b'a';
}*/

//NOTE if the character is not an ascii letter, this may produce invalid UTF-8
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn fast_to_ascii_uppercase(letter: u8) -> u8 {
    return if letter >= b'a' { (Wrapping(letter) - Wrapping(0x20)).0 } else { letter };
}

//NOTE if the character is not an ascii letter, this may produce invalid UTF-8
#[inline(always)]//Only used by the one function in this module, so this makes sense
fn fast_to_ascii_lowercase(letter: u8) -> u8 {
    return if letter <= b'Z' { (Wrapping(letter) + Wrapping(0x20)).0 } else { letter };
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    //NOTE: We don't test byte_string::translate_with_style and other similar functions in here directly since we test them through string.rs

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

    fn translate_word_with_style(english_word: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
        let mut suffix_upper = String::new();
        for letter in suffix_lower.chars() {
            suffix_upper.push(letter.to_ascii_uppercase());
        }
        let mut special_case_suffix_upper = String::new();
        for letter in special_case_suffix_lower.chars() {
            special_case_suffix_upper.push(letter.to_ascii_uppercase());
        }

        let mut pig_latin_word = Vec::<u8>::new();
        translate_word_with_style_reuse_buffers (
            english_word.as_bytes(),
            suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &suffix_upper.as_bytes(), &special_case_suffix_upper.as_bytes(),
            &mut pig_latin_word
        );
        return std::str::from_utf8(pig_latin_word.as_slice()).unwrap().to_string();
    }

    #[test]
    fn test_is_vowel() {
        for letter in b"aeiouAEIOU".iter() {
            assert!(is_vowel(*letter));
        }

        for letter in b"bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".iter() {
            assert!(!is_vowel(*letter));
        }

        for not_letter in b" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".iter() {
            assert!(!is_vowel(*not_letter));
        }
    }

    #[test]
    fn test_is_y() {
        for letter in b"yY".iter() {
            assert!(is_y(*letter));
        }

        for letter in b"abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ".iter() {
            assert!(!is_y(*letter));
        }

        for not_letter in b" !@#$%^&*()_+={}|\":>?~`\\][';/.,\t\n".iter() {
            assert!(!is_y(*not_letter));
        }
    }

    #[test]
    fn test_word_is_uppercase() {
        assert!(word_is_uppercase(b"HELLO"));
        assert!(word_is_uppercase(b"WORLD"));

        assert!(word_is_uppercase(b"I"));
        assert!(!word_is_uppercase(b"would"));
        assert!(!word_is_uppercase(b"like"));
        assert!(!word_is_uppercase(b"a"));
        assert!(!word_is_uppercase(b"pizza"));

        assert!(!word_is_uppercase(b"Sussus"));
        assert!(!word_is_uppercase(b"Amogus"));
    }

    #[test]
    fn test_fast_is_ascii_uppercase() {
        for letter in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".iter() {
            assert_eq!(fast_is_ascii_uppercase(*letter), letter.is_ascii_uppercase());
        }
    }

    /*#[test]
    fn test_fast_is_ascii_lowercase() {
        for letter in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".iter() {
            assert_eq!(fast_is_ascii_lowercase(*letter), letter.is_ascii_lowercase());
        }
    }*/

    #[test]
    fn test_fast_to_ascii_uppercase() {
        for letter in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".iter() {
            assert_eq!(fast_to_ascii_uppercase(*letter), letter.to_ascii_uppercase());
        }
    }

    #[test]
    fn test_fast_to_ascii_lowercase() {
        for letter in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".iter() {
            assert_eq!(fast_to_ascii_lowercase(*letter), letter.to_ascii_lowercase());
        }
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    const PROJECT_DESCRIPTION: &[u8] = b"A simple Rust library to translate from English to Pig Latin!";
    const LOREM_IPSUM: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[bench]
    fn way_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"way", b"AY", b"WAY",
                &mut pig_latin_word
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn yay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"yay", b"AY", b"YAY",
                &mut pig_latin_word
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn hay_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"ay", b"hay", b"AY", b"HAY",
                &mut pig_latin_word
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn ferb_the_word_translator(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(64 * 2);//Longer than all English words to avoid unneeded allocations, times 2 to leave room for whitespace, symbols, and the suffix

        b.iter(|| {
            let word = test::black_box(b"translator");

            translate_word_with_style_reuse_buffers (
                word,
                b"erb", b"ferb", b"ERB", b"FERB",
                &mut pig_latin_word
            );

            pig_latin_word.truncate(0);
        });

        eprintln!("{}", std::str::from_utf8(pig_latin_word.as_slice()).unwrap());//To avoid optimizing things out
    }

    #[bench]
    fn way_project_description(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(PROJECT_DESCRIPTION.len() * 2);

        b.iter(|| {
            let word = test::black_box(PROJECT_DESCRIPTION);

            translate_way(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn yay_project_description(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(PROJECT_DESCRIPTION.len() * 2);

        b.iter(|| {
            let word = test::black_box(PROJECT_DESCRIPTION);

            translate_yay(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn hay_project_description(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(PROJECT_DESCRIPTION.len() * 2);

        b.iter(|| {
            let word = test::black_box(PROJECT_DESCRIPTION);

            translate_hay(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn ferb_project_description(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(PROJECT_DESCRIPTION.len() * 2);

        b.iter(|| {
            let word = test::black_box(PROJECT_DESCRIPTION);

            translate_ferb(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn way_lorem_ipsum(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(LOREM_IPSUM.len() * 2);

        b.iter(|| {
            let word = test::black_box(LOREM_IPSUM);

            translate_way(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn yay_lorem_ipsum(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(LOREM_IPSUM.len() * 2);

        b.iter(|| {
            let word = test::black_box(LOREM_IPSUM);

            translate_yay(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn hay_lorem_ipsum(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(LOREM_IPSUM.len() * 2);

        b.iter(|| {
            let word = test::black_box(LOREM_IPSUM);

            translate_hay(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }

    #[bench]
    fn ferb_lorem_ipsum(b: &mut Bencher) {
        let mut pig_latin_word = Vec::<u8>::with_capacity(LOREM_IPSUM.len() * 2);

        b.iter(|| {
            let word = test::black_box(LOREM_IPSUM);

            translate_ferb(word, &mut pig_latin_word);

            pig_latin_word.truncate(0);
        });
    }
}
