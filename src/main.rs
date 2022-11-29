use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use console::{style, Key, Term};
use dialoguer::{Input, Select};
use lamp_information::{Lamp, COLOR};

use crate::mode::print_mod_1;

mod lamp_information;
mod mode;
mod receive_signal;

fn main() {
    let lamp_edit_mod = Arc::new(Mutex::new(false));
    // let mut lamp_edit_mod = false;
    //** 打开串口 */
    let mut port = serialport::new("/./COM5", 38400)
        .timeout(Duration::from_secs(20))
        .open()
        .expect("Failed to open port");

    //** 初始化路灯 */
    let mut lamp_ns = Lamp::new(99, COLOR::GREEN);
    let mut lamp_we = Lamp::new(99, COLOR::GREEN);

    //** 初始化终端 */
    let term = Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();
    let lamp_edit_mode = Arc::clone(&lamp_edit_mod);

    //** 线程： */
    thread::spawn(move || {
        let term = Term::stdout();
        loop {
            Duration::from_millis(50);
            let key = term.read_key();
            match key {
                Ok(key) => {
                    let mut mode = lamp_edit_mode.lock().unwrap();
                    if key == Key::Char('e') {
                        *mode = true;
                    }
                }
                Err(_) => {}
            }
        }
    });

    loop {
        let mut serial_buf = vec![0; 9];
        port.read(serial_buf.as_mut_slice()).expect("");
        let message = String::from_utf8(serial_buf).unwrap();

        let message: Vec<i32> = message
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();

        lamp_ns.set_message(message[1], message[0]);
        lamp_we.set_message(message[3], message[2]);
        let mut mode = lamp_edit_mod.lock().unwrap();
        if *mode == false {
            print_mod_1(&lamp_ns, &lamp_we, &term);
        } else {
            term.move_cursor_down(10).unwrap();
            let selection = Select::new()
                .item(style("设置总倒计时").blue())
                .item(style("设置路灯信息").blue())
                .interact_on_opt(&Term::stdout())
                .unwrap();

            match selection {
                Some(position) => match position {
                    0 => {
                        port.write(sent_signal(2).as_bytes()).unwrap();
                    }
                    1 => {
                        let selection = Select::new()
                            .item(style("color: green").green())
                            .item(style("color: red").red())
                            .interact_on_opt(&Term::stdout())
                            .unwrap()
                            .unwrap();

                        match selection {
                            0 => {
                                port.write(sent_signal(1).as_bytes()).unwrap();
                            }
                            1 => {
                                port.write(sent_signal(0).as_bytes()).unwrap();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                None => println!("User did not select anything or exited using Esc or q"),
            }

            term.clear_screen().unwrap();
            term.hide_cursor().unwrap();
            *mode = false;
        }
    }
}
fn sent_signal(i: i32) -> String {
    let input: String = Input::new()
        .with_prompt("set time")
        .interact_text()
        .unwrap();
    let s = i.to_string() + &" ".to_string() + &input;
    s
}
