/* anslatetray-file
 * By: John Jekel
 *
 * Frontend to translate a file
 *
*/

/* Imports */

use anslatortray::translate;

/* Functions */

fn main() {
    //Get all arguments after the executable's name
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: expected two arguments, the input file to be translated and the file to output the translated text to");
        return;
    }

    let input_file = std::env::args().nth(1).unwrap();
    let output_file = std::env::args().nth(2).unwrap();

    todo!();
}
