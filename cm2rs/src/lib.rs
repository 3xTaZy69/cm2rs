#![allow(unused)]

pub mod sim;
pub mod sms;
pub mod verilogy;
pub mod bus;
use std::{clone, mem::discriminant, sync::{LazyLock, Mutex, atomic::AtomicU32}};
use std::collections::HashMap;

use crate::sms::SmsBlock;
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
pub static SAVE: LazyLock<Mutex<Save>> = LazyLock::new(|| Mutex::new(Save::new()));

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum AntennaContext {
    Local = 0,
    Global = 1,
}

impl AntennaContext {
    pub fn fromi32(int: i32) -> AntennaContext {
        match int {
            0 => Self::Local,
            1 => Self::Global,
            _ => panic!("argument {int} is not a valid antenna context")
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Material {
    Stud = 1,
    Plastic = 2,
    Foil = 3,
    Neon = 4,
    Forcefield = 5,
    Glass = 6,
    Grass = 7,
    Wood = 8,
    Slate = 9,
    Sand = 10,
    Granite = 11,
    Concrete = 12,
    DiamondPlate = 13,
}

impl Material {
    pub fn fromi32(int: i32) -> Material {
        match int {
            1 => Material::Stud,
            2 => Material::Plastic,
            3 => Material::Foil,
            4 => Material::Neon,
            5 => Material::Forcefield,
            6 => Material::Glass,
            7 => Material::Grass,
            8 => Material::Wood,
            9 => Material::Slate,
            10 => Material::Sand,
            11 => Material::Granite,
            12 => Material::Concrete,
            13 => Material::DiamondPlate,
            _ => panic!("argument {int} is not a valid material")
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Collision {
    Normal = 0,
    Collider = 1,
}

impl Collision {
    pub fn fromi32(int: i32) -> Collision {
        match int {
            0 => Collision::Normal,
            1 => Collision::Collider,
            _ => panic!("argument {int} is not a valid collision")
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SoundInstrument {
    Sine = 0,
    Square = 1,
    Triangle = 2,
    Sawtooth = 3,
    Meow = 4,
    Snare = 5,
}

impl SoundInstrument {
    pub fn fromi32(int: i32) -> SoundInstrument {
        match int {
            0 => Self::Sine,
            1 => Self::Square,
            2 => Self::Triangle,
            3 => Self::Sawtooth,
            4 => Self::Meow,
            5 => Self::Snare,
            _ => panic!("argument {int} is not a valid sound instrument")
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum BlockType {
    Nor, // 0
    And, // 1
    Or, // 2
    Xor, // 3
    Button, // 4
    FlipFlop, // 5
    Led { r: u8, g: u8, b: u8, opacityon: u8, opacityoff: u8, analog: f32 }, // 6
    Sound { freq: f32, instrument: SoundInstrument }, // 7
    Conductor, // 8
    Nand, // 10
    Xnor, // 11
    Random { probability: f32 }, // 12
    Text { symbol: u8}, // 13
    Tile { r: u8, g: u8, b: u8, material: Material, collision: Collision }, // 14
    Node, // 15
    Delay { ticks: u16 }, // 16
    Antenna { channel: u16, context: AntennaContext }, // 17
    ConductorV2, // 18
    Ledmixer { additive: f32 }, // 19
}

impl BlockType {
    pub fn as_sms(&self) -> SmsBlock {
        let id = match self {
            BlockType::Nor => 0,
            BlockType::And => 1,
            BlockType::Or => 2,
            BlockType::Xor => 3,
            BlockType::Button => 4,
            BlockType::FlipFlop => 5,
            BlockType::Led { .. } => 6,
            BlockType::Sound { .. } => 7,
            BlockType::Conductor => 8,
            BlockType::Nand => 10,
            BlockType::Xnor => 11,
            BlockType::Random { .. } => 12,
            BlockType::Text { .. } => 13,
            BlockType::Tile { .. } => 14,
            BlockType::Node => 15,
            BlockType::Delay { .. } => 16,
            BlockType::Antenna { .. } => 17,
            BlockType::ConductorV2 => 18,
            BlockType::Ledmixer { .. } => 19,
        };

        SmsBlock::fromi32(id)
    }
    pub fn as_u8(&self) -> u8 {
        match self {
            BlockType::Nor => 0,
            BlockType::And => 1,
            BlockType::Or => 2,
            BlockType::Xor => 3,
            BlockType::Button => 4,
            BlockType::FlipFlop => 5,
            BlockType::Led { .. } => 6,
            BlockType::Sound { .. } => 7,
            BlockType::Conductor => 8,
            BlockType::Nand => 10,
            BlockType::Xnor => 11,
            BlockType::Random { .. } => 12,
            BlockType::Text { .. } => 13,
            BlockType::Tile { .. } => 14,
            BlockType::Node => 15,
            BlockType::Delay { .. } => 16,
            BlockType::Antenna { .. } => 17,
            BlockType::ConductorV2 => 18,
            BlockType::Ledmixer { .. } => 19,
        }
    }
}

pub fn next_id() -> u32 {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// i prefix - independent from global SAVE, s - state
#[derive(Clone, Debug, Copy)]
pub struct Block {
    pub id: u32,
    pub blocktype: BlockType,
    pub state: bool,
    pub pos: [f32; 3]
}

impl Block {
    pub fn from_string(id: u32, string: String, off: [f32; 3]) -> Block {
        let structure: Vec<&str> = string.split(|c| c == ',' || c == '+').filter(|c| !c.is_empty()).collect();
        let blocktypen: i32 = structure[0].parse().expect("Couldnt get block type");
        let x: f32 = structure[2].parse().expect("Couldnt get block x");
        let y: f32 = structure[3].parse().expect("Couldnt get block y");
        let z: f32 = structure[4].parse().expect("Couldnt get block z");
        let pos: [f32; 3] = [x + off[0], y + off[1], z + off[2]];
        let argslen = structure.len() - 5;
        let mut args: Vec<String> = Vec::new();
        for i in 5..argslen+5 {
            args.push(structure[i].to_string())
        }
        let parse_i32 = |text: &str| -> i32 {
            text.parse().expect(&format!("Couldnt parse text: {text} into i32"))
        };
        let parse_f32 = |text: &str| -> f32 {
            text.parse().expect(&format!("Couldnt parse text: {text} into f32"))
        };
        match blocktypen {
            0 => Block::inew(id, pos, BlockType::Nor),
            1 => Block::inew(id, pos, BlockType::And),
            2 => Block::inew(id, pos, BlockType::Or),
            3 => Block::inew(id, pos, BlockType::Xor),
            4 => Block::inew(id, pos, BlockType::Button),
            5 => Block::inew(id, pos, BlockType::FlipFlop),
            6 => Block::inew(id, pos, BlockType::Led { 
                r: parse_i32(&args[0]) as u8, 
                g: parse_i32(&args[1]) as u8, 
                b: parse_i32(&args[2]) as u8, 
                opacityon: parse_i32(&args[3]) as u8,
                opacityoff: parse_i32(&args[4]) as u8, 
                analog: parse_f32(&args[5]) }),
            7 => Block::inew(id, pos, BlockType::Sound { 
                freq: parse_f32(&args[0]), 
                instrument: SoundInstrument::fromi32(parse_i32(&args[1])) }),
            8 => Block::inew(id, pos, BlockType::Conductor),
            10 => Block::inew(id, pos, BlockType::Nand),
            11 => Block::inew(id, pos, BlockType::Xnor),
            12 => Block::inew(id, pos, BlockType::Random { 
                probability: parse_f32(&args[0]) }),
            13 => Block::inew(id, pos, BlockType::Text { 
                symbol: {
                
                if args.len() > 0 {
                parse_i32(&args[0]) as u8
                } else {
                    65
                }
                
                }}),
            14 => Block::inew(id, pos, BlockType::Tile { 
                r: parse_i32(&args[0]) as u8, 
                g: parse_i32(&args[1]) as u8, 
                b: parse_i32(&args[2]) as u8, 
                material: Material::fromi32(parse_i32(&args[3])), 
                collision: Collision::fromi32(parse_i32(&args[4])) }),
            15 => Block::inew(id, pos, BlockType::Node),
            16 => Block::inew(id, pos, BlockType::Delay { 
                ticks: parse_i32(&args[0]) as u16 }),
            17 => Block::inew(id, pos, BlockType::Antenna { 
                channel: parse_i32(&args[0]) as u16, 
                context: AntennaContext::fromi32(parse_i32(&args[1])) }),
            18 => Block::inew(id, pos, BlockType::ConductorV2),
            19 => Block::inew(id, pos, BlockType::Ledmixer { 
                additive: parse_f32(&args[0]) }),
            _ => panic!("int: {blocktypen} is not a valid block type")
        }

    }
    pub fn new(pos: [f32; 3], blocktype: BlockType) -> Block {
        let id = next_id();
        let block = Block { id: id, blocktype, pos, state: false};
        SAVE.lock().unwrap().blocks.push(block);
        block
    }
    pub fn connect(&self, other: &Block) -> Connection {
        let connection = Connection { src: self.id, dst: other.id };
        SAVE.lock().unwrap().connections.push(connection);
        connection  
    }
    pub fn iconnect(&self, other: &Block) -> Connection {
        Connection { src: self.id, dst: other.id }
    }
    pub fn as_string(&self) -> String {
        let idx = match &self.blocktype {
            BlockType::Nor => 0,
            BlockType::And => 1,
            BlockType::Or => 2,
            BlockType::Xor => 3,
            BlockType::Button => 4,
            BlockType::FlipFlop => 5,
            BlockType::Led { .. } => 6,
            BlockType::Sound { .. } => 7,
            BlockType::Conductor => 8,
            BlockType::Nand => 10,
            BlockType::Xnor => 11,
            BlockType::Random { .. } => 12,
            BlockType::Text { .. } => 13,
            BlockType::Tile { .. } => 14,
            BlockType::Node => 15,
            BlockType::Delay { .. } => 16,
            BlockType::Antenna { .. } => 17,
            BlockType::ConductorV2 => 18,
            BlockType::Ledmixer { .. } => 19,
        };
        let x_str = self.pos[0].to_string();
        let y_str = self.pos[1].to_string();
        let z_str = self.pos[2].to_string();
        let noargs: String = format!("{},{},{},{},{},", 
            idx, 
            if self.state == true {"1"} else {""}, 
            if self.pos[0] != 0.0 {&x_str} else {""}, 
            if self.pos[1] != 0.0 {&y_str} else {""},
            if self.pos[2] != 0.0 {&z_str} else {""}
        );
        match self.blocktype.clone() {
            BlockType::Antenna { channel, context } => {
                format!("{}{}+{}", noargs, channel, context as u8)
            }
            BlockType::Delay { ticks } => {
                format!("{}{}", noargs, ticks)
            }
            BlockType::Led { r, g, b, opacityon, opacityoff, analog } => {
                format!("{}{}+{}+{}+{}+{}+{}", noargs, r, g, b, opacityon, opacityoff, analog)
            }
            BlockType::Ledmixer { additive } => {
                format!("{}{}", noargs, additive)
            }
            BlockType::Random { probability } => {
                format!("{}{}", noargs, probability)
            }
            BlockType::Tile { r, g, b, material, collision } => {
                format!("{}{}+{}+{}+{}+{}", noargs, r, g, b, material as u8, collision as u8)
            }
            BlockType::Sound { freq, instrument } => {
                format!("{}{}+{}", noargs, freq, instrument as u8)
            }
            BlockType::Text { symbol } => {
                format!("{}{}", noargs, symbol)
            }
            BlockType::FlipFlop => {
                format!("{}0+0", noargs)
            }
            _ => {
                noargs
            }
        }
    }
    pub fn inew(id: u32, pos: [f32 ;3], blocktype: BlockType) -> Block {
        Block { id, blocktype, pos, state: false }
    }
    pub fn sinew(id: u32, pos: [f32; 3], blocktype: BlockType, state: bool) -> Block {
        Block { id, blocktype, state, pos }
    }
    pub fn inew_noid(pos: [f32 ;3], blocktype: BlockType) -> Block {
        Block { id: next_id(), blocktype, state: false, pos }
    }
    pub fn snew(pos: [f32; 3], blocktype: BlockType, state: bool) -> Block {
        let block = Block { id: next_id(), blocktype, state, pos };
        SAVE.lock().unwrap().blocks.push(block.clone());
        block
    }
    pub fn add_to_global_save(self) {
        SAVE.lock().unwrap().blocks.push(self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub src: u32,
    pub dst: u32,
}

impl Connection {
    pub fn as_string(&self) -> String {
        format!("{},{}", self.src, self.dst)
    }

    pub fn from_string(string: String) -> Self {
        let structure: Vec<String> = string.split(',').map(|x| x.to_string()).collect();
        let dst = structure[1].parse().expect("Couldnt parse dst into u32");
        let src = structure[0].parse().expect("Couldnt parse src into u32");
        Connection { src, dst }
    }

    pub fn inew(src: u32, dst: u32) -> Self {
        Connection { src, dst }
    }
}


use std::ops::{Shl, Shr};

impl Shl for Block {
    type Output = Connection;

    fn shl(self, rhs: Self) -> Self::Output {
        self.connect(&rhs)
    } 

}

impl Shr for Block {
    type Output = Connection;

    fn shr(self, rhs: Self) -> Self::Output {
        self.connect(&rhs)
    } 

}

impl Shl for &Block {
    type Output = Connection;

    fn shl(self, rhs: Self) -> Self::Output {
        rhs.connect(&self)
    } 

}

impl Shr for &Block {
    type Output = Connection;

    fn shr(self, rhs: Self) -> Self::Output {
        self.connect(rhs)
    } 

}


#[derive(Clone, Debug)]
pub enum BuildingType {
    AsciiKeyInput,
    Assembler,
    Divider,
    Door,
    DualMemory,
    FunctionGenerator,
    Graph,
    HugeMemory,
    IntegratedCircuit,
    KeyInput,
    LargeRGBDisplay,
    MassMemory,
    MassiveMemory,
    Multiplier,
    NTransistor,
    PTransistor,
    PixelDisplay,
    QwertyInput,
    RGBDisplay,
    RealTimeClock,
    Sign,
    TextConsole,
    Divider32Bit,
    Multiplier32Bit,
}

impl BuildingType {
    pub fn from_string(string: String) -> Self {
        match string.as_str() {
            "AsciiKeyInput" => Self::AsciiKeyInput,
            "Assembler" => Self::Assembler,
            "Divider" => Self::Divider,
            "Door" => Self::Door,
            "DualMemory" => Self::DualMemory,
            "FunctionGenerator" => Self::FunctionGenerator,
            "Graph" => Self::Graph,
            "HugeMemory" => Self::HugeMemory,
            "IntegratedCircuit" => Self::IntegratedCircuit,
            "KeyInput" => Self::KeyInput,
            "LargeRGBDisplay" => Self::LargeRGBDisplay,
            "MassMemory" => Self::MassMemory,
            "MassiveMemory" => Self::MassiveMemory,
            "Multiplier" => Self::Multiplier,
            "NTransistor" => Self::NTransistor,
            "PTransistor" => Self::PTransistor,
            "PixelDisplay" => Self::PixelDisplay,
            "QwertyInput" => Self::QwertyInput,
            "RGBDisplay" => Self::RGBDisplay,
            "RealTimeClock" => Self::RealTimeClock,
            "Sign" => Self::Sign,
            "TextConsole" => Self::TextConsole,
            "32BitDivider" => Self::Divider32Bit,
            "32BitMultiplier" => Self::Multiplier32Bit,
            _ => panic!("string: {string} is not a valid building type")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Building {
    pub buildtype: BuildingType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: [[f32; 3]; 3],
    pub connections: Vec<Option<Vec<(u8, u32)>>>
}

impl Building {
    pub fn from_string(string: String) -> Self {
        let structure: Vec<String> = string.split(',').map(|c| c.to_string()).collect();
        let buildingtype = BuildingType::from_string(structure[0].clone());
        let pos: [f32; 3] = [
            structure[1].parse().unwrap(),
            structure[2].parse().unwrap(),
            structure[3].parse().unwrap()
        ];
        let rot: [[f32; 3] ;3] = [
            [
                structure[4].parse().unwrap(),
                structure[5].parse().unwrap(),
                structure[6].parse().unwrap(),
            ],
            [
                structure[7].parse().unwrap(),
                structure[8].parse().unwrap(),
                structure[9].parse().unwrap(),
            ],
            [
                structure[10].parse().unwrap(),
                structure[11].parse().unwrap(),
                structure[12].parse().unwrap(),
            ]
        ];
        let optcon = |vec: Vec<String>| -> Vec<Option<Vec<(u8, u32)>>> {
            let mut dat: Vec<Option<Vec<(u8, u32)>>> = Vec::new();
            for v in vec {
                if v.is_empty() {
                    dat.push(None)
                } else {
                    let concollection: Vec<String> = v.split('+').map(|c| c.to_string()).filter(|v| !v.is_empty()).collect();
                    let mut connections: Vec<(u8, u32)> = Vec::new();
                    for con in concollection {
                        let direction: u8 = con[0..1].parse().unwrap();
                        let idx: u32 = con[1..].parse().unwrap();
                        connections.push((direction, idx));
                    }
                    dat.push(Some(connections));
                }
            }
            dat
        };
        let connectionsvec: Vec<String> = structure[13..].to_vec();
        let connections = optcon(connectionsvec);
        Building { buildtype: buildingtype, x: pos[0], y: pos[1], z: pos[2], rotation: rot, connections }
    }
    pub fn new(buildtype: BuildingType, pos: [f32; 3], rot: [[f32; 3]; 3], connections: Vec<Option<Vec<(u8, u32)>>>) -> Self {
        Building { buildtype, x: pos[0], y: pos[1], z: pos[2], rotation: rot, connections }
    }
    pub fn as_string(&self) -> String {
        let posrot = {
            format!("{},{},{},{},{},{},{},{},{},{},{},{}",
            self.x,
            self.y,
            self.z,
            self.rotation[0][0],
            self.rotation[0][1],
            self.rotation[0][2],
            self.rotation[1][0],
            self.rotation[1][1],
            self.rotation[1][2],
            self.rotation[2][0],
            self.rotation[2][1],
            self.rotation[2][2],
        )
        };
        let constr = |con: &Option<(u8, u32)>| -> String {
            match con {
                &None => String::new(),
                &Some(connection) => {
                    format!("{}{}",
                    connection.0,
                    connection.1)
                }
            }
        };
        let mut connetionsvec: Vec<String> = self.connections.clone().into_iter().map(|con| {
            if let Some(connection) = con {
                let mut allcon: Vec<String> = Vec::new();
                for conn in connection {
                    allcon.push(format!("{}{}", conn.0, conn.1));
                }
                allcon.join("+")
            } else {
                String::new()
            }
        } ).collect();
        let connections = connetionsvec.join(",");

        let name = match self.buildtype {
            BuildingType::Divider32Bit => "32BitDivider",
            BuildingType::Multiplier32Bit => "32BitMultiplier",
            _ => &format!("{:?}", self.buildtype)
        };
        
        format!("{},{},{}", 
        name,
        posrot,
        connections
        )
    }
    pub fn cons_as_dbg_string(&self) -> String {
        let v = self.connections.iter()
        .map(|opt| {
            opt.as_ref()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|(a, b)| format!("{a}{b}"))
                        .collect::<Vec<_>>()
                        .join("+")
                })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();
        
        let mut output_string: String = String::new();

        for con in v.iter().enumerate() {
            if con.0 % 2 == 1 {
                output_string.push_str(con.1);
                output_string.push('\n');
            } else {
                output_string.push_str(con.1);
                output_string.push_str(",   ");
            }
        }

        output_string
    }

}

#[derive(Debug, Clone)]
pub struct Save {
    pub blocks: Vec<Block>,
    pub connections: Vec<Connection>,
    pub buildings: Vec<Building>,
    pub next_id: u32
}

impl Save {
    pub fn as_string(&mut self) -> String{
        let mut buildingstr: String = String::new();
        for building in &self.buildings {
            buildingstr.push_str(&building.as_string());
            buildingstr.push(';')
        }
        if !buildingstr.is_empty() {buildingstr.pop();}
        let mut blockstr: String = String::new();
        self.blocks.sort_by_key(|b| b.id);
        for block in &self.blocks {
            blockstr.push_str(&block.as_string());
            blockstr.push(';');
        }
        if !blockstr.is_empty() {blockstr.pop();}
        let mut connectionstr: String = String::new();
        for connection in &self.connections {
            connectionstr.push_str(&connection.as_string());
            connectionstr.push(';');
        }
        if !connectionstr.is_empty() {connectionstr.pop();}
        connectionstr.push('?');
        blockstr.push('?');
        blockstr.push_str(&connectionstr);
        blockstr.push_str(&buildingstr);
        blockstr
    }
    pub const fn new() -> Save {
        Save { blocks: Vec::new(), connections: Vec::new(), buildings: Vec::new(), next_id: 1}
    }
    pub fn from_string(string: String, off: [f32; 3]) -> Save {
        let mut id = 0;
        let mut structure: Vec<String> = string.split('?').map(|c| c.to_string()).collect();
        while structure.len() < 3 {
            structure.push(String::new());
        }
        let blocks: Vec<Block> = structure[0].split(';').filter(|c| !c.is_empty()).map(|b| {id += 1; Block::from_string(id, b.to_string(), off)}).collect();
        let connections: Vec<Connection> = structure[1].split(';').filter(|c| !c.is_empty()).map(|c| Connection::from_string(c.to_string())).collect();
        let buildings: Vec<Building> = structure[2].split(';').filter(|c| !c.is_empty()).map(|b| Building::from_string(b.to_string())).collect();
        let save = Self::from_bcb(blocks, connections, buildings);
        save
    }
    pub fn from_bcb(blocks: Vec<Block>, connections: Vec<Connection>, buildings: Vec<Building> ) -> Self {
        let id = blocks.len();
        Save { blocks, connections, buildings, next_id: id as u32 + 1 }
    }
    // O(n) + clone for each n
    pub fn get_blocks_hash(&self) ->  HashMap<u32, Block> {
        let mut hash: HashMap<u32, Block> = HashMap::new();
        for block in &self.blocks {
            hash.insert(block.id, block.clone());
        }
        hash
    }
    pub fn reset(&mut self) {
        self.blocks.clear();
        self.connections.clear();
        self.buildings.clear();
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BPortD {
    Input = 1,
    Output = 0
}

impl BPortD {
    pub fn from_u8(i: u8) -> Self {
        match i {
            0 => Self::Output,
            1 => Self::Input,
            // no error handling
            _ => Self::Output
        }
    }
    pub fn as_u8(&self) -> u8 {
        (self == &BPortD::Input) as u8
    }
    pub fn from_connections(c: &Option<Vec<(u8, u32)>>) -> Option<Vec<(BPortD, u32)>>{
        match c {
            None => None,
            Some(v) => Some(v.iter().map(|con| (BPortD::from_u8(con.0), con.1)).collect())
        }
    }
}

// Connectible trait for (u8, u32) and Block
pub trait Connectible {
    fn as_u8u32(&self) -> (u8, u32);
}

impl Connectible for (u8, u32) {
    fn as_u8u32(&self) -> (u8, u32) {
        (self.0, self.1)
    }
}

impl Connectible for (u8, Block) {
    fn as_u8u32(&self) -> (u8, u32) {
        (self.0, self.1.id)
    }
}

impl Connectible for (BPortD, Block) {
    fn as_u8u32(&self) -> (u8, u32) {
        ((self.0 == BPortD::Input) as u8, self.1.id)
    }
}

#[allow(non_snake_case)]
pub mod AdvancedBuildings {
    use super::*;

    pub const DEFAULT_UP_Z_ROT: [[f32; 3]; 3] = [[0.0,0.0,-1.0],[-1.0,0.0,0.0],[0.0,1.0,0.0]];
    pub const DEFAULT_LAY_Z_ROT: [[f32; 3]; 3] = [[0.0,0.0,-1.0],[0.0,1.0,0.0],[1.0,0.0,0.0]];

    pub fn connectible_to_vec(c: Vec<&dyn Connectible>) -> Vec<(u8, u32)> {
        c.iter().map(|v| v.as_u8u32()).collect()
    }

    pub fn block_vec_to_connectible(v: Vec<Block>, direction: BPortD) -> Vec<Option<Vec<(BPortD, Block)>>> {
        let mut vec: Vec<Option<Vec<(BPortD, Block)>>> = Vec::new();

        for block in v {
            vec.push(Some(vec![(direction, block)]));
        }

        vec
    }

    pub fn find_sequence(blocks: Vec<Block>, building: Building) {
        // 4 buses
        eprintln!("Finding connections");
        let mut xyz_hash: HashMap<(i32, i32, i32), &Block> = HashMap::new();
        let mut id_xyz: HashMap<u32, (i32, i32, i32)> = HashMap::new();
        let mut coords_vec: Vec<(i32, i32, i32)> = Vec::new();

        for block in &blocks {
            xyz_hash.insert((block.pos[0] as i32, block.pos[1] as i32, block.pos[2] as i32), &block);
            coords_vec.push((block.pos[0] as i32, block.pos[1] as i32, block.pos[2] as i32));
            id_xyz.insert(block.id, (block.pos[0] as i32, block.pos[1] as i32, block.pos[2] as i32));
        }

        println!("Finding least and most coordinates");

        let mut least_z = coords_vec.clone();
        least_z.sort_by_key(|v| v.2);
        let mut most_z = least_z.clone();
        most_z.reverse();

        let mut least_x = coords_vec.clone();
        least_x.sort_by_key(|v| v.0);
        let mut most_x = least_x.clone();
        most_x.reverse();

        let least_z_block = &least_z[0];
        let least_x_block = &least_x[0];
        let most_z_block = &most_z[0];
        let most_x_block = &most_x[0];

        println!("Finding connections and positions");

        for (idx, connection) in building.connections.iter().enumerate() {
            if let Some(v) = connection {
                println!("{idx}: id{}: {:?}", &v[0].1, id_xyz.get(&v[0].1).unwrap());
            }
        }

        println!("least z: {least_z_block:?}, least x: {least_x_block:?}, most z: {most_z_block:?}, most x: {most_x_block:?}");

        let mut bottom_left_bus: Vec<Block> = Vec::new();
        let mut next_coors: (i32, i32, i32) = (least_x_block.0, 0, most_z_block.2);

        while let Some(block) = xyz_hash.get(&next_coors) {
            bottom_left_bus.push((*block).clone());
            next_coors.0 += 1;
        }

        let mut bottom_right_bus: Vec<Block> = Vec::new();
        let mut next_coors: (i32, i32, i32) = (most_x_block.0, 0, most_z_block.2);
        let mut write: Option<Block> = None;

        while let Some(block) = xyz_hash.get(&next_coors) {
            bottom_right_bus.push((*block).clone());
            next_coors.0 -= 1;
        }

        if bottom_right_bus.len() == 1 {
            write = Some(bottom_right_bus[0]);
            bottom_right_bus.clear();
            next_coors.0 -= 1;
            while let Some(block) = xyz_hash.get(&next_coors) {
                bottom_right_bus.push((*block).clone());
                next_coors.0 -= 1;
            }
        }
        

        let mut top_right_bus: Vec<Block> = Vec::new();
        let mut next_coors: (i32, i32, i32) = (most_x_block.0, 0, least_z_block.2);

        while let Some(block) = xyz_hash.get(&next_coors) {
            top_right_bus.push((*block).clone());
            next_coors.0 -= 1;
        }

        let mut top_left_bus: Vec<Block> = Vec::new();
        let mut next_coors: (i32, i32, i32) = (least_x_block.0, 0, least_z_block.2);

        while let Some(block) = xyz_hash.get(&next_coors) {
            top_left_bus.push((*block).clone());
            next_coors.0 += 1;
        }

        bottom_right_bus.reverse();
        top_right_bus.reverse();

        println!("Found busses, widths: bottom-left: {}, bottom-right: {}, top-left: {}, top-right: {}", bottom_left_bus.len(), bottom_right_bus.len(), top_left_bus.len(), top_right_bus.len());

        let mut bl_id_hash: HashMap<u32, (usize, &Block)> = HashMap::new();
        bottom_left_bus.iter().for_each(|b| {let len = bl_id_hash.len(); bl_id_hash.insert(b.id, (len, b));});

        let mut br_id_hash: HashMap<u32, (usize, &Block)> = HashMap::new();
        bottom_right_bus.iter().for_each(|b| {let len = br_id_hash.len(); br_id_hash.insert(b.id, (len, b));});

        let mut tl_id_hash: HashMap<u32, (usize, &Block)> = HashMap::new();
        top_left_bus.iter().for_each(|b| {let len = tl_id_hash.len(); tl_id_hash.insert(b.id, (len, b));});

        let mut tr_id_hash: HashMap<u32, (usize, &Block)> = HashMap::new();
        top_right_bus.iter().for_each(|b| {let len = tr_id_hash.len(); tr_id_hash.insert(b.id, (len, b));});

        let mut output: String = String::new();

        for (idx, con) in building.connections.iter().enumerate() {
            if let Some(v) = con {
                let id = &v[0].1;

                if let Some((bit, block)) = bl_id_hash.get(id) {
                    output.push_str(&format!("{idx}: bl[{bit}]\n"));
                } else if let Some((bit, block)) = br_id_hash.get(id) {
                    output.push_str(&format!("{idx}: br[{bit}]\n"));
                } else if let Some((bit, block)) = tl_id_hash.get(id) {
                    output.push_str(&format!("{idx}: tl[{bit}]\n"));
                } else if let Some((bit, block)) = tr_id_hash.get(id) {
                    output.push_str(&format!("{idx}: tr[{bit}]\n"))
                } else if let Some(block) = &write {
                    if block.id == *id {
                        output.push_str(&format!("{idx}: write\n"));
                    }
                } else {
                    panic!("This block is something else: {idx}")
                }
            
            }

        }

        println!("---------------\n{output}");



    }

    /// Create text console!!!
    pub fn TextConsole(
        loc: [Option<Vec<&dyn Connectible>>; 8], 
        chr: [Option<Vec<&dyn Connectible>>; 8], 
        clear: Option<Vec<&dyn Connectible>>, 
        cursor: Option<Vec<&dyn Connectible>>, 
        write: Option<Vec<&dyn Connectible>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        // chr + clear + cursor + loc + write
        let connections: Vec<Option<Vec<(u8, u32)>>> = chr.
            into_iter()
            .chain([clear, cursor])
            .chain(loc)
            .chain([write])
            .map(
                |optionvec|
                optionvec.map(
                    |some|
                    connectible_to_vec(some)
                )
            ).collect();

        Building::new(BuildingType::TextConsole, pos, rot, connections)
    }

    pub fn HugeMemory(
        address: [Option<Vec<&dyn Connectible>>; 16], 
        value: [Option<Vec<&dyn Connectible>>; 16], 
        output: [Option<Vec<&dyn Connectible>>; 16], 
        write: Option<Vec<&dyn Connectible>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building { 
        // addr[0] + addr[9..=15] + addr[1..=8] + out(same as addr) + value(same as addr) + write
        let connections: Vec<Option<Vec<(u8, u32)>>>  = std::iter::once(&address[0])
            .chain(&address[9..=15])
            .chain(&address[1..=8])
            .chain(&output[0..1])
            .chain(&output[9..=15])
            .chain(&output[1..=8])
            .chain(&value[0..1])
            .chain(&value[9..=15])
            .chain(&value[1..=8])
            .chain(std::iter::once(&write))
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();
        
        Building::new(BuildingType::HugeMemory, pos, rot, connections)
    }

    pub fn Multiplier(
        a: [Option<Vec<(BPortD, Block)>>; 16],
        b: [Option<Vec<(BPortD, Block)>>; 16],
        upper: [Option<Vec<(BPortD, Block)>>; 16],
        lower: [Option<Vec<(BPortD, Block)>>; 16],
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        /* Multiplier connections: 
            a[0], a[9..=15], a[1..=8],b[0],b[9..=15],b[1..=8],
            upper[0],upper[9..=15],lower[0..=2],upper[1],lower[3..=10],
            lower[11..=12],upper[2],lower[13..=15],upper[3..=8] 
        */

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&a[0])
            .chain(&a[9..=15])
            .chain(&a[1..=8])
            .chain(std::iter::once(&b[0]))
            .chain(&b[9..=15])
            .chain(&b[1..=8])
            .chain(std::iter::once(&upper[0]))
            .chain(&upper[9..=15])
            .chain(&lower[0..=2])
            .chain(std::iter::once(&upper[1]))
            .chain(&lower[3..=10])
            .chain(&lower[11..=12])
            .chain(std::iter::once(&upper[2]))
            .chain(&lower[13..=15])
            .chain(&upper[3..=8])
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();
        
        Building::new(BuildingType::Multiplier, pos, rot, connections)
    }

    pub fn Divider(
        a: [Option<Vec<(BPortD, Block)>>; 16],
        b: [Option<Vec<(BPortD, Block)>>; 16],
        quot: [Option<Vec<(BPortD, Block)>>; 16],
        rem: [Option<Vec<(BPortD, Block)>>; 16],
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        /*
            a[0], a[9..=15], a[1..=8], b[0], b[9..=15],
            b[1..=8], quot[0], quot[9..=15], quot[1..=8], 
            rem[0], rem[9..=15], rem[1..=8]
         */

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&a[0])
            .chain(&a[9..=15])
            .chain(&a[1..=8])
            .chain(std::iter::once(&b[0]))
            .chain(&b[9..=15])
            .chain(&b[1..=8])
            .chain(std::iter::once(&quot[0]))
            .chain(&quot[9..=15])
            .chain(&quot[1..=8])
            .chain(std::iter::once(&rem[0]))
            .chain(&rem[9..=15])
            .chain(&rem[1..=8])
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();

        Building::new(BuildingType::Divider, pos, rot, connections)
    }

    pub fn DualMemory(
        load_addr: [Option<Vec<(BPortD, Block)>>; 8],
        save_addr: [Option<Vec<(BPortD, Block)>>; 8],
        output: [Option<Vec<(BPortD, Block)>>; 8],
        value: [Option<Vec<(BPortD, Block)>>; 8],
        write: Option<Vec<(BPortD, Block)>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        // sa[0..=4], sa[5..=7], la[0..=7], output[0..=7], value[0..=7], write

        let connections: Vec<Option<Vec<(u8, u32)>>> = save_addr.into_iter()
            .chain(load_addr)
            .chain(output)
            .chain(value)
            .chain(std::iter::once(write))
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();
        
        Building::new(BuildingType::DualMemory, pos, rot, connections)
    }

    pub fn MassMemory(
        addr: [Option<Vec<(BPortD, Block)>>; 12],
        output: [Option<Vec<(BPortD, Block)>>; 8],
        value: [Option<Vec<(BPortD, Block)>>; 8],
        write: Option<Vec<(BPortD, Block)>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        // addr[0], addr[9..=11], addr[1..=8], output[0..=7], value[0..=7], write

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&addr[0])
            .chain(&addr[9..=11])
            .chain(&addr[1..=8])
            .chain(&output[0..=7])
            .chain(&value[0..=7])
            .chain(std::iter::once(&write))
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();
        
        Building::new(BuildingType::MassMemory, pos, rot, connections)
    }

    pub fn MassiveMemory(
        addr: [Option<Vec<(BPortD, Block)>>; 12],
        output: [Option<Vec<(BPortD, Block)>>; 16],
        value: [Option<Vec<(BPortD, Block)>>; 16],
        write: Option<Vec<(BPortD, Block)>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        // addr[0], addr[9..=11], addr[1..=8], output[0], output[9..=15], output[1..=8], value[0], value[9..=15], value[1..=8], write

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&addr[0])
            .chain(&addr[9..=11])
            .chain(&addr[1..=8])
            .chain(std::iter::once(&output[0]))
            .chain(&output[9..=15])
            .chain(&output[1..=8])
            .chain(std::iter::once(&value[0]))
            .chain(&value[9..=15])
            .chain(&value[1..=8])
            .chain(std::iter::once(&write))
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();

        Building::new(BuildingType::MassiveMemory, pos, rot, connections)
    }

    pub fn Multiplier32Bit( //? fixed
        a: [Option<Vec<(BPortD, Block)>>; 32],
        b: [Option<Vec<(BPortD, Block)>>; 32],
        lower: [Option<Vec<(BPortD, Block)>>; 32],
        upper: [Option<Vec<(BPortD, Block)>>; 32],
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        /* 
           a[0], a[9..=18], a[1], a[19..=28], a[2], a[29..=31], a[3..=8], 
           b[0], b[9..=18], b[1], b[19..=28], b[2], b[29..=31], b[3..=8], 
           upper[0], upper[9..=18], upper[1], upper[19..=28], upper[2], 
           upper[29..=31], lower[0..=6], upper[3], lower[7..=16], upper[4], 
           lower[17..=26], upper[5], lower[27..=31], upper[6..=8]
         */

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&a[0])
            .chain(&a[9..=18])
            .chain(std::iter::once(&a[1]))
            .chain(&a[19..=28])
            .chain(std::iter::once(&a[2]))
            .chain(&a[29..=31])
            .chain(&a[3..=8])
            .chain(std::iter::once(&b[0]))
            .chain(&b[9..=18])
            .chain(std::iter::once(&b[1]))
            .chain(&b[19..=28])
            .chain(std::iter::once(&b[2]))
            .chain(&b[29..=31])
            .chain(&b[3..=8])
            .chain(std::iter::once(&upper[0]))
            .chain(&upper[9..=18])
            .chain(std::iter::once(&upper[1]))
            .chain(&upper[19..=28])
            .chain(std::iter::once(&upper[2]))
            .chain(&upper[29..=31])
            .chain(&lower[0..=6])
            .chain(std::iter::once(&upper[3]))
            .chain(&lower[7..=16])
            .chain(std::iter::once(&upper[4]))
            .chain(&lower[17..=26])
            .chain(std::iter::once(&upper[5]))
            .chain(&lower[27..=31])
            .chain(&a[6..=8])
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();
        
        Building::new(BuildingType::Multiplier32Bit, pos, rot, connections)
    }

    pub fn Divider32Bit(
        a: [Option<Vec<(BPortD, Block)>>; 32],
        b: [Option<Vec<(BPortD, Block)>>; 32],
        quot: [Option<Vec<(BPortD, Block)>>; 32],
        rem: [Option<Vec<(BPortD, Block)>>; 32],
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> Building {
        /*
            a[0], a[17..=24], a[25..=31], a[1..=8], a[9..=16], b[0],
            b[17..=24], b[25..=31], b[1..=8], b[9..=16], quot[0],
            quot[17..=24], quot[25..=31], quot[1..=8], quot[9..=16],
            rem[0], rem[17..=24], rem[25..=31], rem[1..=8], rem[9..=16]
         */

        let connections: Vec<Option<Vec<(u8, u32)>>> = std::iter::once(&a[0])
            .chain(&a[17..=24])
            .chain(&a[25..=31])
            .chain(&a[1..=8])
            .chain(&a[9..=16])
            .chain(std::iter::once(&b[0]))
            .chain(&b[17..=24])
            .chain(&b[25..=31])
            .chain(&b[1..=8])
            .chain(&b[9..=16])
            .chain(std::iter::once(&quot[0]))
            .chain(&quot[17..=24])
            .chain(&quot[25..=31])
            .chain(&quot[1..=8])
            .chain(&quot[9..=16])
            .chain(std::iter::once(&rem[0]))
            .chain(&rem[17..=24])
            .chain(&rem[25..=31])
            .chain(&rem[1..=8])
            .chain(&rem[9..=16])
            .map(
                |opt|
                opt.as_ref().map(
                    |v|
                    v.iter().map(
                        |c|
                        c.as_u8u32()
                    ).collect()
                )
            ).collect();

        Building::new(BuildingType::Divider32Bit, pos, rot, connections)
    }

}

pub mod lut {
    use core::fmt;

    use super::*;

    pub struct LuToverview {
        luts: u64,
        nands: u64,
        ands: u64,
        ors: u64,
        nots: u64,
        xors: u64,
        wires: u64,
    }

    impl fmt::Display for LuToverview {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let overview = format!("===LuT overview===
    LUTs: {}
    NANDs: {}
    ANDs: {}
    ORs: {}
    NOTs: {}
    XORs: {}
    WIREs: {}

            ", self.luts, self.nands, self.ands, self.ors, self.nots, self.xors, self.wires);

            write!(f, "{}", overview)
        }
    }

    pub fn count_luts(save: &Save) -> LuToverview {
        let mut hash: HashMap<u32, u16> = HashMap::new();
        let mut luts: u64 = 0;
        let mut nands: u64 = 0;
        let mut ands: u64 = 0;
        let mut ors: u64 = 0;
        let mut nots: u64 = 0;
        let mut xors: u64 = 0;
        let mut wires: u64 = 0;

        for con in &save.connections {
            *hash.entry(con.dst).or_insert(0) += 1;
        } 

        for blk in &save.blocks {

            let inputs = hash.get(&blk.id).copied().unwrap_or(0);

            luts += match inputs {

                1 => {
                    match blk.blocktype {
                        BlockType::Nor => {
                            nots += 1;
                            nands += 1;
                            1
                        }
                        _ => {
                            wires += 1;
                            1
                        }
                    }
                    
                },

                2 => {
                    match blk.blocktype {
                        BlockType::Nor => {
                            nots += 1;
                            ors += 1;
                            nands += 4;
                            1
                        }
                        BlockType::And => {
                            ands += 1;
                            nands += 2;
                            1
                        }
                        BlockType::Nand => {
                            nands += 1;
                            1
                        }
                        BlockType::Or | BlockType::Node => {
                            ors += 1;
                            nands += 3;
                            1
                        }
                        BlockType::Xor => {
                            nands += 4;
                            xors += 1;
                            1
                        }
                        _ => 1
                    }
                },

                i if i >= 3 => {
                    match blk.blocktype {
                        BlockType::Nor => {
                            nots += 1 * (i as u64 - 1);
                            ors += 1 * (i as u64 - 1);
                            nands += 4 * (i as u64 - 1);
                            (i as u64 - 1)
                        }
                        BlockType::And => {
                            ands += 1 * (i as u64 - 1);
                            nands += 2 * (i as u64 - 1);
                            (i as u64 - 1)
                        }
                        BlockType::Nand => {
                            nands += 1 * (i as u64 - 1);
                            (i as u64 - 1)
                        }
                        BlockType::Or => {
                            ors += 1 * (i as u64 - 1);
                            nands += 3 * (i as u64 - 1);
                            (i as u64 - 1)
                        }
                        BlockType::Xor => {
                            nands += 4 * (i as u64 - 1);
                            xors += 1 * (i as u64 - 1);
                            (i as u64 - 1)
                        }
                        _ => ( i as u64 - 1 )
                    }
                }

                _ => 0,

            }

        }

        LuToverview { luts, nands, ands, ors, nots, xors, wires }
    }

}

pub fn save_string() -> String {
    SAVE.lock().unwrap().as_string()
}