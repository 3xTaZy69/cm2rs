use rand;
use crate::{*, sim::emulator::*};

impl Emulator {
    pub fn calculate_cpu(&mut self) {
        for block in &mut self.cpu_blocks {
            match block.blocktype {
                BlockType::FlipFlop => {
                    let mut count = 0;
                    for input in &block.inputs {
                        if self.states[*input] {
                            count += 1;
                        }
                    }
                    if count > block.previnputs {
                        block.new_state = !self.states[block.id];
                    }
                    block.previnputs = count;
                }
                BlockType::Delay { .. } => {
                    block.new_state = false;
                    block.delay.retain_mut(|i| {
                        *i -= 1;
                        if *i == 0 {
                            block.new_state = true;
                        }
                        *i > 0
                    });
                }
                BlockType::Antenna { channel, .. } => {
                    // local emulator so contexts don`t matter
                    block.new_state = *self.channels.get(&channel).unwrap();
                    *self.channels.get_mut(&channel).unwrap() = false;
                    for input in &block.inputs {
                        if self.states[*input] {
                            block.new_state = true;
                            *self.channels.get_mut(&channel).unwrap() = true;
                            break;
                        }
                    }
                }
                BlockType::Node => {
                    for input in &block.inputs {
                        self.states[block.id] = false;
                        if self.states[*input] {
                            self.states[block.id] = true;
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
    let states = self.states.as_slice();

    for block in &mut self.gpu_blocks {
        match block.blocktype {
            BlockType::And => {
                block.new_state = block
                    .inputs
                    .iter()
                    .all(|&i| unsafe { *states.get_unchecked(i) });
            }
            BlockType::Nand => {
                block.new_state = !block
                    .inputs
                    .iter()
                    .all(|&i| unsafe { *states.get_unchecked(i) });
            }
            BlockType::Or
            | BlockType::Button
            | BlockType::Tile { .. }
            | BlockType::Led { .. }
            | BlockType::Ledmixer { .. }
            | BlockType::Conductor
            | BlockType::ConductorV2
            | BlockType::Text { .. }
            | BlockType::Sound { .. } => {
                block.new_state = block
                    .inputs
                    .iter()
                    .any(|&i| unsafe { *states.get_unchecked(i) });
            }
            BlockType::Nor => {
                block.new_state = !block
                    .inputs
                    .iter()
                    .any(|&i| unsafe { *states.get_unchecked(i) });
            }
            BlockType::Xor => {
                block.new_state = block
                    .inputs
                    .iter()
                    .fold(false, |acc, &i| acc ^ unsafe { *states.get_unchecked(i) });
            }
            BlockType::Xnor => {
                block.new_state = !block
                    .inputs
                    .iter()
                    .fold(false, |acc, &i| acc ^ unsafe { *states.get_unchecked(i) });
            }
            _ => {}
        }
    }
    }
}