#[derive(PartialEq)]
pub enum RtlAdderType {
    KSA,
    RCA,
}

#[derive(PartialEq)]
pub enum RtlFlopType {
    XOR,
    TFF,
}

pub const RTL_WIRE_TYPE: BlockType = BlockType::Node;
pub const RTL_CONST_ON_TYPE: BlockType = BlockType::Nor;
pub const RTL_CONST_OFF_TYPE: BlockType = BlockType::Node;

pub const RTL_ADDER_TYPE: RtlAdderType = RtlAdderType::KSA;
// only flipflops may have initial value
pub const RTL_FLOP_TYPE: RtlFlopType = RtlFlopType::TFF;

use std::mem::{Discriminant, discriminant};

pub const BLOCKS_IGNORED: [Discriminant<BlockType>; 1] =
    [discriminant(&BlockType::Text { symbol: 0 })];
pub const BLOCKS_DEFAULT_POS: [f32; 3] = [0.0, 0.0, 0.0];

use std::collections::HashMap;

use crate::*;

pub trait Bus {
    fn contents(&self) -> &Vec<Block>;
    fn width(&self) -> usize;
    fn slice(&self, start: usize, end: usize) -> Vec<Block>;
    fn connect_bitwise_lsb(&self, rhs: &dyn Bus);
    fn connect_bitwise_msb(&self, rhs: &dyn Bus);
    fn connect_logical(&self, rhs: &Block);
    fn concat(&self, rhs: &dyn Bus) -> Vec<Block>;
    fn fan_in(&self, rhs: &Block);
}

#[derive(Clone, Debug)]
pub struct Wire {
    pub contains: Vec<Block>,
    pub width: usize,
}

impl Wire {
    pub fn new(pos: [f32; 3], width: usize) -> Wire {
        let [x, y, z] = pos;
        let mut blocks: Vec<Block> = Vec::new();
        for b in 0..width {
            let block = Block::new([x + b as f32, y, z], RTL_WIRE_TYPE);
            blocks.push(block);
        }
        Wire {
            contains: blocks,
            width,
        }
    }
}

impl Bus for Wire {
    fn contents(&self) -> &Vec<Block> {
        &self.contains
    }
    fn width(&self) -> usize {
        self.width
    }
    fn connect_bitwise_lsb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let rhs_width = rhs.width();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[self.width - b - 1] >> rhs_contains[rhs_width - b - 1];
        }
    }
    fn connect_bitwise_msb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[b] >> rhs_contains[b];
        }
    }
    fn connect_logical(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block >> rhs;
        }
    }
    fn slice(&self, start: usize, end: usize) -> Vec<Block> {
        self.contains[start..end].to_vec()
    }
    fn concat(&self, rhs: &dyn Bus) -> Vec<Block> {
        let mut blocks = self.contains.clone();
        blocks.extend(rhs.contents().iter().cloned());
        blocks
    }
    fn fan_in(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block << rhs;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Const {
    pub contains: Vec<Block>,
    pub width: usize,
}

impl Const {
    pub fn new(pos: [f32; 3], width: usize, bitmask: &str) -> Self {
        let mut blocks: Vec<Block> = Vec::new();
        let [x, y, z] = pos;

        for en in bitmask.chars().enumerate() {
            let (idx, bit) = en;
            if idx >= width {
                break;
            }
            let block = match bit {
                '1' => Block::snew([x + idx as f32, y, z], RTL_CONST_ON_TYPE, true),
                '0' => Block::snew([x + idx as f32, y, z], RTL_CONST_OFF_TYPE, false),
                _ => panic!("Error creating Const object, string contained restricted characters"),
            };
            blocks.push(block);
        }

        Const {
            contains: blocks,
            width,
        }
    }
    pub fn new_u(pos: [f32; 3], width: usize, mask: Vec<u64>) -> Self {
        let [x, y, z] = pos;
        let mut blocks: Vec<Block> = Vec::new();

        let mut mask_string: String = String::new();
        mask.iter()
            .for_each(|v| mask_string.push_str(&format!("{:b}", v)));

        mask_string = format!("{:0>1$}", mask_string, width);

        for (dx, bit) in mask_string.chars().enumerate() {
            blocks.push(Block::snew(
                [x + dx as f32, y, z],
                if bit == '1' {
                    RTL_CONST_ON_TYPE
                } else {
                    RTL_CONST_OFF_TYPE
                },
                bit == '1',
            ))
        }
        Self {
            contains: blocks,
            width,
        }
    }
}

impl Bus for Const {
    fn width(&self) -> usize {
        self.width
    }
    fn contents(&self) -> &Vec<Block> {
        &self.contains
    }
    fn slice(&self, start: usize, end: usize) -> Vec<Block> {
        let slice = self.contains[start..end].to_vec();
        slice
    }
    fn concat(&self, rhs: &dyn Bus) -> Vec<Block> {
        let mut blocks = self.contains.clone();
        blocks.extend(rhs.contents().iter().cloned());
        blocks
    }
    fn connect_bitwise_lsb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let rhs_width = rhs.width();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[self.width - b - 1] >> rhs_contains[rhs_width - b - 1];
        }
    }
    fn connect_bitwise_msb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[b] >> rhs_contains[b];
        }
    }
    fn connect_logical(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block >> rhs;
        }
    }
    fn fan_in(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block << rhs;
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomBus {
    pub contains: Vec<Block>,
    pub width: usize,
}

impl CustomBus {
    pub fn new(pos: [f32; 3], width: usize, blocktype: BlockType) -> Self {
        let [x, y, z] = pos;
        let mut blocks: Vec<Block> = Vec::new();
        for b in 0..width {
            let block = Block::new([x + b as f32, y, z], blocktype);
            blocks.push(block);
        }
        CustomBus {
            contains: blocks,
            width,
        }
    }
    pub fn from_vec(vec: Vec<Block>) -> Self {
        let len = vec.len();
        Self {
            contains: vec,
            width: len,
        }
    }
    pub fn default() -> Self {
        Self {
            contains: Vec::new(),
            width: 0,
        }
    }
}

impl Bus for CustomBus {
    fn contents(&self) -> &Vec<Block> {
        &self.contains
    }
    fn width(&self) -> usize {
        self.width
    }
    fn connect_bitwise_lsb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let rhs_width = rhs.width();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[self.width - b - 1] >> rhs_contains[rhs_width - b - 1];
        }
    }
    fn connect_bitwise_msb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[b] >> rhs_contains[b];
        }
    }
    fn connect_logical(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block >> rhs;
        }
    }
    fn slice(&self, start: usize, end: usize) -> Vec<Block> {
        let self_slice = self.contains[start..end].to_vec();
        self_slice
    }
    fn concat(&self, rhs: &dyn Bus) -> Vec<Block> {
        let mut blocks = self.contains.clone();
        blocks.extend(rhs.contents().iter().cloned());
        blocks
    }
    fn fan_in(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block << rhs;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Reg {
    pub contains: Vec<Block>,
    pub width: usize,
}

impl Bus for Reg {
    fn contents(&self) -> &Vec<Block> {
        &self.contains
    }
    fn width(&self) -> usize {
        self.width
    }
    fn connect_bitwise_lsb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let rhs_width = rhs.width();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[self.width - b - 1] >> rhs_contains[rhs_width - b - 1];
        }
    }
    fn connect_bitwise_msb(&self, rhs: &dyn Bus) {
        let rhs_contains = rhs.contents();
        let len = std::cmp::min(self.contains.len(), rhs_contains.len());

        for b in 0..len {
            let _ = self.contains[b] >> rhs_contains[b];
        }
    }
    fn connect_logical(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block >> rhs;
        }
    }
    fn slice(&self, start: usize, end: usize) -> Vec<Block> {
        let self_slice = self.contains[start..end].to_vec();
        self_slice
    }
    fn concat(&self, rhs: &dyn Bus) -> Vec<Block> {
        let mut blocks = self.contains.clone();
        blocks.extend(rhs.contents().iter().cloned());
        blocks
    }
    fn fan_in(&self, rhs: &Block) {
        for block in &self.contains {
            let _ = block << rhs;
        }
    }
}

impl Reg {
    pub fn new(pos: [f32; 3], width: usize, mask: Vec<u64>) -> Reg {
        let mut blocks: Vec<Block> = Vec::new();
        let [x, y, z] = pos;

        if RTL_FLOP_TYPE == RtlFlopType::TFF {
            let mut mask_string: String = String::new();
            mask.iter()
                .for_each(|v| mask_string.push_str(&format!("{:b}", v)));

            mask_string = format!("{:0>1$}", mask_string, width);

            for (dx, bit) in mask_string.chars().enumerate() {
                blocks.push(Block::snew(
                    [x + dx as f32, y, z],
                    BlockType::FlipFlop,
                    bit == '1',
                ));
            }
        } else {
            let nodes = CustomBus::new([x, y, z - 1.0], width, BlockType::Node);
            let xors = CustomBus::new([x, y, z], width, BlockType::Xor);

            nodes.connect_bitwise_lsb(&xors);
            xors.connect_bitwise_lsb(&nodes);

            blocks = xors.contains;
        }

        Reg {
            contains: blocks,
            width: width,
        }
    }
    pub fn from_vec(vec: Vec<Block>) -> Self {
        let len = vec.len();
        Reg {
            contains: vec,
            width: len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RegIn {
    pub wdata: CustomBus,
    pub rdata: CustomBus,
}

impl RegIn {
    pub fn new(pos: [f32; 3], width: usize) -> Self {
        let [x, y, z] = pos;
        let wdata = CustomBus::new([x, y, z], width, BlockType::And);
        let rdata = CustomBus::new([x, y, z - 1.0], width, BlockType::Xor);
        let clk = Block::new([x + width as f32, y, z], BlockType::Node);
        wdata.fan_in(&clk);
        rdata.connect_bitwise_lsb(&wdata);
        Self { wdata, rdata }
    }
    pub fn new_driven(
        pos: [f32; 3],
        width: usize,
        drive: &dyn Bus,
        reg: &Reg,
        clk: &Block,
    ) -> Self {
        let [x, y, z] = pos;
        let wdata = CustomBus::new([x, y, z], width, BlockType::And);
        let rdata = CustomBus::new([x, y, z - 1.0], width, BlockType::Xor);
        wdata.fan_in(clk);
        rdata.connect_bitwise_lsb(&wdata);
        reg.connect_bitwise_lsb(&rdata);
        wdata.connect_bitwise_lsb(reg);
        drive.connect_bitwise_lsb(&rdata);
        Self { wdata, rdata }
    }
}

#[derive(Clone, Debug)]
pub struct Adder {
    pub width: usize,
    pub output: CustomBus,
    pub cout: Block,
    pub cin: Block,
}

impl Adder {
    pub fn new(
        pos: [f32; 3],
        width: usize,
        sub: bool,
        a: &dyn Bus,
        b: &dyn Bus,
        kind: RtlAdderType,
    ) -> Self {
        match kind {
            RtlAdderType::KSA => Self::new_ksa(pos, width, sub, a, b),
            RtlAdderType::RCA => Self::new_rca(pos, width, sub, a, b),
        }
    }
    pub fn new_ksa(pos: [f32; 3], width: usize, sub: bool, a: &dyn Bus, b: &dyn Bus) -> Self {
        let [x, y, mut z] = pos;

        let b_new = CustomBus::new(
            [x, y, z],
            width,
            if sub { BlockType::Nor } else { BlockType::Node },
        );
        b >> &b_new;
        let b = b_new;

        let mut prevg = CustomBus::new([x, y, z - 1.0], width, BlockType::And);
        let mut prevp = CustomBus::new([x, y, z - 2.0], width, BlockType::Xor);
        let p = prevp.clone();

        a.connect_bitwise_msb(&prevg);
        a.connect_bitwise_msb(&prevp);
        b.connect_bitwise_msb(&prevp);
        b.connect_bitwise_msb(&prevg);

        let levels = (width as f32).log2().ceil() as u32;
        let mut free = 1usize;
        z -= 3.0;

        for _ in 0..levels {
            let logic = width - free;

            let newp = CustomBus::new([x, y, z], logic, BlockType::And);
            let newfp = CustomBus::new([x + logic as f32, y, z], free, BlockType::Node);
            let tmpp = CustomBus::from_vec(newp.concat(&newfp));
            prevp.connect_bitwise_msb(&tmpp);
            CustomBus::from_vec(prevp.slice(free as usize, width as usize))
                .connect_bitwise_msb(&newp);

            let newgand = CustomBus::new([x, y, z - 1.0], logic, BlockType::And);
            let newgor = CustomBus::new([x, y + 1.0, z - 1.0], logic, BlockType::Node);
            let newfg = CustomBus::new([x + logic as f32, y + 1.0, z - 1.0], free, BlockType::Node);
            prevp.connect_bitwise_msb(&newgand);
            CustomBus::from_vec(prevg.slice(free as usize, width as usize))
                .connect_bitwise_msb(&newgand);
            let tmpg = CustomBus::from_vec(newgor.concat(&newfg));
            prevg.connect_bitwise_msb(&tmpg);
            newgand.connect_bitwise_msb(&newgor);

            prevg = tmpg;
            prevp = tmpp;
            z -= 2.0;
            free *= 2;
        }

        let carryand = CustomBus::new([x, y, z], width, BlockType::And);
        let carryor = CustomBus::new([x, y + 1.0, z], width, BlockType::Node);
        carryand.connect_bitwise_msb(&carryor);

        let cout = Block::new(
            [x + width as f32, y, z],
            if sub { BlockType::Nor } else { BlockType::Node },
        );
        carryand.fan_in(&cout);
        prevp.connect_bitwise_msb(&carryand);
        prevg.connect_bitwise_msb(&carryor);

        let out = CustomBus::new([x, y, z - 2.0], width, BlockType::Xor);
        p.connect_bitwise_msb(&out);
        CustomBus::from_vec(carryor.slice(1, width as usize)).connect_bitwise_msb(&out);
        cout.connect(&out.contains[width - 1]);

        Adder {
            width,
            output: out,
            cout: carryor.contains[0],
            cin: cout,
        }
    }
    #[allow(unused)]
    pub fn new_rca(pos: [f32; 3], width: usize, sub: bool, a: &dyn Bus, b: &dyn Bus) -> Self {
        let make_tile = |pos: [f32; 3], cin: &Block, a: &Block, b: &Block| -> (Block, Block) {
            let [x, y, z] = pos;
            let xor1 = Block::new([x, y, z], BlockType::Xor);
            let and1 = Block::new([x - 1.0, y, z], BlockType::And);
            let xor2 = Block::new([x, y, z - 1.0], BlockType::Xor);
            let and2 = Block::new([x - 1.0, y, z - 1.0], BlockType::And);
            let or = Block::new([x - 1.0, y + 1.0, z - 1.0], BlockType::Node);
            a >> &xor1;
            b >> &xor1;
            a >> &and1;
            b >> &and2;
            xor1 >> and2;
            xor1 >> xor2;
            cin >> &xor2;
            cin >> &and2;
            and2 >> or;
            and1 >> or;
            (or, xor2)
        };

        let [mut x, mut y, mut z] = pos;
        let real_carry_in = Block::new([x, y + 1.0, z], BlockType::Node);
        let mut carry_in = real_carry_in.clone();

        let b = CustomBus::new(
            [x, y, z - 1.0],
            width,
            if sub { BlockType::Xor } else { BlockType::Node },
        );
        let out = CustomBus::new([x, y, z - 2.0], width, BlockType::Node);
        let mut cout = Block::inew(0, [0.0, 0.0, 0.0], BlockType::And);
        z -= 3.0;

        for i in (0..(width - 1)).rev() {
            let adder = make_tile([x, y, z], &carry_in, &a.slice(0, 1)[i], &b.slice(0, 1)[i]);
            carry_in = adder.0;
            &adder.1 >> &out.contains[i];
            x -= 2.0;

            if width == 0 {
                cout = adder.0;
            }
        }

        Adder {
            width,
            output: out,
            cout,
            cin: real_carry_in,
        }
    }
    pub fn new_incrementer(pos: [f32; 3], width: usize, a: &dyn Bus) -> CustomBus {
        let [x, y, z] = pos;
        let carries = CustomBus::new([x, y, z], width - 1, BlockType::And);
        let not = Block::new([x + width as f32 - 1.0, y, z - 1.0], BlockType::Nor);
        let mut xors = CustomBus::new([x, y, z - 1.0], width - 1, BlockType::Xor);
        carries.connect_bitwise_msb(&xors);
        a.connect_bitwise_msb(&xors);
        let a_local = a.slice(0, a.width());
        let _ = &a_local[width - 1] >> &not;
        for (idx, ..) in a_local.iter().enumerate() {
            if idx < carries.contains.len() {
                let slice = CustomBus::from_vec(a_local[idx + 1..].to_vec());
                slice.connect_logical(&carries.contains[idx]);
            }
        }
        xors.contains.push(not);
        xors.width += 1;
        xors
    }
}

#[derive(Debug, Clone)]
pub struct Comparison {
    pub output: Block,
    output_vec: Vec<Block>,
}

impl std::ops::Shr for &dyn Bus {
    type Output = ();

    fn shr(self, rhs: Self) -> Self::Output {
        self.connect_bitwise_lsb(rhs);
    }
}

impl std::ops::Shr for &CustomBus {
    type Output = ();

    fn shr(self, rhs: Self) -> Self::Output {
        self.connect_bitwise_lsb(rhs);
    }
}

impl Comparison {
    pub fn new_gt(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let not_b = CustomBus::new([x, y, z], width, BlockType::Nor);
        let and_ab = CustomBus::new([x, y, z - 1.0], width, BlockType::And);
        rhs >> &not_b;
        lhs >> &and_ab;
        &not_b >> &and_ab;
        let xor_ab = CustomBus::new([x, y, z - 2.0], width, BlockType::Xor);
        lhs >> &xor_ab;
        rhs >> &xor_ab;
        let nor_xor = CustomBus::new([x + 1.0, y, z - 3.0], width - 1, BlockType::Nor);
        &nor_xor >> &and_ab;
        let output = Block::new([x, y, z - 3.0], BlockType::Node);
        let output_vec = vec![output.clone()];
        and_ab.connect_logical(&output);

        for (idx, block) in xor_ab.contains.iter().enumerate() {
            CustomBus::from_vec(nor_xor.slice(idx, width - 1)).fan_in(block);
        }

        Self { output, output_vec }
    }
    pub fn new_eq(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let xnors = CustomBus::new(pos, width, BlockType::Xnor);
        let output = Block::new([x + width as f32 - 1.0, y + 1.0, z], BlockType::And);
        let output_vec = vec![output.clone()];
        xnors.connect_logical(&output);
        lhs >> &xnors;
        rhs >> &xnors;
        Self { output, output_vec }
    }
    pub fn new_neq(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let xnors = CustomBus::new(pos, width, BlockType::Xnor);
        let output = Block::new([x, y + 1.0, z], BlockType::Nand);
        let output_vec = vec![output.clone()];
        xnors.connect_logical(&output);
        lhs >> &xnors;
        rhs >> &xnors;
        Self { output, output_vec }
    }
    pub fn new_lt(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        Self::new_gt(pos, width, rhs, lhs)
    }
    pub fn new_ge(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let gt = Self::new_gt([x, y, z - 1.0], width, lhs, rhs);
        let xnors = CustomBus::new(pos, width, BlockType::Xnor);
        lhs >> &xnors;
        rhs >> &xnors;
        let and = Block::new([x, y + 1.0, z], BlockType::And);
        xnors.connect_logical(&and);
        let _ = and >> gt.output;
        gt
    }
    pub fn new_le(pos: [f32; 3], width: usize, lhs: &dyn Bus, rhs: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let lt = Self::new_lt([x, y, z - 1.0], width, lhs, rhs);
        let xnors = CustomBus::new(pos, width, BlockType::Xnor);
        lhs >> &xnors;
        rhs >> &xnors;
        let and = Block::new([x, y + 1.0, z], BlockType::And);
        xnors.connect_logical(&and);
        let _ = and >> lt.output;
        lt
    }
    pub fn output_vec(&self) -> &Vec<Block> {
        &self.output_vec
    }
}

#[derive(Debug, Clone)]
pub enum EdgeKind {
    Rising,
    Falling,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub output: Block,
    output_vec: Vec<Block>,
}

impl Edge {
    pub fn new_rising(pos: [f32; 3], input: &Block) -> Self {
        let [x, y, z] = pos;
        let not = Block::new([x, y, z], BlockType::Nor);
        let and = Block::new([x, y, z - 1.0], BlockType::And);
        let output_vec = vec![and.clone()];
        let _ = not >> and;
        let _ = input >> &not;
        let _ = input >> &and;
        Edge {
            output: and,
            output_vec,
        }
    }
    pub fn new_falling(pos: [f32; 3], input: &Block) -> Self {
        let [x, y, z] = pos;
        let n_in = Block::new([x, y, z], BlockType::Nor);
        let nn_in = Block::new([x, y, z - 1.0], BlockType::Nor);
        let and = Block::new([x, y, z - 2.0], BlockType::And);
        let output_vec = vec![and.clone()];
        let _ = input >> &n_in;
        let _ = n_in >> nn_in;
        let _ = nn_in >> and;
        let _ = n_in >> and;
        Self {
            output: and,
            output_vec,
        }
    }
    pub fn new(pos: [f32; 3], input: &Block, kind: EdgeKind) -> Self {
        match kind {
            EdgeKind::Falling => Self::new_falling(pos, input),
            EdgeKind::Rising => Self::new_rising(pos, input),
        }
    }
    pub fn output_vec(&self) -> &Vec<Block> {
        &self.output_vec
    }
}

#[derive(Debug, Clone)]
pub struct Neg {
    pub output: CustomBus,
}

impl Neg {
    pub fn new(pos: [f32; 3], width: usize, operand: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let not = CustomBus::new([x, y, z], width, BlockType::Nor);
        operand >> &not;
        let adder = Adder::new_incrementer([x, y, z - 1.0], width, &not);
        Self { output: adder }
    }
}

pub fn reorder_blocks(x: u32, z: u32, y: u32, mut blocks: Vec<Block>) -> Vec<Block> {
    for (idx, block) in blocks.iter_mut().enumerate() {
        let idx = idx as u32;

        let yi = idx / (x * z);
        let rem = idx % (x * z);
        let xi = rem / z;
        let zi = rem % z;

        if yi >= y {
            break;
        }

        if !BLOCKS_IGNORED.contains(&discriminant(&block.blocktype)) {
            block.pos[0] = xi as f32;
            block.pos[1] = yi as f32;
            block.pos[2] = zi as f32;
        }
    }

    blocks
}

/// xxotic`s idea
pub fn reorder_blocks_sandwichify(z_limit: Option<usize>, blocks: Vec<Block>) -> Vec<Block> {
    let mut added: HashMap<u8, u8> = HashMap::new();
    let mut blocks_hash: HashMap<u8, Vec<Block>> = HashMap::new();

    let mut new_blocks: Vec<Block> = Vec::new();

    for block in blocks {
        if !BLOCKS_IGNORED.contains(&discriminant(&block.blocktype)) {
            let added_len = added.len() as u8;
            let idx = *added.entry(block.blocktype.as_u8()).or_insert(added_len);
            blocks_hash.entry(idx).or_insert(Vec::new()).push(block);
        } else {
            new_blocks.push(block);
        }
    }

    let mut dx: u8 = 0;
    let mut dz: usize = 0;

    for (_, group) in blocks_hash.iter() {
        for &block in group {
            let mut block = block.clone();
            block.pos = [dx as f32, 0.0, dz as f32];
            new_blocks.push(block);

            if let Some(limit) = z_limit {
                if dz >= limit - 1 {
                    dz = 0;
                    dx += 1;
                } else {
                    dz += 1;
                }
            } else {
                dz += 1;
            }
        }
    }

    new_blocks
}

#[derive(Debug, Clone)]
pub struct Shifter {
    pub output: CustomBus,
    pub width: usize,
}

impl Shifter {
    pub fn new_constant_left(lhs: &dyn Bus, rhs: u16, pos: [f32; 3]) -> Self {
        let width = lhs.width();

        let output = CustomBus::new(pos, width, RTL_WIRE_TYPE);
        let output_slice = CustomBus::from_vec(output.slice(0, width - rhs as usize));
        lhs.connect_bitwise_lsb(&output_slice);

        Self { output, width }
    }
    pub fn new_constant_right(lhs: &dyn Bus, rhs: u16, pos: [f32; 3], arithmetic: bool) -> Self {
        let width = lhs.width();
        let [x, y, z] = pos;

        let output = CustomBus::new(pos, width, RTL_WIRE_TYPE);
        let output_slice = CustomBus::from_vec(output.slice(rhs as usize, width));
        lhs.connect_bitwise_msb(&output_slice);

        if arithmetic {
            let sign_extend = SignExtend::new([x, y, z - 1.0], &output);
            Self {
                output: sign_extend.output,
                width,
            }
        } else {
            Self { output, width }
        }
    }
    #[allow(unused)]
    pub fn new_multiplier_left(lhs: &dyn Bus, rhs: &dyn Bus, pos: [f32; 3]) {
        // 16 bit logic
        let width = lhs.width();
        let mul16_count = width / 16;
        let leftover_bits = width % 16;

        let output = CustomBus::new(pos, width, BlockType::Node);

        let rhs_bus =
            CustomBus::from_vec(rhs.slice(rhs.width() - 1 - width.ilog2() as usize, rhs.width()));

        let lookup = LookUpDecoder::new_all_bits(rhs.width(), pos, Some(rhs));
        let b_ready: Vec<Block> = lookup.higher_to_lower_vec().iter().map(|b| **b).collect();
        let b_ready: [Option<Vec<(BPortD, u32)>>; 16] =
            AdvancedBuildings::block_vec_to_connectible(b_ready, BPortD::Input)
                .try_into()
                .unwrap();

        for mul in (0..mul16_count).rev() {
            let a = lhs.slice(mul * 16, mul * 16 + 16);
            let a = AdvancedBuildings::block_vec_to_connectible(a, BPortD::Input);
            let outputv = output.slice(mul * 16, mul * 16 + 16);
            let upper: [Option<Vec<(BPortD, u32)>>; 16] = if mul > 0 {
                AdvancedBuildings::block_vec_to_connectible(
                    output.slice(mul * 16 - 16, mul * 16),
                    BPortD::Output,
                )
                .try_into()
                .unwrap()
            } else {
                [const { None }; 16]
            };
            let outputv = AdvancedBuildings::block_vec_to_connectible(outputv, BPortD::Output);

            let multiplier = AdvancedBuildings::Multiplier(
                a.try_into().unwrap(),
                b_ready.clone(),
                upper,
                outputv.try_into().unwrap(),
                pos,
                AdvancedBuildings::DEFAULT_LAY_Z_ROT,
            );
            SAVE.lock().unwrap().buildings.push(multiplier)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mux {
    pub output: CustomBus,
    pub width: usize,
}

impl Mux {
    pub fn new(
        pos: [f32; 3],
        width: usize,
        truehs: &dyn Bus,
        falsehs: &dyn Bus,
        sel: &Block,
    ) -> Self {
        let [x, y, z] = pos;
        let true_and = CustomBus::new([x, y, z], width, BlockType::And);
        let false_and = CustomBus::new([x, y, z - 1.0], width, BlockType::And);
        let sel_delayed = Block::new([x, y + 1.0, z], BlockType::Or);
        let sel_n = Block::new([x, y + 1.0, z - 1.0], BlockType::Nor);
        let _ = sel >> &sel_delayed;
        let _ = sel >> &sel_n;
        false_and.fan_in(&sel_n);
        true_and.fan_in(&sel_delayed);
        let output = CustomBus::new([x, y, z - 2.0], width, BlockType::Node);
        &false_and >> &output;
        &true_and >> &output;

        truehs >> &true_and;
        falsehs >> &false_and;

        Self { output, width }
    }
}

#[derive(Debug, Clone)]
pub struct Replicate {
    pub output: CustomBus,
}

impl Replicate {
    pub fn new(pos: [f32; 3], src: &dyn Bus, times: u16) -> Self {
        let width = src.width();
        let [mut x, y, z] = pos;
        let mut buses: Vec<Block> = Vec::new();

        for _ in 0..times {
            let bus = CustomBus::new([x, y, z], width, BlockType::Node);
            src >> &bus;
            buses.extend(bus.contains);
            x += width as f32;
        }

        Self {
            output: CustomBus::from_vec(buses),
        }
    }
}

pub fn generate_io(pos: [f32; 3], name: &str, width: usize) -> CustomBus {
    let [x, y, z] = pos;
    let mut blocks: Vec<Block> = Vec::new();

    for (dx, symbol) in name.chars().enumerate() {
        blocks.push(Block::new(
            [x + dx as f32, y, z],
            BlockType::Text {
                symbol: symbol as u8,
            },
        ))
    }

    if name.len() < width {
        for dx in (name.len())..width {
            blocks.push(Block::new(
                [x + dx as f32, y, z],
                BlockType::Text { symbol: b' ' },
            ))
        }
    }

    CustomBus::from_vec(blocks)
}

#[derive(Debug, Clone)]
pub struct Clock {
    pub output: Block,
    pub timing: u16,
    output_vec: Vec<Block>,
}

impl Clock {
    pub fn new(pos: [f32; 3], timing: u16) -> Self {
        let clock = Block::snew(pos, BlockType::Delay { ticks: timing }, true);
        let _ = clock >> clock;
        let output_vec: Vec<Block> = vec![clock.clone()];

        Self {
            output: clock,
            timing,
            output_vec,
        }
    }
    pub fn output_vec(&self) -> &Vec<Block> {
        &self.output_vec
    }
}

#[allow(unused)]
pub struct SignExtend {
    output: CustomBus,
    width: usize,
}

impl SignExtend {
    pub fn new(pos: [f32; 3], src: &dyn Bus) -> Self {
        let [x, y, z] = pos;
        let inv = CustomBus::new(pos, src.width(), BlockType::Nor);
        let ands = CustomBus::new([x + 1.0, y, z - 1.0], src.width() - 1, BlockType::And);
        let ors = CustomBus::new([x, y, z - 2.0], src.width(), BlockType::Node);
        src >> &ors;
        src >> &inv;
        src >> &ands;

        for (idx, block) in inv.contains.iter().enumerate() {
            let and_bus = CustomBus::from_vec(ands.contains[idx..].to_vec());
            and_bus.fan_in(block);
        }

        for (idx, block) in ands.contains.iter().enumerate() {
            let or_bus = CustomBus::from_vec(ors.contains[0..=idx].to_vec());
            or_bus.fan_in(block);
        }

        Self {
            output: ors,
            width: src.width(),
        }
    }
}

#[allow(unused)]
pub struct LookUpDecoder {
    output: HashMap<Vec<u64>, Block>,
    input: Option<CustomBus>,
}

impl LookUpDecoder {
    pub fn new_all_bits(width: usize, pos: [f32; 3], src: Option<&dyn Bus>) -> Self {
        let [x, y, mut z] = pos;
        let ors = CustomBus::new(pos, width, BlockType::Or);
        let nors = CustomBus::new([x, y, z - 1.0], width, BlockType::Nor);
        let mut map: HashMap<Vec<u64>, Block> = HashMap::new();

        let inputv;

        if let Some(bus) = src {
            bus >> &ors;
            bus >> &nors;
            inputv = None;
        } else {
            let input = CustomBus::new([x, y + 1.0, z], width, BlockType::Node);
            &input >> &ors;
            &input >> &nors;
            inputv = Some(input)
        }

        z -= 2.0;

        for n in 0..2u32.pow(width as u32) {
            let mask = format!("{:01$b}", n, width);

            let and = Block::new([x, y, z], BlockType::And);
            z -= 1.0;

            for (idx, bit) in mask.chars().enumerate() {
                if bit == '1' {
                    let _ = &ors.contains[idx] >> &and;
                } else {
                    let _ = &nors.contains[idx] >> &and;
                }
            }

            map.insert(vec![n as u64], and);
        }

        Self {
            output: map,
            input: inputv,
        }
    }
    pub fn new_selective(
        width: usize,
        pos: [f32; 3],
        src: Option<&dyn Bus>,
        values: Vec<Vec<u64>>,
    ) -> Self {
        let [x, y, mut z] = pos;
        let ors = CustomBus::new(pos, width, BlockType::Or);
        let nors = CustomBus::new([x, y, z - 1.0], width, BlockType::Nor);
        let mut map: HashMap<Vec<u64>, Block> = HashMap::new();

        let inputv;

        if let Some(bus) = src {
            bus >> &ors;
            bus >> &nors;
            inputv = None;
        } else {
            let input = CustomBus::new([x, y + 1.0, z], width, BlockType::Node);
            &input >> &ors;
            &input >> &nors;
            inputv = Some(input)
        }

        z -= 2.0;

        for n in values {
            let mut mask_string: String = String::new();
            n.iter()
                .for_each(|v| mask_string.push_str(&format!("{:b}", v)));
            mask_string = format!("{:0>1$}", mask_string, width);

            let and = Block::new([x, y, z], BlockType::And);
            z -= 1.0;

            for (idx, bit) in mask_string.chars().enumerate() {
                if bit == '1' {
                    let _ = &ors.contains[idx] >> &and;
                } else {
                    let _ = &nors.contains[idx] >> &and;
                }
            }

            map.insert(n, and);
        }

        Self {
            output: map,
            input: inputv,
        }
    }
    pub fn higher_to_lower_vec(&self) -> Vec<&Block> {
        let mut v: Vec<&Block> = Vec::new();

        let mut keys: Vec<&Vec<u64>> = self.output.keys().collect();
        keys.sort();
        keys.reverse();

        for key in keys {
            v.push(&self.output[key])
        }

        v
    }
}
