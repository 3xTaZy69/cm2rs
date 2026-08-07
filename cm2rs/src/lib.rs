#![allow(unused)]

pub mod sim;
pub mod sms;
pub mod rtl;
use std::{clone, mem::discriminant, sync::{LazyLock, Mutex, atomic::AtomicU32}};
use std::collections::HashMap;

use crate::sms::SmsBlock;
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
pub static SAVE: LazyLock<Mutex<Save>> = LazyLock::new(|| Mutex::new(Save::new()));

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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
        let noargs: String = format!("{},{},{},{},{},", idx, (discriminant(&self.blocktype) == discriminant(&BlockType::Nor)) as u8, self.pos[0], self.pos[1], self.pos[2]);
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
        rhs.connect(&self)
    } 

}

impl Shl for &Block {
    type Output = Connection;

    fn shl(self, rhs: Self) -> Self::Output {
        self.connect(&rhs)
    } 

}

impl Shr for &Block {
    type Output = Connection;

    fn shr(self, rhs: Self) -> Self::Output {
        rhs.connect(&self)
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
            "Divider32Bit" => Self::Divider32Bit,
            "Multiplier32Bit" => Self::Multiplier32Bit,
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
        
        format!("{:?},{},{}", 
        self.buildtype,
        posrot,
        connections
        )
    }
    pub fn cons_as_dbg_string(&self) -> String {
        self.connections.iter()
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
        .collect::<Vec<_>>()
        .join("\n")
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
    pub fn as_string(&self) -> String{
        let mut buildingstr: String = String::new();
        for building in &self.buildings {
            buildingstr.push_str(&building.as_string());
            buildingstr.push(';')
        }
        if !buildingstr.is_empty() {buildingstr.pop();}
        let mut blockstr: String = String::new();
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

#[derive(PartialEq, Debug, Clone)]
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

    pub struct TextConsolePorts {
        loc: [Option<Vec<(BPortD, u32)>>; 8], 
        chr: [Option<Vec<(BPortD, u32)>>; 8], 
        clear: Option<Vec<(BPortD, u32)>>, 
        cursor: Option<Vec<(BPortD, u32)>>, 
        write: Option<Vec<(BPortD, u32)>>,
    }

    pub struct HugeMemoryPorts {
        address: [Option<Vec<(BPortD, u32)>>; 16],
        value: [Option<Vec<(BPortD, u32)>>; 16],
        output: [Option<Vec<(BPortD, u32)>>; 16],
        write: Option<Vec<(BPortD, u32)>>,
    }

    pub fn connectible_to_vec(c: Vec<&dyn Connectible>) -> Vec<(u8, u32)> {
        c.iter().map(|v| v.as_u8u32()).collect()
    }

    pub fn extract_cons(
        connections: &Vec<Option<Vec<(u8, u32)>>>,
        range: std::ops::RangeInclusive<usize>
    ) -> Vec<Option<Vec<(BPortD, u32)>>> {
        connections[range]
            .iter()
            .cloned()
            .map(|o| BPortD::from_connections(&o))
            .collect()
    }

    pub fn extract_single(
        connections: &Vec<Option<Vec<(u8, u32)>>>,
        idx: usize
    ) -> Option<Vec<(BPortD, u32)>> {
        match &connections[idx] {
            None => None,
            Some(x) => Some(x
                .iter()
                .map(|v| (BPortD::from_u8(v.0), v.1)).collect())
        }
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
    ) -> (Building, TextConsolePorts) {
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


            let chr_ports: [Option<Vec<(BPortD, u32)>>; 8] = extract_cons(&connections, 0..=7)
                .try_into()
                .unwrap(); 
            let clear: Option<Vec<(BPortD, u32)>> = extract_single(&connections, 8);
            let cursor: Option<Vec<(BPortD, u32)>> = extract_single(&connections, 9);
            let loc_ports: [Option<Vec<(BPortD, u32)>>; 8] = extract_cons(&connections, 10..=17)
                .try_into()
                .unwrap();
            let write: Option<Vec<(BPortD, u32)>> = extract_single(&connections, 18);

            let tcp = TextConsolePorts { loc: loc_ports, chr: chr_ports, clear, cursor, write };

        (Building::new(BuildingType::TextConsole, pos, rot, connections), tcp)
    }

    pub fn HugeMemory(
        address: [Option<Vec<&dyn Connectible>>; 16], 
        value: [Option<Vec<&dyn Connectible>>; 16], 
        output: [Option<Vec<&dyn Connectible>>; 16], 
        write: Option<Vec<&dyn Connectible>>,
        pos: [f32; 3],
        rot: [[f32; 3]; 3]
    ) -> (Building, HugeMemoryPorts) { 
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

            let get_ports = |additive: usize| -> [Option<Vec<(BPortD, u32)>>; 16] {
                vec![extract_single(&connections, 0+additive)]
                    .iter()
                    .chain(extract_cons(&connections, (9+additive)..=(15+additive)).iter())
                    .chain(extract_cons(&connections, (1+additive)..=(8+additive)).iter())
                    .cloned()
                    .collect::<Vec<Option<Vec<(BPortD, u32)>>>>()
                    .try_into()
                    .unwrap()
            };

            let addr_ports: [Option<Vec<(BPortD, u32)>>; 16] = get_ports(0);
            let output_ports: [Option<Vec<(BPortD, u32)>>; 16] = get_ports(16);
            let value_ports = get_ports(32);
            let write = extract_single(&connections, 48);

            let hmp = HugeMemoryPorts {address: addr_ports, value: value_ports, output: output_ports, write};
        
        (Building::new(BuildingType::HugeMemory, pos, rot, connections), hmp)
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

