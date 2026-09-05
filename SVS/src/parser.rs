pub type Idx = u32;

pub enum IOdirection {
    Input,
    Output,
}

pub enum VarType {
    Reg,
    Wire,
}

pub struct ArrayRange {
    start: Idx,
    end: Idx,
    ascending: bool,
}

pub struct IO {
    direction: IOdirection,
    name: String,
    bitwidth: u32,
    ascending_idx: bool,
    vtype: VarType,
    array_len: Option<ArrayRange>,
}

pub struct Parameter {
    name: String,
    value: Expr,
}

pub enum Item {
    Module {
        parameters: Vec<Parameter>,
        name: String,
        ports: Vec<IO>,
        stms: Vec<Stmt>,
    },
}

pub enum Edge {
    Rising(String),
    Falling(String),
    Both(String),
}

pub enum AlwaysSens {
    Comb,
    Seq(Vec<Edge>),
}

pub enum Bit {
    Zero,
    One,
    X,
}

pub enum Stmt {
    Block(Vec<Stmt>),

    Decl {
        vtype: VarType,
        name: String,
        bitwidth: u32,
        ascending: bool,
        array_len: Option<ArrayRange>,
        init: Option<Expr>,
    },

    ContinuousAssign {
        lhs: Expr,
        rhs: Expr,
    },

    BlockingAssign {
        lhs: Expr,
        rhs: Expr,
    },

    NonBlockingAssign {
        lhs: Expr,
        rhs: Expr,
    },

    Instance {
        module_name: String,
        name: String,
        params: Vec<Parameter>,
        ports: Vec<(String, Expr)>,
    },

    Always {
        sens: AlwaysSens,
        body: Box<Stmt>,
    },

    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Case {
        casex: bool,
        subject: Expr,
        arms: Vec<(Vec<Expr>, Stmt)>,
        default: Option<Box<Stmt>>,
    },
}

pub enum Source {
    Literal(Vec<u64>),
    Dynamic(Box<Expr>),
}

pub enum Expr {
    Ident(String),
    Lit {
        value: Vec<u64>,
    },
    Signed(Box<Expr>),
    Unsigned(Box<Expr>),
    Unary {
        op: UnaryOp,
        src: Box<Expr>,
    },
    Binary {
        lhs: Source,
        rhs: Source,
        op: BinaryOp,
    },
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    BitSelect {
        src: Box<Expr>,
        idx: Box<Expr>,
    },
    PartSelect {
        src: Box<Expr>,
        msb: Idx,
        lsb: Idx,
    },
    Replicate {
        src: Box<Expr>,
        times: u32,
    },
    Concat(Vec<Expr>),
    Index {
        src: Box<Expr>,
        idx: Source,
    },
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogAnd,
    LogOr,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Ashr,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
}

pub enum UnaryOp {
    Neg,
    BitNot,
    AndReduct,
    OrReduct,
    NotReduct,
    XorReduct,
}
