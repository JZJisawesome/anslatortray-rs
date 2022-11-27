/* anslatetray
 * By: John Jekel
 *
 * Simple command-line frontend for the Anslatortray for Rust
 *
*/

/* Imports */

use anslatortray::translate;

/* Functions */

fn main() {
    //Get all arguments after the executable's name
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: expected at least one string to translate");
        return;
    }
    args.remove(0);

    //Translate the arguments and print them out for the user
    for string in args {
        print!("{} ", translate(&string));
    }
    println!();
}
