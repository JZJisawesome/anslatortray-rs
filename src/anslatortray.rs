/* anslatortray
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Command-line frontend for the anslatortray library
 *
*/

/* Imports */

use anslatortray::translate;
use anslatortray::translate_ascii;

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

fn main() {
    eprintln!("Anslatortray: frontend for the Anslatortray for Rust library\n");

    //Get all arguments after the executable's name
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        interactive(&args);
        return;
    }

    let option = args[0].clone();
    args.remove(0);

    match option.as_str() {
        "--help" => { help(); },
        "--interactive" => { interactive(&args); },
        "--file" => { file(&args); },
        "--benchmark-file" => { benchmark_file(&args); },
        "--translate-args" => { translate_args(&args); },
        "--stdin-to-stdout" => { stdin_to_stdout(&args); },
        bad_option => {
            eprintln!("Error: {} is not a valid option", bad_option);
            help();
        }
    }
}

fn help() {
    eprintln!("Options:");
    eprintln!("--help            Print this helpful text!");
    eprintln!("--interactive     Start an interactive translation session");
    eprintln!("--file            Translate a file (requires two arguments, the file to translate and the destination)");
    eprintln!("--benchmark-file  Benchmark translating a file (requires two arguments, the file to translate and the number of iterations to perform)");
    eprintln!("--translate-args  Translates all remaining arguments provided and outputs them to stdout");
    eprintln!("--stdin-to-stdout Translates input from stdin directly to stdout");

    eprintln!("\n{}", translate_ascii("Have a good day!"));
}

fn interactive(args: &Vec<String>) {
    if args.len() != 0 {
        eprintln!("Error: didn't expect any arguments");
        help();
        return;
    }

    eprintln!("Starting interactive mode!");
    eprintln!("Type what you'd like to translate and then press enter, or press Ctrl+C to exit...\n");

    let stdin = std::io::stdin();
    let mut line_buffer = String::new();
    loop {
        eprint!("anslatortray> ");
        stdin.read_line(&mut line_buffer).unwrap();
        eprintln!("{}", translate(&line_buffer));
    }
}

fn file(args: &Vec<String>) {
    eprintln!("Note: anslatortray --file is highly experimental and has poor error handling. You have been warned.");

    if args.len() != 2 {
        eprintln!("Error: expected two arguments, two arguments, the file to translate and the destination");
        help();
        return;
    }

    //TODO error handling

    let input_file = &args[0];
    let output_file = &args[1];

    let file_contents = std::fs::read_to_string(input_file).unwrap();
    let start_time = std::time::Instant::now();
    let translated_file_contents = translate(&file_contents);
    let time_to_translate = start_time.elapsed();
    std::fs::write(output_file, &translated_file_contents).unwrap();

    eprintln!("Sucessful: took {}ns to translate", time_to_translate.as_nanos());
}

fn benchmark_file(args: &Vec<String>) {
    eprintln!("Note: anslatortray --benchmark-file is highly experimental and has poor error handling. You have been warned.");

    if args.len() != 2 {
        eprintln!("Error: expected two arguments, the file to translate and the number of iterations to perform");
        help();
        return;
    }

    //TODO error handling

    let input_file = &args[0];
    let iterations = args[1].parse::<u128>().unwrap();//TODO error handling

    let file_contents = std::fs::read_to_string(input_file).unwrap();

    let mut total_duration_utf8 = std::time::Duration::new(0, 0);

    for _ in 0..iterations {
        let start_time = std::time::Instant::now();
        let translated_file_contents = translate(&file_contents);
        let time_to_translate = start_time.elapsed();

        total_duration_utf8 += time_to_translate;
        std::fs::write("/dev/null", &translated_file_contents).unwrap();//TODO avoid needing unix
    }
    eprintln!("Sucessful: UTF-8 translation took {}ns to translate on average over {} runs.", total_duration_utf8.as_nanos() / iterations, iterations);

    for character in file_contents.chars() {
        if !character.is_ascii() {
            eprintln!("Not performing ASCII translation benchmarks as the file's contents are not entirely ASCII.");
            return;
        }
    }

    let mut total_duration_ascii = std::time::Duration::new(0, 0);

    for _ in 0..iterations {
        let start_time = std::time::Instant::now();
        let translated_file_contents = translate_ascii(&file_contents);
        let time_to_translate = start_time.elapsed();
        total_duration_ascii += time_to_translate;
        std::fs::write("/dev/null", &translated_file_contents).unwrap();//TODO avoid needing unix
    }
    eprintln!("Sucessful: ASCII translation took {}ns to translate on average over {} runs.", total_duration_ascii.as_nanos() / iterations, iterations);
}

fn translate_args(args: &Vec<String>) {
    if args.len() == 0 {
        eprintln!("Error: expected at least one string to translate");
        help();
        return;
    }

    //Translate the arguments and print them out for the user
    for string in args {
        print!("{} ", translate(&string));
    }
    println!();
}

fn stdin_to_stdout(args: &Vec<String>) {
    use std::io::{Read, Write};

    if args.len() != 0 {
        eprintln!("Error: didn't expect any arguments");
        help();
        return;
    }

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();

    while let Ok(bytes_read) = stdin.read_to_string(&mut buffer) {
        if bytes_read == 0 { return; }
        write!(stdout, "{}", translate(&buffer)).unwrap();//TODO do this more efficiently (avoid format string)
    }
}
