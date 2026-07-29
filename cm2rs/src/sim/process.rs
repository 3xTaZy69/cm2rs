use core::panic;
use rand;
use crate::{*, sim::{entity::*, emulator::*}};

impl Emulator {
    pub fn calculate_cpu(&mut self) {
        for block in &mut self.cpu_blocks {
            match block.blocktype {
                BlockType::FlipFlop => {
                    let mut count = 0;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] {
                            count += 1;
                        }
                    }
                    if count > block.previnputs {
                        block.new_state = !self.states[block.id as usize - 1];
                    }
                    block.previnputs = count;
                }
                BlockType::Delay { ticks } => {
                    block.new_state = false;
                    block.delay.retain_mut(|i| {
                        *i -= 1;
                        if *i == 0 {
                            block.new_state = true;
                        }
                        *i > 0
                    });
                }
                BlockType::Antenna { channel, context } => {
                    // local emulator so contexts don`t matter
                    block.new_state = *self.channels.get(&channel).unwrap();
                    *self.channels.get_mut(&channel).unwrap() = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] {
                            block.new_state = true;
                            *self.channels.get_mut(&channel).unwrap() = true;
                            break;
                        }
                    }
                }
                BlockType::Node => {
                    for input in &block.inputs {
                        self.states[block.id as usize - 1] = false;
                        if self.states[*input as usize - 1] {
                            self.states[block.id as usize - 1] = true;
                        }
                    }
                }
                BlockType::Random { probability } => {
                    block.new_state = rand::random_bool(probability as f64);
                }
                _ => {}
            }
        }
    }
    pub fn calculate_gpu_by_cpu(&mut self) {
        // test
        for block in &mut self.gpu_blocks {
            match block.blocktype {
                BlockType::And => {
                    let mut state = true;
                    for input in &block.inputs {
                        state = self.states[*input as usize - 1] && state;
                    }
                    block.new_state = state;
                }
                BlockType::Or => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Xor => {
                    block.new_state = false;
                    let mut count = 0;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            count += 1;
                        }
                    }
                    if count % 2 == 1 {
                        block.new_state = true;
                    }
                }
                BlockType::Xnor => {
                    block.new_state = false;
                    let mut count = 0;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            count += 1;
                        }
                    }
                    if count % 2 == 0 {
                        block.new_state = true;
                    }
                }
                BlockType::Button => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Nor => {
                    block.new_state = true;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = false;
                            break;
                        }
                    }
                }
                BlockType::Tile { .. } => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Led { .. } => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Ledmixer { .. } => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Conductor => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::ConductorV2 => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Text { .. } => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                BlockType::Nand => {
                    let mut state = true;
                    for input in &block.inputs {
                        state = !(self.states[*input as usize - 1] && state);
                    }
                    block.new_state = state;
                }
                BlockType::Sound { .. } => {
                    block.new_state = false;
                    for input in &block.inputs {
                        if self.states[*input as usize - 1] == true {
                            block.new_state = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}