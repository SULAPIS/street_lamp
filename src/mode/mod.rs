use console::{style, Term};

use crate::{lamp_information::Lamp, receive_signal::print_lamp};

pub fn print_mod_1(lamp_ns: &Lamp, lamp_we: &Lamp, term: &Term) {
    println!("press {} to edit", style("e").red());
    print_lamp(
        "                  ".to_string(),
        &lamp_ns,
        "N S".to_string(),
    );
    print_lamp("".to_string(), &lamp_we, "W E".to_string());

    term.move_cursor_up(20).unwrap();
}
