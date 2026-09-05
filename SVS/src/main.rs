#![allow(unused)]

use std::fs;

use cm2rs::{
    AdvancedBuildings::{self, BuildingPorts, DEFAULT_LAY_Z_ROT, fetch_ports},
    BPortD, Block, BlockType, Connection, SAVE, Save, lut, save_string, verilogy,
};

mod parser;
mod ssa;

use cm2rs::rtl::*;
use cm2rs::sms::{Evaluator, execute_string};
use ssa::*;

pub fn main() {
    let contents = std::fs::read_to_string("./constrs.cd").unwrap();

    let c = parse_constraints(&contents);

    println!("{:?}", c);
}
