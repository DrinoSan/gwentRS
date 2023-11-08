use std::env;

mod oop;
// use oop::{Interface, Game};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", oop::Interface::ERROR_WRONG_ARGUMENTS);
        std::process::exit(oop::Interface::RETURN_ERROR_ARGUMENT);
    }

    // oop for the module folder, First Interface for the file, second Interface for the struct
    // itself
    let interface = oop::Interface::Interface::new();
    let game = oop::Game::Game::new(interface);

    dbg!(args);
}
