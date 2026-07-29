use std::collections::HashMap;

use crate::*;

pub struct Eblock {
    // block for emulator
    pub id: u32,
    pub new_state: bool,
    pub inputs: Vec<u32>,
    pub blocktype: BlockType,
    // count of previous inputs
    pub previnputs: u16,
    pub delay: Vec<i32>,
}

impl Block {
    pub fn isforcpu(&self) -> bool {
        // checks if block is for cpu computation(early stage)
        matches!(self.blocktype,
            BlockType::Antenna { .. } |
            BlockType::FlipFlop |
            BlockType::Delay { .. } | 
            BlockType::Random { .. }
        )
    }
    pub fn as_eblock(&self, hash: &HashMap<u32, Vec<u32>>) -> Eblock {
        // block -> eblock, connectionhash needed
        let inputs: Vec<u32> = hash.get(&self.id).cloned().unwrap_or_default();
        Eblock { id: self.id, inputs, blocktype: self.blocktype, previnputs: 0, delay: Vec::new(), new_state: discriminant(&self.blocktype) == discriminant(&BlockType::Nor) }
    }
}

impl Save {
    pub fn connectionshash(&self) -> HashMap<u32, Vec<u32>> {
        // turns connections into hashmap collecting all their inputs
        let mut hash: HashMap<u32, Vec<u32>> = HashMap::new();

        for connection in &self.connections {
            hash.entry(connection.dst).or_default().push(connection.src);
        }

        hash
    }
}