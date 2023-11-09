use std::env;

mod oop;
use oop::Game::ConfigErrorKind;
use oop::Interface;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", oop::Interface::ERROR_WRONG_ARGUMENTS);
        std::process::exit(oop::Interface::RETURN_ERROR_ARGUMENT);
    }

    // oop for the module folder, First Interface for the file, second Interface for the struct
    // itself
    let io = Interface::Interface::new();
    let mut game = oop::Game::Game::new(io);

    let config_name: &str = &args[1];
    let ret = match game.load_config(config_name) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ConfigErrorKind::FsError => panic!("File {} not found please fix your path", &args[1]),
            ConfigErrorKind::SerdeError => panic!("Json error valid is not valid"),
        },
    };

    game.print_config();

    // if ( !game.loadConfig( std::string( argv[ 1 ] ) ) )
    // {
    //     return Oop::RETURN_ERROR_BAD_CONFIG;
    // }
    // game.run();

    dbg!(args);
}
