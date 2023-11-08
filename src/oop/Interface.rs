pub struct Interface {}

impl Interface {
    pub fn new() -> Interface {
        Interface {}
    }
}


pub const RETURN_OK: i32 = 0;
pub const RETURN_ERROR_ALLOC: i32 = -1;
pub const RETURN_ERROR_ARGUMENT: i32 = -2;
pub const RETURN_ERROR_BAD_CONFIG: i32 = -3;

pub const ERROR_WRONG_ARGUMENTS: &str = "Usage: ./game <config-file>";
pub const ERROR_INVALID_CONFIG: &str = "Invalid config file!";
pub const ERROR_BAD_ALLOC: &str = "Memory error!";

pub const WARNING_INVALID_NAME: &str = "This name is not valid! Try a different one!";
pub const WARNING_UNKNOWN_COMMAND: &str = "Unknown command!";
pub const WARNING_WRONG_PARAMETER: &str = "Invalid command parameter!";
pub const WARNING_WRONG_PARAM_COUNT: &str = "Wrong parameter count!";
pub const WARNING_NOT_ENOUGH_MANA: &str = "Not enough mana for this move!";
pub const WARNING_EXECUTION_NOT_POSSIBLE: &str = "Execution not possible!";
pub const WARNING_SHIELD_MONSTER: &str = "Destroy all shields first!";
pub const WARNING_REBIRTH_UNSUCCESSFUL: &str = "Rebirth unsuccessful!";

pub const INFO_HELP_MSGS: [&str; 22] = [
    "Available Commands:",
    "- Attack <x> with <y>",
    "    Attack position x on the enemy side with creature at position y on your side.",
    "    The value x is the position of the victim [0=enemy, 1-7=enemy card]",
    "    The value y is the position of your monster on the gamefield [1-7=enemy card]",
    "- Set <x> to <y>",
    "    Place creature card with number x in hand at position y on your side.",
    "    The value x and y has to be between 1 and 7.",
    "- Cast <x>",
    "    Cast spell from the card on gamefield index x.",
    "    The value x has to be between 1 and 7.",
    "- Sacrifice <x>",
    "    Sacrifice hand card with index x and gain +1 health in exchange.",
    "    The value x has to be between 1 and 7.",
    "- State",
    "    Prints the current game",
    "- Finish",
    "    End your turn.",
    "- Quit",
    "    Terminates the game, which leads to the winning of the opponent player!",
    "- Help",
    "    Display this help.",
];

pub const PLAYER_1_NAME: &str = "Please enter the name of player 1: ";
pub const PLAYER_2_NAME: &str = "Please enter the name of player 2: ";
pub const TARGET_TRAITOR_SPELL: &str = "Please enter the target creature: ";
pub const SET_TRAITOR_SPELL: &str = "Please enter where you would like to set the creature: ";

pub const COMMAND_HELP: &str = "help";
pub const COMMAND_ATTACK: &str = "attack";
pub const COMMAND_SET: &str = "set";
pub const COMMAND_CAST: &str = "cast";
pub const COMMAND_SACRIFICE: &str = "sacrifice";
pub const COMMAND_STATE: &str = "state";
pub const COMMAND_FINISH: &str = "finish";
pub const COMMAND_QUIT: &str = "quit";

pub const INFO_ROUND: &str = "Round: ";
pub const INFO_CURRENT_PLAYER: &str = "Current player is ";

pub const ENDLINE_PART_ONE: &str = "Game ended. Player ";
pub const ENDLINE_PART_TWO: &str = " won!";

pub const STRING_HEALER: &str = "Healer";
pub const STRING_REBIRTH: &str = "Rebirth";
pub const STRING_DRACULA: &str = "Dracula";
pub const STRING_RELIEF: &str = "Relief";
pub const STRING_TRAITOR: &str = "Traitor";

pub const PROTOCOL: [(&str, &str); 4] = [
    ("[DEBUG]", "\033[34m"),   // BLUE
    ("[ERROR]", "\033[31m"),   // RED
    ("[WARNING]", "\033[33m"), // YELLOW
    ("[INFO]", "\033[37m"),    // WHITE
];

pub const COLOR_END: &str = "\033[0m";

pub const MAP_GAME: &str = " GAME ";
pub const MAP_HAND: &str = " HAND ";
pub const MAP_SEPERATOR: &str =
    "\n==================================================================================\n";
pub const MIDFIELD_SEPERATOR: &str =
    "\n~~~~~~ 01 ~~~~~~~ 02 ~~~~~~ 03 ~~~~~~~ 04 ~~~~~~~ 05 ~~~~~~~ 06 ~~~~~~~ 07 ~~~~~~\n";
pub const CARD_SEPERATOR: &str = "   ";
pub const PLAYER_PADDING: &str = "=============================";
pub const CARD_EMTPY: [&str; 6] = [
    " ______ ", "|      |", "|      |", "|      |", "|      |", "|______|",
];

pub const MAX_NAME_LENGTH: i32 = 8;
pub const MIN_LETTERS_NAME: i32 = 1;
pub const MAX_LETTERS_NAME: i32 = 20;

pub const MAX_MANA: i32 = 15;
pub const MAX_MANA_GAIN: i32 = 8;

pub const MAX_HAND_CARDS: i32 = 7;
pub const NUM_OF_BOARD_LINES: i32 = 6;
