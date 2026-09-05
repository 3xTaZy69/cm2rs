use crate::parser;
use cm2rs::*;
use core::slice;
use std::{collections::HashMap, ops::Add};

// flexible structure
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sid {
    Signed(usize),
    Unsigned(usize),
}
pub type Swidth = usize;
pub type Svalue = Vec<u64>;

#[derive(Debug, Clone)]
pub enum BGate {
    And,
    Or,
    Xor,
    Not,
}

impl BGate {
    pub fn as_blocktype(&self) -> BlockType {
        match self {
            BGate::And => BlockType::And,
            BGate::Or => BlockType::Node,
            BGate::Xor => BlockType::Xor,
            BGate::Not => BlockType::Nor,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LGate {
    And,
    Or,
    Not,
    Xor,
}

impl LGate {
    pub fn as_blocktype(&self) -> BlockType {
        match self {
            LGate::And => BlockType::And,
            LGate::Or => BlockType::Node,
            LGate::Not => BlockType::Nor,
            LGate::Xor => BlockType::Xor,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComparisonKind {
    Lt,
    Le,
    Eq,
    Neq,
    Gt,
    Ge,
}

pub enum SSA {
    // variables
    Const {
        id: Sid,
        width: Swidth,
        value: Svalue,
    }, //? done
    Reg {
        id: Sid,
        width: Swidth,
        init_value: Svalue,
    }, //? done
    Wire {
        id: Sid,
        width: Swidth,
        src: Sid,
    }, //? done
    Mem {
        id: Sid,
        width: Swidth,
        length: Swidth,
    },

    // memory and register io
    RegIn {
        id: Sid,
        data: Sid,
        reg: Sid,
        en: Sid,
    }, //? done
    MemWrite {
        id: Sid,
        addr: Sid,
        mem: Sid,
        data: Sid,
        en: Sid,
    },
    MemRead {
        id: Sid,
        addr: Sid,
        mem: Sid,
        en: Sid,
    },

    // arithmetical operations
    Add {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
    }, //? done
    Sub {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
    }, //? done
    // mul, div, mod are limited to 32 bits
    Mul {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
    },
    Div {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
    },
    Mod {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
    },
    Neg {
        id: Sid,
        src: Sid,
        width: Swidth,
    }, //? done

    // logical operations
    // generated with CustomBus
    Bitwise {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
        gate: BGate,
    }, //? done
    Logical {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
        gate: LGate,
    }, //? done

    // only constant shifts
    ConstShl {
        id: Sid,
        lhs: Sid,
        rhs: Svalue,
        width: Swidth,
    }, //? done
    ConstShr {
        id: Sid,
        lhs: Sid,
        rhs: Svalue,
        width: Swidth,
        arithmetic: bool,
    }, //? done

    // comparison
    Comparison {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
        width: Swidth,
        kind: ComparisonKind,
    }, //? done

    // bit operations
    // option for signals where width will be known only after processing
    Replicate {
        id: Sid,
        times: Swidth,
        src: Sid,
    }, //? done
    Concat {
        id: Sid,
        lhs: Sid,
        rhs: Sid,
    }, //? done
    // only constant slice
    Slice {
        id: Sid,
        src: Sid,
        start: Swidth,
        end: Swidth,
    }, //? done

    // mux
    Mux {
        id: Sid,
        width: Swidth,
        true_value: Sid,
        false_value: Sid,
        sel: Sid,
    }, //? done

    // edges
    PosEdge {
        id: Sid,
        src: Sid,
    }, //? done
    NegEdge {
        id: Sid,
        src: Sid,
    }, //? done

    // io
    GenerateInput {
        id: Sid,
        pos: [f32; 3],
        name: String,
        width: usize,
    }, //? done
    GenerateOutput {
        src: Sid,
        pos: [f32; 3],
        name: String,
        width: usize,
    }, //? done
    GenerateClock {
        id: Sid,
        timing: u16,
    }, //? done
}

use crate::*;
use cm2rs::rtl::*;

#[derive(Debug, Clone)]
pub enum RtlKind {
    Const(Const),
    Reg(Reg),
    Wire(Wire),
    Mem, //? not realized
    RegIn(RegIn),
    MemWrite, //? not realized
    MemRead,  //? not realized
    Add(Adder),
    Sub(Adder),
    Mul, //? not realized
    Div, //? not realized
    Mod, //? not realized
    Neg(Neg),
    Bitwise(CustomBus),
    Logical(Vec<Block>), // one block
    ConstShl(Shifter),
    ConstShr(Shifter),
    Comparison(Comparison),
    Replicate(Replicate),
    Concat(CustomBus),
    Slice(CustomBus),
    Mux(Mux),
    Edge(Edge),
    Clock(Clock),
}

impl RtlKind {
    pub fn get_contents(&self) -> &Vec<Block> {
        match self {
            RtlKind::Const(v) => &v.contains,
            RtlKind::Wire(v) => &v.contains,
            RtlKind::Reg(v) => &v.contains,
            RtlKind::Mem => todo!(), //? not realized
            RtlKind::RegIn(v) => &v.rdata.contains,
            RtlKind::MemWrite => todo!(), //? not realized
            RtlKind::MemRead => todo!(),  //? not realized
            RtlKind::Add(v) => &v.output.contains,
            RtlKind::Sub(v) => &v.output.contains,
            RtlKind::Mul => todo!(), //? not realized
            RtlKind::Div => todo!(), //? not realized
            RtlKind::Mod => todo!(), //? not realized
            RtlKind::Neg(v) => &v.output.contains,
            RtlKind::Bitwise(v) => &v.contains,
            RtlKind::Logical(v) => &v,
            RtlKind::ConstShl(v) => &v.output.contains,
            RtlKind::ConstShr(v) => &v.output.contains,
            RtlKind::Comparison(v) => v.output_vec(),
            RtlKind::Replicate(v) => &v.output.contains,
            RtlKind::Concat(v) => &v.contains,
            RtlKind::Slice(v) => &v.contains,
            RtlKind::Mux(v) => &v.output.contains,
            RtlKind::Edge(v) => v.output_vec(),
            RtlKind::Clock(v) => v.output_vec(),
        }
    }
    pub fn get_block(&self) -> &Block {
        match self {
            RtlKind::Comparison(v) => &v.output,
            RtlKind::Edge(v) => &v.output,
            RtlKind::Clock(v) => &v.output,
            _ => panic!("Cannot extract block from {:?}", self),
        }
    }
}

/// generate [0.0, 0.0, 0.0] and reorder
///
/// backups SAVE, acts like stack?
///
/// generate modules separately and combine later
pub fn realize_netlist(ir: Vec<SSA>) {
    let backup = SAVE.lock().unwrap().clone();
    SAVE.lock().unwrap().reset();

    let mut var_hash: HashMap<Sid, RtlKind> = HashMap::new();

    macro_rules! get_contents_cloned {
        ($key:expr) => {{
            var_hash
                .get(&$key)
                .expect(&format!("No such build: {:?}", $key))
                .get_contents()
                .clone()
        }};
    }

    for ssa in ir {
        match ssa {
            SSA::Const { id, width, value } => {
                var_hash.insert(
                    id,
                    RtlKind::Const(Const::new_u(BLOCKS_DEFAULT_POS, width, value)),
                );
            }
            SSA::Wire { id, width, src } => {
                let wire = Wire::new(BLOCKS_DEFAULT_POS, width);
                CustomBus::from_vec(get_contents_cloned!(src)).connect_bitwise_lsb(&wire);
                var_hash.insert(id, RtlKind::Wire(wire));
            }
            SSA::Reg {
                id,
                width,
                init_value,
            } => {
                var_hash.insert(
                    id,
                    RtlKind::Reg(Reg::new(BLOCKS_DEFAULT_POS, width, init_value)),
                );
            }
            SSA::Mem { id, width, length } => {
                todo!()
            }
            SSA::RegIn { id, data, reg, en } => {
                let reg_contents = Reg::from_vec(get_contents_cloned!(reg));
                let src_bus = CustomBus::from_vec(get_contents_cloned!(data));
                let clk = get_contents_cloned!(en);
                let width = src_bus.width;
                var_hash.insert(
                    id,
                    RtlKind::RegIn(RegIn::new_driven(
                        BLOCKS_DEFAULT_POS,
                        width,
                        &src_bus,
                        &reg_contents,
                        &clk[0],
                    )),
                );
            }
            SSA::MemWrite {
                id,
                addr,
                mem,
                data,
                en,
            } => {
                todo!()
            }
            SSA::MemRead { id, addr, mem, en } => {
                todo!()
            }
            SSA::Add {
                id,
                lhs,
                rhs,
                width,
            } => {
                let a = CustomBus::from_vec(get_contents_cloned!(lhs));
                let b = CustomBus::from_vec(get_contents_cloned!(rhs));
                var_hash.insert(
                    id,
                    RtlKind::Add(Adder::new(
                        BLOCKS_DEFAULT_POS,
                        width,
                        false,
                        &a,
                        &b,
                        RTL_ADDER_TYPE,
                    )),
                );
            }
            SSA::Sub {
                id,
                lhs,
                rhs,
                width,
            } => {
                let a = CustomBus::from_vec(get_contents_cloned!(lhs));
                let b = CustomBus::from_vec(get_contents_cloned!(rhs));
                var_hash.insert(
                    id,
                    RtlKind::Sub(Adder::new(
                        BLOCKS_DEFAULT_POS,
                        width,
                        true,
                        &a,
                        &b,
                        RTL_ADDER_TYPE,
                    )),
                );
            }
            SSA::Mul {
                id,
                lhs,
                rhs,
                width,
            } => {
                todo!()
            }
            SSA::Div {
                id,
                lhs,
                rhs,
                width,
            } => {
                todo!()
            }
            SSA::Mod {
                id,
                lhs,
                rhs,
                width,
            } => {
                todo!()
            }
            SSA::Neg { id, src, width } => {
                let src_bus = CustomBus::from_vec(get_contents_cloned!(src));
                var_hash.insert(
                    id,
                    RtlKind::Neg(Neg::new(BLOCKS_DEFAULT_POS, width, &src_bus)),
                );
            }
            SSA::Bitwise {
                id,
                lhs,
                rhs,
                width,
                gate,
            } => {
                let gate_bus = CustomBus::new(BLOCKS_DEFAULT_POS, width, gate.as_blocktype());
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                let rhs = CustomBus::from_vec(get_contents_cloned!(rhs));
                &lhs >> &gate_bus;
                &rhs >> &gate_bus;
                var_hash.insert(id, RtlKind::Bitwise(gate_bus));
            }
            SSA::Logical {
                id,
                lhs,
                rhs,
                width,
                gate,
            } => {
                let block = Block::new(BLOCKS_DEFAULT_POS, gate.as_blocktype());
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                let rhs = CustomBus::from_vec(get_contents_cloned!(rhs));
                lhs.connect_logical(&block);
                rhs.connect_logical(&block);
                var_hash.insert(id, RtlKind::Logical(vec![block]));
            }
            SSA::ConstShl {
                id,
                lhs,
                rhs,
                width,
            } => {
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                var_hash.insert(
                    id,
                    RtlKind::ConstShl(Shifter::new_constant_left(
                        &lhs,
                        rhs[0] as u16,
                        BLOCKS_DEFAULT_POS,
                    )),
                );
            }
            SSA::ConstShr {
                id,
                lhs,
                rhs,
                width,
                arithmetic,
            } => {
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                var_hash.insert(
                    id,
                    RtlKind::ConstShr(Shifter::new_constant_right(
                        &lhs,
                        rhs[0] as u16,
                        BLOCKS_DEFAULT_POS,
                        arithmetic,
                    )),
                );
            }
            SSA::Comparison {
                id,
                lhs,
                rhs,
                width,
                kind,
            } => {
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                let rhs = CustomBus::from_vec(get_contents_cloned!(rhs));

                let comparison = match kind {
                    ComparisonKind::Eq => Comparison::new_eq(BLOCKS_DEFAULT_POS, width, &lhs, &rhs),
                    ComparisonKind::Neq => {
                        Comparison::new_neq(BLOCKS_DEFAULT_POS, width, &lhs, &rhs)
                    }
                    ComparisonKind::Ge => Comparison::new_ge(BLOCKS_DEFAULT_POS, width, &lhs, &rhs),
                    ComparisonKind::Gt => Comparison::new_gt(BLOCKS_DEFAULT_POS, width, &lhs, &rhs),
                    ComparisonKind::Le => Comparison::new_le(BLOCKS_DEFAULT_POS, width, &lhs, &rhs),
                    ComparisonKind::Lt => Comparison::new_lt(BLOCKS_DEFAULT_POS, width, &lhs, &rhs),
                };

                var_hash.insert(id, RtlKind::Comparison(comparison));
            }
            SSA::Replicate { id, times, src } => {
                let src_bus = CustomBus::from_vec(get_contents_cloned!(src));

                var_hash.insert(
                    id,
                    RtlKind::Replicate(Replicate::new(BLOCKS_DEFAULT_POS, &src_bus, times as u16)),
                );
            }
            SSA::Concat { id, lhs, rhs } => {
                let lhs = CustomBus::from_vec(get_contents_cloned!(lhs));
                let rhs = CustomBus::from_vec(get_contents_cloned!(rhs));
                let width = lhs.width + rhs.width;
                let bus = CustomBus::new(BLOCKS_DEFAULT_POS, width, BlockType::Node);
                lhs.connect_bitwise_msb(&bus);
                rhs.connect_bitwise_lsb(&bus);
                var_hash.insert(id, RtlKind::Concat(bus));
            }
            SSA::Slice {
                id,
                src,
                start,
                end,
            } => {
                let sliced = CustomBus::from_vec(get_contents_cloned!(src));

                var_hash.insert(
                    id,
                    RtlKind::Slice(CustomBus::from_vec(sliced.contains[start..end].to_vec())),
                );
            }
            SSA::Mux {
                id,
                width,
                true_value,
                false_value,
                sel,
            } => {
                let true_val = CustomBus::from_vec(get_contents_cloned!(true_value));
                let false_val = CustomBus::from_vec(get_contents_cloned!(false_value));
                let sel = get_contents_cloned!(sel);

                var_hash.insert(
                    id,
                    RtlKind::Mux(Mux::new(
                        BLOCKS_DEFAULT_POS,
                        width,
                        &true_val,
                        &false_val,
                        &sel[0],
                    )),
                );
            }
            SSA::PosEdge { id, src } => {
                let src = get_contents_cloned!(src)[0];

                var_hash.insert(
                    id,
                    RtlKind::Edge(Edge::new(BLOCKS_DEFAULT_POS, &src, EdgeKind::Rising)),
                );
            }
            SSA::NegEdge { id, src } => {
                let src = get_contents_cloned!(src)[0];

                var_hash.insert(
                    id,
                    RtlKind::Edge(Edge::new(BLOCKS_DEFAULT_POS, &src, EdgeKind::Falling)),
                );
            }

            SSA::GenerateInput {
                id,
                pos,
                name,
                width,
            } => {
                let blocks = generate_io(pos, &name, width).contains;
                let len = blocks.len();
                var_hash.insert(
                    id,
                    RtlKind::Wire(Wire {
                        contains: blocks,
                        width,
                    }),
                );
            }
            SSA::GenerateOutput {
                src,
                pos,
                name,
                width,
            } => {
                let blocks = generate_io(pos, &name, width);
                let source = CustomBus::from_vec(get_contents_cloned!(src));
                source.connect_bitwise_lsb(&blocks);
            }
            SSA::GenerateClock { id, timing } => {
                var_hash.insert(id, RtlKind::Clock(Clock::new(BLOCKS_DEFAULT_POS, timing)));
            }

            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum ConstrToken<'a> {
    GENERATEPHYS,
    SETPROPERTY,
    POS([f32; 3]),
    STRING(&'a str),
    IDENT(&'a str),
    NUMBER(u64),
    INPUT,
    OUTPUT,
    CLOCK,
    TOP,
}

#[derive(Debug)]
pub enum ConstrError<'a> {
    FailedToParseNumber { line: usize, number: &'a str },
    UnexpectedChar { line: usize, ch: char },
    ArrayOverflow { line: usize },
    UnclosedArray { line: usize },
    UnclosedParentheses { line: usize },
    UnclosedString { line: usize },
}

pub enum ConstrIO<'a> {
    Input {
        name: &'a str,
        bitwidth: u64,
        pos: [f32; 3],
        top_name: &'a str,
    },
    Output {
        name: &'a str,
        bitwidth: u64,
        pos: [f32; 3],
        top_name: &'a str,
    },
}

pub struct ConstrClock<'a> {
    timing: u16,
    top_name: &'a str,
}

pub struct Constraints<'a> {
    io_ports: Vec<ConstrIO<'a>>,
    top_module: &'a str,
    clocks: Vec<ConstrClock<'a>>,
}

pub fn parse_constraints<'a>(contents: &'a str) -> Result<(), ConstrError<'a>> {
    let mut pc = 0usize;
    let chars = contents.as_bytes();
    let mut line = 1usize;

    let mut tokens: Vec<ConstrToken> = Vec::new();

    while pc < chars.len() {
        let c = chars[pc] as char;

        match c {
            x if x == '\n' => {
                line += 1;
                pc += 1;
                continue;
            }
            x if x.is_whitespace() => {
                pc += 1;
                continue;
            }
            '#' => {
                while pc < chars.len() && chars[pc] != b'\n' {
                    pc += 1;
                }
            }
            x if x.is_ascii_alphanumeric() || x == '_' => {
                let start = pc;

                while pc < chars.len() && (chars[pc].is_ascii_alphanumeric() || chars[pc] == b'_') {
                    pc += 1;
                }

                let word = &contents[start..pc];

                match word {
                    "generate_phys" => tokens.push(ConstrToken::GENERATEPHYS),
                    "set_property" => tokens.push(ConstrToken::SETPROPERTY),
                    "clock" => tokens.push(ConstrToken::CLOCK),
                    "input" => tokens.push(ConstrToken::INPUT),
                    "output" => tokens.push(ConstrToken::OUTPUT),
                    "top" => tokens.push(ConstrToken::TOP),
                    _ => tokens.push(ConstrToken::IDENT(word)),
                }
            }
            '"' => {
                pc += 1;
                let start = pc;

                while pc < chars.len() && chars[pc] != b'"' {
                    pc += 1;
                }

                if pc >= chars.len() {
                    return Err(ConstrError::UnclosedString { line });
                }

                let word = &contents[start..pc];

                pc += 1;

                tokens.push(ConstrToken::STRING(word))
            }
            '[' => {
                pc += 1;
                let mut coords = [0.0f32; 3];

                for axis in 0..3 {
                    let start = pc;

                    while pc < chars.len() && (chars[pc] != b',' && chars[pc] != b']') {
                        pc += 1;
                    }

                    if pc >= chars.len() {
                        return Err(ConstrError::UnclosedArray { line });
                    }

                    let axis_str = &contents[start..pc];

                    coords[axis] =
                        axis_str
                            .parse()
                            .map_err(|_| ConstrError::FailedToParseNumber {
                                line,
                                number: axis_str,
                            })?;

                    if axis < 2 {
                        if chars[pc] != b',' {
                            return Err(ConstrError::UnclosedArray { line });
                        }
                        pc += 1;
                    }
                }

                if chars[pc] == b',' {
                    return Err(ConstrError::ArrayOverflow { line });
                } else if chars[pc] == b']' {
                    pc += 1;
                    tokens.push(ConstrToken::POS(coords))
                } else {
                    return Err(ConstrError::UnclosedArray { line });
                }
            }
            '(' => {
                pc += 1;
                let start = pc;

                while pc < chars.len() && chars[pc] != b')' {
                    pc += 1;
                }

                if pc >= chars.len() {
                    return Err(ConstrError::UnclosedParentheses { line });
                }

                let number_str = &contents[start..pc];

                let number: u64 =
                    number_str
                        .parse()
                        .map_err(|_| ConstrError::FailedToParseNumber {
                            line,
                            number: number_str,
                        })?;

                pc += 1;
                tokens.push(ConstrToken::NUMBER(number))
            }
            _ => return Err(ConstrError::UnexpectedChar { line, ch: c }),
        }
    }

    Ok(())
}

pub struct VarInfo {
    vtype: parser::VarType,
    name: String,
    bitwidth: u32,
    ascending: bool,
    array_len: Option<parser::ArrayRange>,
    init: Option<parser::Expr>,
}

pub struct Loverer {
    ast: Vec<parser::Item>,
    pub consts: HashMap<Vec<u64>, usize>,
    pub ssa: HashMap<Sid, SSA>,
    pub idents: HashMap<String, usize>,
    pub declared_vars: HashMap<String, VarInfo>,
}

impl Loverer {
    pub fn new(ast: Vec<parser::Item>) -> Self {
        Loverer {
            ast,
            consts: HashMap::new(),
            ssa: HashMap::new(),
            idents: HashMap::new(),
            declared_vars: HashMap::new(),
        }
    }
    pub fn gen_expr(&mut self, expr: parser::Expr) -> Sid {
        match expr {
            parser::Expr::Lit { value } => {
                let width = (value.len() - 1) * 64 + {
                    let v = *value.last().unwrap() as usize;
                    if v == 0 { 0 } else { v.ilog2() as usize }
                };
                self.gen_const(value, width)
            }
            parser::Expr::Signed(signed_expr) => {
                let e = self.gen_expr(*signed_expr);

                match e {
                    Sid::Signed(id) => e,
                    Sid::Unsigned(id) => Sid::Signed(id),
                }
            }
            parser::Expr::Unsigned(signed_expr) => {
                let e = self.gen_expr(*signed_expr);

                match e {
                    Sid::Signed(id) => Sid::Unsigned(id),
                    Sid::Unsigned(id) => e,
                }
            }
            parser::Expr::Binary { lhs, rhs, op } => {
                todo!();
            }
            parser::Expr::BitSelect { src, idx } => {
                todo!()
                // this means i get only one bit from a variable using its index
                // the problem is i dont know order of bits
                // like they can be ascending or descending
                // variables can go down like [7:0] and if i do [7] it will give me highest bit
                // or if i do [0:7] and i get [7] it will give me lowest one
                // i need a way to store that
                // i already have that in variable declarations
                // maybe when i generate new variable i can store its bits order
                // 🦀🦀🦀🦀🦀
            }
            _ => todo!(),
        }
    }
    pub fn next_id(&self) -> usize {
        self.ssa.len()
    }
    pub fn gen_const(&mut self, value: Svalue, width: Swidth) -> Sid {
        if let Some(id) = self.consts.get(&value) {
            Sid::Unsigned(*id)
        } else {
            let id = self.next_id();
            self.ssa.insert(
                Sid::Unsigned(id),
                SSA::Const {
                    id: Sid::Unsigned(id),
                    width,
                    value: value.clone(),
                },
            );

            self.consts.insert(value, id);

            Sid::Unsigned(id)
        }
    }
}
