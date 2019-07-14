/*
 * See https://en.wiktionary.org/wiki/Faden
 */

use std::io;

fn main() {
    println!("How do you feel today?");

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_n) => {
            println!("I see, you are {}", line);
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }
}
