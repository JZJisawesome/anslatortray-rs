/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

use crate::byte_string::translate_with_style as translate_byte_string_with_style;

/* Constants */

//TODO

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

//TODO

/* Associated Functions and Methods */

//TODO

/* Functions */

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
    if english.is_empty() {
        return String::new();
    }

    //Convert the string slices to byte slices and translate those (only ASCII letters are affected, non-letters or UTF-8 are preserved)
    let mut pig_latin_string_bytes = Vec::<u8>::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    translate_byte_string_with_style(english.as_bytes(), suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &mut pig_latin_string_bytes);

    //This is safe since translate_byte_string_with_style does not touch any unicode bytes (it just copies them)
    return unsafe { String::from_utf8_unchecked(pig_latin_string_bytes) };
}
