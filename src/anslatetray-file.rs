/* anslatetray-file
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Frontend to translate a file
 *
*/

/* Imports */

use anslatortray::translate;

/* Functions */

fn main() {
    eprintln!("Note: This anslatetray-file is highly experimental and has poor error handling. You have been warned.");

    //Get all arguments after the executable's name
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: expected two arguments, the input file to be translated and the file to output the translated text to");
        return;
    }

    //TODO error handling
    //TODO preserve file formatting better

    let input_file = std::env::args().nth(1).unwrap();
    let output_file = std::env::args().nth(2).unwrap();

    let file_contents = std::fs::read_to_string(input_file).unwrap();
    let start_time = std::time::Instant::now();
    let translated_file_contents = translate(&file_contents);
    let time_to_translate = start_time.elapsed();
    std::fs::write(output_file, &translated_file_contents).unwrap();

    eprintln!("Sucessful: took {}ns to translate", time_to_translate.as_nanos());
}
