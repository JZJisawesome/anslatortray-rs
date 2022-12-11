/* string.rs
 * By: John Jekel
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Translation functions operating on &str and String (the ones most users will want to use)
 *
*/

/* Imports */

use crate::byte_string::translate_with_style as translate_byte_string_with_style;

/* Functions */

//TODO use byte_string::translate_with_style_lower_and_upper_suffixes for speed

pub fn translate(english: &str) -> String {
    return translate_way(english);
}

pub fn translate_way(english: &str) -> String {
    return translate_with_style(english, "ay", "way");
}

pub fn translate_yay(english: &str) -> String {
    return translate_with_style(english, "ay", "yay");
}

pub fn translate_hay(english: &str) -> String {
    return translate_with_style(english, "ay", "hay");
}

pub fn translate_ferb(english: &str) -> String {
    return translate_with_style(english, "erb", "ferb");
}

pub fn translate_with_style(english: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    //Convert the string slices to byte slices and translate those (only ASCII letters are affected, non-letters or UTF-8 are preserved)
    let mut pig_latin_string_bytes = Vec::<u8>::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    translate_byte_string_with_style(english.as_bytes(), suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &mut pig_latin_string_bytes);

    //This is safe since translate_byte_string_with_style does not touch any unicode bytes (it just copies them)
    return unsafe { String::from_utf8_unchecked(pig_latin_string_bytes) };
}
