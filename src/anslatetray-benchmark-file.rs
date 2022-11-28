/* anslatetray-benchmark-file
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Frontend to translate a file for benchmarking purposes
 *
*/

/* Imports */

use anslatortray::translate;
use anslatortray::translate_ascii;

/* Constants */

const ITERATIONS: u128 = 100;

/* Functions */

fn main() {
    eprintln!("Note: anslatetray-benchmark-file is highly experimental and has poor error handling. You have been warned.");

    //Get all arguments after the executable's name
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: expected one argument, the input file to be translated and benchmarked");
        return;
    }

    //TODO error handling

    let input_file = std::env::args().nth(1).unwrap();
    let file_contents = std::fs::read_to_string(input_file).unwrap();

    let mut total_duration_utf8 = std::time::Duration::new(0, 0);

    for _ in 0..ITERATIONS {
        let start_time = std::time::Instant::now();
        let translated_file_contents = translate(&file_contents);
        let time_to_translate = start_time.elapsed();

        total_duration_utf8 += time_to_translate;
        std::fs::write("/dev/null", &translated_file_contents).unwrap();
    }
    eprintln!("Sucessful: UTF-8 translation took {}ns to translate on average over {} runs.", total_duration_utf8.as_nanos() / ITERATIONS, ITERATIONS);

    for character in file_contents.chars() {
        if !character.is_ascii() {
            eprintln!("Not performing ASCII translation benchmarks as the file's contents are not entirely ASCII.");
            return;
        }
    }

    let mut total_duration_ascii = std::time::Duration::new(0, 0);

    for _ in 0..ITERATIONS {
        let start_time = std::time::Instant::now();
        let translated_file_contents = translate_ascii(&file_contents);
        let time_to_translate = start_time.elapsed();
        total_duration_ascii += time_to_translate;
        std::fs::write("/dev/null", &translated_file_contents).unwrap();
    }
    eprintln!("Sucessful: ASCII translation took {}ns to translate on average over {} runs.", total_duration_ascii.as_nanos() / ITERATIONS, ITERATIONS);
}
