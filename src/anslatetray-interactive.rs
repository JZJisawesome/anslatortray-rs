/* anslatetray-interactive
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Simple command-line frontend for the Anslatortray for Rust (interactive)
 *
*/

/* Imports */

use anslatortray::translate;

/* Functions */

fn main() {
    //Ensure there were no arguments
    if std::env::args().len() > 1 {
        eprintln!("Error: didn't expect any arguments");
        return;
    }
    eprintln!("Welcome to anslatetray-interactive, powered by Anslatortray!");
    eprintln!("Type what you'd like to translate and then press enter, or press Ctrl+C to exit...");

    let mut buffer = String::new();
    loop {
        eprint!("anslatetray> ");
        std::io::stdin().read_line(&mut buffer).unwrap();
        eprint!("{}", translate(&buffer));
    }
}
