use crate::{Block, BlockType, Connection};

#[derive(Clone)]
pub struct Bus {
    width: usize,
    contains: Vec<Block>
}

impl Bus {
    pub fn new(width: usize, pos: [f32; 3], blocktype: BlockType) -> Bus {
        let [x, y, z] = pos;
        let mut contains: Vec<Block> = Vec::new();
        for b in 0..width {
            contains.push(Block::new([x+b as f32, y, z], blocktype));
        }
        Bus { width, contains }
    }
    pub fn connect_bitwise(&self, other: &Bus) {
        for b in 0..std::cmp::min(self.contains.len(), other.contains.len()) {
            self.contains[b].connect(&other.contains[b]);
        }
    }
    pub fn connect_bitwise_lsb(&self, other: &Bus) {
        for b in 0..std::cmp::min(self.contains.len(), other.contains.len()) {
            self.contains[self.width - b - 1].connect(&other.contains[other.width - b - 1]);
        }
    }
    pub fn slice(&self, start: usize, end: usize) -> Bus {
        let blocks = self.contains[start..end].to_vec();
        Bus {width: blocks.len(), contains: blocks}
    }
    pub fn concat(&self, other: &Bus) -> Bus {
        let mut contains = self.contains.clone();
        for block in other.contains.clone() {
            contains.push(block);
        }
        Bus { width: contains.len(), contains }
    }
    pub fn connect_from_one(&self, other: &Block) {
        for b in 0..self.width {
            other.connect(&self.contains[b]);
        }
    }
}


pub struct Adder {
    pub width: usize,
    pub output: Bus,
    pub cout: Block,
    pub a: Bus,
    pub b: Bus,
}

impl Adder {
    pub fn new(pos: [f32; 3], width: usize, sub: bool) -> Self {
        let [x, y, mut z] = pos;

        let a = Bus::new(width, [x, y, z-1.0], BlockType::Node);
        let b = Bus::new(width, [x, y, z],   if sub { BlockType::Nor } else { BlockType::Node });

        let mut prevg = Bus::new(width, [x, y, z-2.0], BlockType::And);
        let mut prevp = Bus::new(width, [x, y, z-3.0], BlockType::Xor);
        let p = prevp.clone();

        a.connect_bitwise(&prevg);
        a.connect_bitwise(&prevp);
        b.connect_bitwise(&prevp);
        b.connect_bitwise(&prevg);

        let levels = (width as f32).log2().ceil() as u32;
        let mut free = 1usize;
        z -= 4.0;

        for _ in 0..levels {
            let logic = width - free;

            let newp    = Bus::new(logic, [x, y, z],     BlockType::And);
            let newfp   = Bus::new(free,  [x + logic as f32, y, z], BlockType::Node);
            let tmpp    = newp.concat(&newfp);
            prevp.connect_bitwise(&tmpp);
            prevp.slice(free as usize, width as usize).connect_bitwise(&newp);

            let newgand = Bus::new(logic, [x, y, z-1.0],   BlockType::And);
            let newgor  = Bus::new(logic, [x, y+1.0, z-1.0], BlockType::Node);
            let newfg   = Bus::new(free,  [x + logic as f32, y+1.0, z-1.0], BlockType::Node);
            prevp.connect_bitwise(&newgand);
            prevg.slice(free as usize, width as usize).connect_bitwise(&newgand);
            let tmpg = newgor.concat(&newfg);
            prevg.connect_bitwise(&tmpg);
            newgand.connect_bitwise(&newgor);

            prevg = tmpg;
            prevp = tmpp;
            z -= 2.0;
            free *= 2;
        }

        let carryand = Bus::new(width, [x, y, z],   BlockType::And);
        let carryor  = Bus::new(width, [x, y+1.0, z], BlockType::Node);
        carryand.connect_bitwise(&carryor);

        let cout = Block::new(
            [x + width as f32, y, z],
            if sub { BlockType::Nor } else { BlockType::Node }
        );
        carryand.connect_from_one(&cout);
        prevp.connect_bitwise(&carryand);
        prevg.connect_bitwise(&carryor);

        let out = Bus::new(width, [x, y, z-2.0], BlockType::Xor);
        p.connect_bitwise(&out);
        carryor.slice(1, width as usize).connect_bitwise(&out);
        cout.connect(&out.contains[width - 1]);

        Adder { width, output: out, cout: carryor.contains[0], a, b }
    }
}