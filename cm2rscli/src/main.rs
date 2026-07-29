#![allow(unused)]

use cm2rs::sms::execute_string;
use cm2rs::*;
use std::env;
use std::fs;
mod bview;

fn main() {
    let helpstring = "HELP FOR THIS TOOL:
        --help | -h - show this info
        -a <width> <1 or 0 for cin, b = nor> - create N bit kogge-stone
        -c <1 or 0 for enabling blueprint viewer(written by LLM)> <file_name> - run .sms file
        -e <file_name> - emulate savefile(no io for now)
    ";
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{helpstring}");
        panic!();
    }
    match args[1].as_str() {
        "--help" => println!("{helpstring}"),
        "-h" => println!("{helpstring}"),
        "-a" => {
            if args.len() < 4 {
                panic!("Expected arguments count to be 4")
            }
            let width: u32 = args[2].parse().expect("Width is not an integer");
            let sub: u8 = args[3].parse().expect("Sub is not an integer");
            let adder = rtl::Adder::new([0.0,0.0,0.0], width as usize, sub == 1);
            println!("{}", SAVE.lock().unwrap().as_string());
        }
        "-c" => {
            if args.len() < 4 {
                panic!("Expected arguments count to be 4")
            }
            let b: u8 = args[2].parse().expect("Sub is not an integer");
            let f: String = args[3].clone();

            if b == 1 {
                let _ = bview::call_bview(f.clone());
            } else {
                let fcontents = fs::read_to_string(f).expect("Couldnt read file");

                let mut ev = execute_string(fcontents);
                if ev.do_merge {
                    let save = ev.get_save("lower");
                    println!("{}", save.as_string());
                }
            }
        }
        "-e" => {
            if args.len() < 3 {
                panic!("Expected arguments count to be 4")
            }

            let f: String = args[2].clone();

            let fcontents = fs::read_to_string(f).expect("Couldnt read file");

            let save = Save::from_string(fcontents, [0.0,0.0,0.0]);

            let mut emu = sim::emulator::Emulator::new(save);

            loop {
                emu.tick();
            }


        }
        _ => println!("{helpstring}")
    }
}