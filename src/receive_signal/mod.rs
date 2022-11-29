use console::style;

use crate::lamp_information::{Lamp, COLOR};

pub fn print_lamp(space: String, lamp: &Lamp, direction: String) {
    let s;
    match lamp.state {
        COLOR::GREEN => {
            s = "⚫⚫🟢";
        }
        COLOR::RED => {
            s = "🔴⚫⚫";
        }
        COLOR::YELLOW => {
            s = "⚫🟡⚫";
        }
    }
    println!("    {}{}", space, style(direction).blue());
    println!("{}{}  {:02}", space, s, lamp.time);
}
