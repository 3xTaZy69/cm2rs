use std::collections::HashMap;

use crate::{*, sim::{entity::*, process::*}};

pub struct Emulator {
    // blocks for gpu calculation
    pub gpu_blocks: Vec<Eblock>,
    // blocks for cpu calculation
    pub cpu_blocks: Vec<Eblock>,
    // stores all blocks states so gpu_blocks and cpu_blocks can acces them faster
    pub states: Vec<bool>,
    // buildings, stored as other type of entities
    pub buildings: Vec<Building>,
    // antenna channels
    pub channels: HashMap<u16, bool>
}

// utilites
impl Emulator {
    pub fn new(save: Save) -> Emulator {
        let mut cpu_blocks: Vec<Eblock> = Vec::new();
        let mut gpu_blocks: Vec<Eblock> = Vec::new();
        let mut states: Vec<bool> = Vec::new();
        let mut channels: HashMap<u16, bool> = HashMap::new();
        let hash = save.connectionshash();
        // loads everything from save to emulator
        let mut save2 = save.clone();
        save2.blocks.sort_by_key(|k| k.id );
        for block in save2.blocks {
            let eblock = block.as_eblock(&hash);
            states.push(eblock.new_state);
            if let BlockType::Antenna { channel, context} = &block.blocktype {
                channels.insert(*channel, false);
            }
            if block.isforcpu() {
                cpu_blocks.push(eblock);
            } else {
                gpu_blocks.push(eblock)
            }
        }
        Emulator { gpu_blocks, cpu_blocks, states, buildings: save.buildings, channels }

    }
    pub fn tick(&mut self) {
        self.calculate_cpu();
        self.calculate_gpu_by_cpu();
        

        for block in self.cpu_blocks.iter().chain(self.gpu_blocks.iter()) {
            self.states[block.id] = block.new_state;
        }
    }
    pub fn loop_emulator(&mut self) {
        loop {
            self.tick();
        }
    }
    pub fn change_state(&mut self, id: u32, state: bool) {
        self.states[id as usize - 1] = state;
    }
    pub fn print_state(&mut self, id: u32) {
        eprintln!("{}", self.states[id as usize - 1]);
    }
}
