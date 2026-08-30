use std::collections::HashMap;

use crate::*;

pub struct Eblock {
    // block for emulator
    pub id: usize,
    pub new_state: bool,
    pub inputs: Vec<usize>,
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
    pub fn as_eblock(&self, hash: &HashMap<usize, Vec<usize>>) -> Eblock {
        // block -> eblock, connectionhash needed
        let inputs: Vec<usize> = hash.get(&(self.id as usize - 1)).cloned().unwrap_or_default();
        Eblock { id: self.id as usize - 1, inputs, blocktype: self.blocktype, previnputs: 0, delay: Vec::new(), new_state: discriminant(&self.blocktype) == discriminant(&BlockType::Nor) }
    }
}

impl Save {
    pub fn connectionshash(&self) -> HashMap<usize, Vec<usize>> {
        // turns connections into hashmap collecting all their inputs
        let mut hash: HashMap<usize, Vec<usize>> = HashMap::new();

        for connection in &self.connections {
            hash.entry(connection.dst as usize-1).or_default().push(connection.src as usize-1);
        }

        hash
    }
}