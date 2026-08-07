use std::{collections::HashMap, mem::discriminant, str::Matches, string};
use super::*;
#[derive(Clone, Debug)]
pub enum Token {
    COMMA,
    CONNECT,
    DIV,
    END,
    ENDIF,
    EQ,
    EQUAL,
    FOR,
    GE,
    GT,
    IF,
    INT(f32),
    INTKW,
    LBRACK,
    LPAREN,
    LE,
    LET,
    LT,
    MERGE,
    MINUS,
    MINUSMINUS,
    MUL,
    NEQ,
    PLUS,
    PLUSPLUS,
    RBRACK,
    RPAREN,
    SAVEFILE(String),
    SEMICOLON,
    VAR(String),
    BLOCK(SmsBlock),
    LOG,
    MODULO,
    ELSE,
    STRING(String),
    DO,
    THEN,
    MEME
}


#[derive(Clone, Debug)]
pub enum SmsBlock {
    Nor,
    And,
    Or,
    Xor,
    Button,
    FlipFlop,
    Led,
    Sound,
    Conductor,
    Nand,
    Xnor,
    Random,
    Text,
    Tile,
    Node,
    Delay,
    Antenna,
    ConductorV2,
    Ledmixer,
}

impl SmsBlock {
    pub fn fromi32(int: i32) -> Self {
        match int {
            0 => Self::Nor,
            1 => Self::And,
            2 => Self::Or,
            3 => Self::Xor,
            4 => Self::Button,
            5 => Self::FlipFlop,
            6 => Self::Led,
            7 => Self::Sound,
            8 => Self::Conductor,
            10 => Self::Nand,
            11 => Self::Xnor,
            12 => Self::Random,
            13 => Self::Text,
            14 => Self::Tile,
            15 => Self::Node,
            16 => Self::Delay,
            17 => Self::Antenna,
            18 => Self::ConductorV2,
            19 => Self::Ledmixer,
            _ => panic!("Invalid int {int} for smsblock")
        }
    }
}

impl Token {
    pub fn extract_var(&self) -> Option<String> {
        if let Token::VAR(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }
    pub fn extract_int(&self) -> Option<f32> {
        if let Token::INT(x) = self {
            Some(*x)
        } else {
            None
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    code: String,
    pos: usize
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer { tokens: Vec::new(), code, pos: 0 }
    }
    pub fn advance(&mut self) {
        self.pos += 1;
    }
    pub fn peek(&self) -> Option<char> {
        if self.pos < self.code.len() {
        *(&self.code[self.pos..self.pos+1].chars().next().clone())}
        else {None}
    }
    pub fn word(&mut self) -> Token {
        let mut c: String = String::new();
        while let Some(x) = self.peek() {
            if !(x.is_ascii_alphanumeric() || x == '_') {
                break
            }
            c.push(x);
            self.advance();
        }
        match c.as_str() {
            "for"          => Token::FOR,
            "end"          => Token::END,
            "endif"        => Token::ENDIF,
            "int"          => Token::INTKW,
            "merge"        => Token::MERGE,
            "let"          => Token::LET,
            "NOR"          => Token::BLOCK(SmsBlock::Nor),
            "AND"          => Token::BLOCK(SmsBlock::And),
            "OR"           => Token::BLOCK(SmsBlock::Or),
            "XOR"          => Token::BLOCK(SmsBlock::Xor),
            "BUTTON"       => Token::BLOCK(SmsBlock::Button),
            "FLIPFLOP"     => Token::BLOCK(SmsBlock::FlipFlop),
            "LED"          => Token::BLOCK(SmsBlock::Led),
            "SOUND"        => Token::BLOCK(SmsBlock::Sound),
            "CONDUCTOR"    => Token::BLOCK(SmsBlock::Conductor),
            "NAND"         => Token::BLOCK(SmsBlock::Nand),
            "XNOR"         => Token::BLOCK(SmsBlock::Xnor),
            "RANDOM"       => Token::BLOCK(SmsBlock::Random),
            "TEXT"         => Token::BLOCK(SmsBlock::Text),
            "TILE"         => Token::BLOCK(SmsBlock::Tile),
            "NODE"         => Token::BLOCK(SmsBlock::Node),
            "DELAY"        => Token::BLOCK(SmsBlock::Delay),
            "ANTENNA"      => Token::BLOCK(SmsBlock::Antenna),
            "CONDUCTOR_V2" => Token::BLOCK(SmsBlock::ConductorV2),
            "LEDMIXER"     => Token::BLOCK(SmsBlock::Ledmixer),
            "if"           => Token::IF,
            "log"          => Token::LOG,
            "else"         => Token::ELSE,
            "do"           => Token::DO,
            "then"         => Token::THEN,
            "meme"         => Token::MEME,
            _              => Token::VAR(c)
        }
    }
    pub fn number(&mut self) -> Token {
        let mut c: String = String::new();
        while let Some(x) = self.peek() {
            if !((x == '_') || x.is_ascii_digit() || x == '.') {
                break
            }
            c.push(x);
            self.advance();
        }
        Token::INT(c.parse().unwrap())
    }
    pub fn op(&mut self) -> Token {
        let mut op: String = String::new();
        op.push(self.peek().unwrap());
        self.advance();
        if let Some(x) = self.peek() {
            if "-+>".contains(x) {
                op.push(x);
                self.advance();
            } else if x.is_ascii_digit() {
                return Token::INT(-1.0 * self.number().extract_int().unwrap());
            }
        }
        match op.as_str() {
            "+" => Token::PLUS,
            "++" => Token::PLUSPLUS,
            "-" => Token::MINUS,
            "--" => Token::MINUSMINUS,
            "->" => Token::CONNECT,
            "*" => Token::MUL,
            "/" => Token::DIV,
            "%" => Token::MODULO,
            _ => panic!("Unknown operation: {op}")
        }
    }
    pub fn savefile(&mut self) -> Token {
        let mut c = String::new();
        self.advance();
        while let Some(x) = self.peek() {
            match x {
                '"' => {
                    self.advance();
                    return Token::SAVEFILE(c)},
                '\'' => {
                    self.advance();
                    return Token::STRING(c);
                }
                '\\' => {
                    self.advance();
                    if let Some('"') = self.peek() {
                        self.advance();
                        return Token::SAVEFILE(c);
                    } else if let Some('\'') = self.peek() {
                        self.advance();
                        return Token::STRING(c);
                    } else if let Some('n') = self.peek() {
                        self.advance();
                        c.push('\n');
                    } else {
                        c.push('\\')
                    }
                }
                _ => {
                    self.advance();
                    c.push(x);
                }
            }
        }
        panic!("Expected \" at the end of savefile");
    }
    pub fn eq(&mut self) -> Token {
        let mut c = String::new();
        c.push(self.peek().unwrap());
        self.advance();
        if let Some('=') = self.peek() {
            c.push('=');
            self.advance();
        }
        match c.as_str() {
            "=" => Token::EQUAL,
            "==" => Token::EQ,
            ">" => Token::GT,
            ">=" => Token::GE,
            "<" => Token::LT,
            "<=" => Token::LE,
            "!=" => Token::NEQ,
            _ => panic!("Unknown operator: {c:?}")
        }
    }
    pub fn lex(&mut self) {
        let mut tok: Token = Token::SEMICOLON;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                tok = self.word();
            } else if c.is_ascii_digit() {
                tok = self.number();
            } else if "+-*%/".contains(c) {
                tok = self.op();
            } else if "\"'".contains(c) {
                tok = self.savefile();
            } else if c.is_whitespace() {
                self.advance();
                continue;
            } else if c == ';' {
                self.advance();
                tok = Token::SEMICOLON;
            } else if "><!=".contains(c) {
                tok = self.eq();
            } else if c == ',' {
                tok = Token::COMMA;
                self.advance();
            } else if c == '(' {
                tok = Token::LPAREN;
                self.advance();
            } else if c == ')' {
                tok = Token::RPAREN;
                self.advance();
            } else if c == '[' {
                tok = Token::LBRACK;
                self.advance();
            } else if c == ']' {
                tok = Token::RBRACK;
                self.advance();
            } else if c == '#' {
                while let Some(x) = self.peek() {
                    self.advance();
                    if x == '\n' {
                        break;
                    }
                }
                continue;
            } else {
                panic!("Unexpected symbol: \"{c}\"")
            }
            self.tokens.push(tok);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Bop {
    Mul,
    Add,
    Sub,
    Div,
    Eq,
    Gt,
    Ge,
    Le,
    Lt,
    Neq,
    Modulo,
}

#[derive(Clone, Debug)]
pub enum Uop {
    Useadd,
    Usesub,
    Adduse,
    Subuse
}

#[derive(Clone, Debug)]
pub enum Expr {
    For { name: String, value: Box<Expr>, expr: Box<Expr>, addition: Box<Expr>, code: Vec<Expr> },
    BinOp { left: Box<Expr>, op: Bop, right: Box<Expr> },
    Merge { first: Box<Expr>, second: Box<Expr> },
    Offset { dx: f32, dy: f32, dz: f32 },
    If { expr: Box<Expr>, true_code: Vec<Expr>, false_code: Vec<Expr> },
    Int { value: f32 },
    Var { name: String },
    UnOp { operand: Box<Expr>, op: Uop },
    Decl { name: String, value: Box<Expr>, off: Box<Expr> },
    Assign { dest: Box<Expr>, value: Box<Expr> },
    Block { blocktype: SmsBlock },
    Savefile { text: String },
    ExprOffset { dx: Box<Expr>, dy: Box<Expr>, dz: Box<Expr> },
    VexprOff { name: String, off: Box<Expr> },
    Log { expr: Vec<Expr> },
    String { value: String },
    Meme { dest: Box<Expr>, value: String },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub ast: Vec<Expr>
}

// utilites
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0, ast: Vec::new() }
    }
    pub fn advance(&mut self) {
        self.pos += 1;
    }
    pub fn peek(&self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos].clone())
        } else {
            None
        }
    }
    pub fn requireint(&self) {
        if let Some(Token::INT(x)) = self.peek() {} else {
            panic!("Expected int got {:?}", self.peek())
        }
    }
    pub fn requiresv(&self) {
        if let Some(Token::SAVEFILE(x)) = self.peek() {} else {
            panic!("Expected savefile got {:?}", self.peek())
        }
    }
    pub fn requirevar(&self) {
        if let Some(Token::VAR(x)) = self.peek() {} else {
            panic!("Expected name got {:?}", self.peek())
        }
    }
    pub fn requiretok(&self, tok: Token) {
        if matches!(self.peek(), Some(tok)) {} else {
            panic!("Expected {tok:?} got {:?}", self.peek())
        }
    }
    pub fn requirestring(&self) {
        if let Some(Token::STRING(x)) = self.peek() {} else {
            panic!("Expected string, got {:?}", self.peek())
        }
    }
    pub fn getcords(&mut self) -> (f32, f32, f32) {
        self.advance();
        self.requireint();
        let x = self.peek().unwrap().extract_int().unwrap() as f32;
        self.advance();
        self.requiretok(Token::COMMA);
        self.advance();
        let y = self.peek().unwrap().extract_int().unwrap() as f32;
        self.advance();
        self.requiretok(Token::COMMA);
        self.advance();
        let z = self.peek().unwrap().extract_int().unwrap() as f32;
        self.advance();
        self.requiretok(Token::RBRACK);
        self.advance();
        (x, y, z)
    }
    pub fn getoff(&mut self) -> Expr {
        let modifier = match self.peek() {
            Some(Token::PLUS) => 1,
            Some(Token::MINUS) => -1,
            _ => panic!("Offset modifier can`t be other than + or -")
        };
        self.advance();
        self.requiretok(Token::LBRACK);
        let (dx, dy, dz) = self.getcords();
        Expr::Offset { dx: modifier as f32*dx, dy: modifier as f32*dy, dz: modifier as f32*dz }
    }
    pub fn getexproff(&mut self) -> Expr {
        self.requiretok(Token::LBRACK);
        self.advance();
        let x = self.parse_exp();
        self.requiretok(Token::COMMA);
        self.advance();
        let y = self.parse_exp();
        self.requiretok(Token::COMMA);
        self.advance();
        let z = self.parse_exp();
        self.requiretok(Token::RBRACK);
        self.advance();
        Expr::ExprOffset { dx: Box::new(x), dy: Box::new(y), dz: Box::new(z) }
    }
}

// expression parsing part
impl Parser {
    pub fn parse_fact(&mut self) -> Expr {
        let first = self.peek().unwrap_or_else(|| panic!("Expected expression, got none"));
        self.advance();
        let mut getnumvar = || -> Expr {
            let tok = self.peek();
            self.advance();
            if let Some(Token::VAR(x)) = tok {
                Expr::Var { name: x }
            } else if let Some(Token::INT(x)) = tok {
                Expr::Int { value: x }
            } else {
                panic!("Expected variable or int, got {tok:?}")
            }
        };
        match first {
            Token::PLUSPLUS => Expr::UnOp { operand: Box::new(getnumvar()), op: Uop::Adduse },
            Token::MINUSMINUS => Expr::UnOp { operand: Box::new(getnumvar()), op: Uop::Subuse },
            Token::VAR(x) => {
                match self.peek() {
                    Some(Token::PLUSPLUS) => {self.advance(); Expr::UnOp { operand: Box::new(Expr::Var { name: x }), op: Uop::Useadd }},
                    Some(Token::MINUSMINUS) => {self.advance(); Expr::UnOp { operand: Box::new(Expr::Var { name: x }), op: Uop::Usesub }},
                    Some(Token::LBRACK) => {
                        let off = self.getexproff();
                        Expr::VexprOff { name: x, off: Box::new(off) }
                    }
                    _ => Expr::Var { name: x }
                }
            }
            Token::INT(x) => {
                match self.peek() {
                    Some(Token::PLUSPLUS) => {self.advance(); Expr::UnOp { operand: Box::new(Expr::Int { value: x }), op: Uop::Useadd }},
                    Some(Token::MINUSMINUS) => {self.advance(); Expr::UnOp { operand: Box::new(Expr::Int { value: x }), op: Uop::Usesub }},
                    _ => Expr::Int { value: x }
                }
            }
            Token::SAVEFILE(x) => {
                let exp = Expr::Savefile { text: x };
                exp
            },
            Token::LPAREN => {
                let exp = self.parse_exp();
                self.requiretok(Token::RPAREN);
                self.advance();
                exp
            }
            Token::BLOCK(x) => {
                Expr::Block { blocktype: x }
            }
            Token::STRING(value) => {
                Expr::String { value }
            }
            _ => panic!("Expected expression, got: {first:?}")
        }
    }
    pub fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_fact();
        let mut op; 
        let mut right;
        while matches!(self.peek(), Some(Token::MUL) | Some(Token::DIV) | Some(Token::MODULO)) {
            op = match self.peek() {
                Some(Token::MUL) => Bop::Mul,
                Some(Token::MODULO) => Bop::Modulo,
                _ => Bop::Div
            };
            self.advance();
            right = self.parse_fact();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) }
        }
        left
    }
    pub fn parse_pm(&mut self) -> Expr {
        let mut left = self.parse_term();
        let mut op; 
        let mut right;
        while matches!(self.peek(), Some(Token::PLUS) | Some(Token::MINUS)) {
            op = match self.peek() {
                Some(Token::PLUS) => Bop::Add,
                _ => Bop::Sub
            };
            self.advance();
            right = self.parse_term();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) }
        }
        left
    }
    pub fn parse_exp(&mut self) -> Expr {
        let mut left = self.parse_pm();
        let mut op; 
        let mut right;
        while matches!(self.peek(), Some(Token::GE) | Some(Token::GT) | Some(Token::LT) | Some(Token::LE) | Some(Token::NEQ) | Some(Token::EQ) | Some(Token::CONNECT)) {
            op = match self.peek() {
                Some(Token::EQ) => Bop::Eq,
                Some(Token::NEQ) => Bop::Neq,
                Some(Token::GE) => Bop::Ge,
                Some(Token::GT) => Bop::Gt,
                Some(Token::LT) => Bop::Lt,
                Some(Token::LE) => Bop::Le,
                Some(Token::MODULO) => Bop::Modulo,
                _ => Bop::Eq
            };
            self.advance();
            right = self.parse_pm();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) }
        }
        left
    }
}

// main parsing code
impl Parser {
    pub fn parse_decl(&mut self) -> Expr {
        self.advance();
        let mut off;
        if matches!(self.peek(), Some(Token::PLUS) | Some(Token::MINUS)) {
            off = self.getoff();
        } else {
            off = Expr::Offset { dx: 0.0, dy: 0.0, dz: 0.0 }
        }
        self.requirevar();
        let name = self.peek().unwrap().extract_var().unwrap();
        self.advance();
        self.requiretok(Token::EQUAL);
        self.advance();
        let expr = self.parse_exp();
        self.requiretok(Token::SEMICOLON);
        self.advance();
        Expr::Decl { name, value: Box::new(expr), off: Box::new(off) }
    }
    pub fn parse_assign(&mut self) -> Expr {
        self.requirevar();
        let name = self.parse_exp();
        self.requiretok(Token::EQUAL);
        self.advance();
        let expr = self.parse_exp();
        self.requiretok(Token::SEMICOLON);
        self.advance();
        Expr::Assign { dest: Box::new(name), value: Box::new(expr)}
    }
    pub fn parse_for(&mut self) -> Expr {
        self.advance();
        self.requirevar();
        let name = self.peek().unwrap().extract_var().unwrap();
        self.advance();
        self.requiretok(Token::EQUAL);
        self.advance();
        let value = self.parse_exp();
        self.requiretok(Token::SEMICOLON);
        self.advance();
        let expr = self.parse_exp();
        self.requiretok(Token::SEMICOLON);
        self.advance();
        let addition = self.parse_exp();
        self.requiretok(Token::DO);
        self.advance();
        let code = self.parse_code(vec![Token::END]);
        self.advance();
        Expr::For { name, value: Box::new(value), expr: Box::new(expr), addition: Box::new(addition), code }
    }
    pub fn parse_if(&mut self) -> Expr {
        self.advance();
        let expr = self.parse_exp();
        self.requiretok(Token::THEN);
        self.advance();
        let code = self.parse_code(vec![Token::ENDIF, Token::ELSE]);
        let false_code;
        if let Some(Token::ELSE) = self.peek() {
            self.advance();
            if let Some(Token::IF) = self.peek() {
                false_code = vec![self.parse_if()];
            } else {
                false_code = self.parse_code(vec![Token::ENDIF]);
                self.advance();
            }
        } else {
            self.advance();
            false_code = Vec::new();
        }
        Expr::If { expr: Box::new(expr), true_code: code, false_code: false_code }
    }
    pub fn parse_merge(&mut self) -> Expr {
        self.advance();
        self.requirevar();
        let first = self.peek().unwrap().extract_var().unwrap();
        self.advance();
        let second = self.peek().unwrap().extract_var().unwrap();
        self.advance();
        Expr::Merge { first: Box::new(Expr::Var { name: first }), second: Box::new(Expr::Var { name: second }) }
    }
    pub fn parse_log(&mut self) -> Expr {
        self.advance();
        let mut exprs: Vec<Expr> = Vec::new();
        exprs.push(self.parse_exp());
        while let Some(Token::COMMA) = self.peek() {
            self.advance();
            exprs.push(self.parse_exp());
        }
        self.requiretok(Token::SEMICOLON);
        self.advance();
        Expr::Log { expr: exprs }
    }
    pub fn parse_meme(&mut self) -> Expr {
        self.advance();
        self.requirevar();
        let name = self.peek().unwrap().extract_var().unwrap();
        self.advance();
        self.requiretok(Token::LBRACK);
        let off = self.getexproff();
        self.requiretok(Token::EQUAL);
        self.advance();
        let path;
        if let Some(Token::STRING(v)) = self.peek() {
            path = v;
        } else {
            panic!("Expected string, got {:?}", self.peek())
        }
        self.advance();
        self.requiretok(Token::SEMICOLON);
        self.advance();
        Expr::Meme { dest: Box::new(Expr::VexprOff { name, off: Box::new(off) }), value: path }

    }
}

// calls 
impl Parser {
    pub fn parse_line(&mut self) -> Expr {
        let tok = self.peek();
        if let None = tok {
            panic!("Expected code, got none");
        } else {
            match tok.clone().unwrap() {
                Token::LET => self.parse_decl(),
                Token::VAR(x) => self.parse_assign(),
                Token::IF => self.parse_if(),
                Token::FOR => self.parse_for(),
                Token::MERGE => self.parse_merge(),
                Token::LOG => self.parse_log(),
                Token::MEME => self.parse_meme(),
                _ => panic!("Unexpected token: {:?}", tok.unwrap())
            }
        }

    }
    pub fn parse_code(&mut self, endpoint: Vec<Token>) -> Vec<Expr> {
        let mut code: Vec<Expr> = Vec::new();
        while let Some(actual_tok) = self.peek() {
            if endpoint.iter().any(|t| discriminant(t) == discriminant(&actual_tok)) {
                break;
            } else {
                code.push(self.parse_line());
            }
        }
        code
    }
    pub fn parse(&mut self) {
        while let Some(tok) = self.peek() {
            let expr = self.parse_line();
            self.ast.push(expr);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Varval {
    Int(f32),
    Savefile(String, f32, f32, f32),
    Block(SmsBlock),
    String(String)
}

pub struct Evaluator {
    pub ast: Vec<Expr>,
    pos: usize,
    pub symtab: HashMap<String, Varval>,
    pub decodedtab: HashMap<String, Save>,
    pub decodedpostab: HashMap<String, HashMap<(u32, u32, u32), u32>>,
    pub idtoblock: HashMap<String, HashMap<u32, Block>>,
    pub constoadd: HashMap<String, Vec<Connection>>,
    // cords - path
    pub memein: HashMap<String, HashMap<(u32, u32, u32), String>>,
    pub do_merge: bool
}

impl Varval {
    pub fn extract_float(&self) -> f32 {
        if let Varval::Int(x) = self {
            *x
        } else {
            panic!("Cant extract int from other variable type than integer")
        }
    }
    pub fn extract_save(&self) -> String {
        if let Varval::Savefile(save, x, y, z) = self {
            save.clone()
        } else {
            panic!("Cant extract string from other variable type than savefile")
        }
    }
    pub fn extract_block(&self) -> SmsBlock {
        if let Varval::Block(block) = self {
            block.clone()
        } else {
            panic!("Cant extract block from other variable type than block")
        }
    }
    pub fn extract_val(&self) -> String {
        match self {
            Varval::Block(x) => format!("{:?}", self.extract_block()),
            Varval::Int(x) => x.to_string(),
            Varval::Savefile(text,x ,y , z) => text.clone(),
            Varval::String(string) => string.clone()
        }
    }
}

impl Evaluator {
    pub fn new(ast: Vec<Expr> ) -> Evaluator {
        Evaluator { ast, pos: 0, symtab: HashMap::new(), decodedtab: HashMap::new(), decodedpostab: HashMap::new(), idtoblock: HashMap::new(), constoadd: HashMap::new(), memein: HashMap::new(), do_merge: false }
    }
    pub fn eval_exp(&mut self, expr: Expr) -> f32 {
        match expr.clone() {
            Expr::BinOp { left, op, right } => {
                if let Expr::VexprOff { name, off } = *left.clone() {
                    let block_type;
                    if let Expr::Block { blocktype } = *right.clone() {
                        block_type = blocktype;
                    } else {
                        panic!("If comparing block with something, right operand must be uppercase block type")
                    }
                    let x;
                    let y;
                    let z;
                    if let Expr::ExprOffset { dx, dy, dz } = *off {
                        x = self.eval_exp(*dx);
                        y = self.eval_exp(*dy);
                        z = self.eval_exp(*dz);
                    } else {panic!()}
                    let id = *self.decodedpostab.get(&name)
                        .unwrap_or_else(|| panic!("No such var: {name}"))
                        .get(&(x.to_bits(),y.to_bits(),z.to_bits()))
                        .unwrap_or_else(|| panic!("No such block on {x},{y},{z} in {name}"));
                    let block = self.idtoblock.get(&name)
                        .unwrap()
                        .get(&id)
                        .unwrap();
                    if discriminant(&block.blocktype.as_sms()) == discriminant(&block_type) {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
                let l = self.eval_exp(*left);
                let r = self.eval_exp(*right);
                match op {
                    Bop::Add => l + r,
                    Bop::Div => l / r,
                    Bop::Eq => (l.to_bits() == r.to_bits()) as i32 as f32,
                    Bop::Neq => (l.to_bits() != r.to_bits()) as i32 as f32,
                    Bop::Ge => (l.to_bits() >= r.to_bits()) as i32 as f32,
                    Bop::Gt => (l.to_bits() > r.to_bits()) as i32 as f32,
                    Bop::Le => (l.to_bits() <= r.to_bits()) as i32 as f32,
                    Bop::Lt => (l.to_bits() < r.to_bits()) as i32 as f32,
                    Bop::Mul => l * r,
                    Bop::Sub => l - r,
                    Bop::Modulo => l % r,
                    _ => panic!("{expr:?} is not a number expression")
                }
            }
            Expr::UnOp { operand, op } => {
                let mut vname = String::new();
                let mut o = match *operand.clone() {
                    Expr::Var { name } => {
                        vname = name;
                        self.eval_exp(*operand)
                    }
                    _ => self.eval_exp(*operand)
                    };
                match op {
                    Uop::Adduse => {
                        self.write_var(vname, Varval::Int(o+1.0));
                        o + 1.0
                    }
                    Uop::Subuse => {
                        self.write_var(vname, Varval::Int(o-1.0));
                        o - 1.0
                    }
                    Uop::Useadd => {
                        self.write_var(vname, Varval::Int(o+1.0));
                        o
                    }
                    Uop::Usesub => {
                        self.write_var(vname, Varval::Int(o-1.0));
                        o
                    }
                }
            }
            Expr::Int { value } => value,
            Expr::Var { name } => self.symtab.get(&name).unwrap_or_else(|| panic!("No such var: {name}")).extract_float(),
            _ => panic!("{expr:?} is not a number expression")
        }
    }
    pub fn write_var(&mut self, name: String, val: Varval) {
        self.symtab.insert(name, val);
    }
    pub fn eval_decl(&mut self, expr: Expr) {
        if let Expr::Decl { name, value, off } = expr {
            let mut vvalue: Varval;
            let x;
            let y;
            let z;
            if let Expr::Offset { dx, dy, dz } = *off {
                (x, y, z) = (dx, dy, dz)
            } else {
                panic!("Expected offset, got: {off:?}")
            }
            if let Expr::Savefile { text } = *value.clone() {
                vvalue = Varval::Savefile(text.clone(), x, y, z);
                let mut save = Save::from_string(text.clone(), [x, y, z]);
                self.decodedpostab.insert(name.clone(), HashMap::new());
                let pat = self.decodedpostab.get_mut(&name).unwrap();
                self.idtoblock.insert(name.clone(), HashMap::new());
                let savepat = self.idtoblock.get_mut(&name).unwrap();
                for i in &save.blocks {
                    pat.insert(((i.pos[0] - x).to_bits(), (i.pos[1] - y).to_bits(), (i.pos[2] - z).to_bits()), i.id);
                    savepat.insert(i.id, i.clone());
                }
                self.decodedtab.insert(name.clone(), save);
            } else if let Expr::Block { blocktype } = *value.clone() {
                vvalue = Varval::Block(blocktype)
            } else {
                vvalue = Varval::Int(self.eval_exp(*value))
            }
            self.write_var(name, vvalue);
        } else {
            panic!("Expected decl, got {expr:?}")
        }
    }
    pub fn eval_assign(&mut self, expr: Expr) -> f32 {
        if let Expr::Assign { dest, value } = expr {
            let vdx;
            let vdy;
            let vdz;
            let sname;
            if let Expr::VexprOff { name, off } = *dest {
                let mut dst;
                let mut src;
                let vlen;
                if let Expr::ExprOffset { dx, dy, dz } = *off {
                    vdx = self.eval_exp(*dx);
                    vdy = self.eval_exp(*dy);
                    vdz = self.eval_exp(*dz);
                    dst = *self.decodedpostab.get(&name).unwrap_or_else(|| panic!("No such variable: {name}")).get(&(vdx.to_bits(), vdy.to_bits(), vdz.to_bits())).unwrap_or_else(|| panic!("No such block on ({vdx},{vdy},{vdz})"));
                } else {
                    panic!()
                }
                if let Expr::VexprOff { name, off } = *value {
                    if let Expr::ExprOffset { dx, dy, dz } = *off {
                        let sdx = self.eval_exp(*dx);
                        let sdy = self.eval_exp(*dy);
                        let sdz = self.eval_exp(*dz);
                        src = *self.decodedpostab.get(&name).unwrap_or_else(|| panic!("No such variable: {name}")).get(&(sdx.to_bits(), sdy.to_bits(), sdz.to_bits())).unwrap_or_else(|| panic!("No such block on ({sdx},{sdy},{sdz})"));
                        sname = name;
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
                if name.as_str() == "lower" {
                    vlen = self.decodedtab.get(&name).unwrap().blocks.len();
                } else if sname.as_str() == "lower" {
                    vlen = self.decodedtab.get(&sname).unwrap().blocks.len();
                } else {
                    panic!()
                }
                if (sname.as_str() != name.as_str()) && (sname.as_str() != "lower") {
                    src += vlen as u32;
                }
                if (name.as_str() != "lower") {
                    dst += vlen as u32;
                }
                println!("src: {src}, dst: {dst}");
                self.constoadd.entry(name.clone()).or_insert_with(|| Vec::new()).push(Connection::inew(src, dst));
            } else {
                let vvalue = self.eval_exp(*value);
                if let Expr::Var { name } = *dest {
                    let ptr = self.symtab.insert(name, Varval::Int(vvalue));
                    return vvalue;
                } else {
                    panic!("Expecter variable or offset, got: {:?}", *dest);
                }
            }
        } else {
            panic!("Expected assign, got {expr:?}")
        }
        0.0
    }
    pub fn eval_if(&mut self, expr: Expr) {
        if let Expr::If { expr, true_code, false_code } = expr {
            let vexpr = self.eval_exp(*expr.clone());
            if vexpr == 1.0 {
                self.eval_code(true_code);
            } else if vexpr == 0.0 {
                self.eval_code(false_code);
            } else {
                panic!("Expected boolean expression evaluating to 0 or 1, got {expr:?}")
            }
        } else {
            panic!("Expected if, got {expr:?}")
        }
    }
    pub fn eval_code(&mut self, code: Vec<Expr>) {
        for expr in code {
            match expr {
                Expr::Assign { .. } => {self.eval_assign(expr);}
                Expr::Decl { .. } => self.eval_decl(expr),
                Expr::For { .. } => self.eval_for(expr),
                Expr::If { .. } => self.eval_if(expr),
                Expr::Merge { .. } => {self.eval_merge(expr);},
                Expr::Log { .. } => self.eval_log(expr),
                Expr::Meme { .. } => self.eval_meme(expr),
                _ => panic!("Unexpected expression")
            }
        }
    }
    pub fn eval_for(&mut self, expr: Expr) {
        if let Expr::For { name, value, expr, addition, code } = expr {
            let vvalue = self.eval_exp(*value);
            let v: Option<Varval>;
            if self.symtab.contains_key(&name) {
                v = Some(self.get_var(&name));
            } else {
                v = None;
            }
            let mut rewrite: Option<Varval> = None;
            self.symtab.insert(name.clone(), Varval::Int(vvalue));
            while self.eval_exp(*expr.clone()) == 1.0 {
                if let Some(x) = rewrite {
                    self.write_var(name.clone(), x);
                }
                self.eval_code(code.clone());
                if let Expr::UnOp { operand, op } = *addition.clone() {
                    let tmp = self.eval_exp(*addition.clone());
                    rewrite = Some(self.symtab.get(&name).unwrap().clone());
                    self.write_var(name.clone(), Varval::Int(tmp));
                } else {
                    rewrite = None;
                    let addval = self.eval_exp(*addition.clone());
                    self.write_var(name.clone(), Varval::Int(addval));
                }
            }

            if let Some(val) = v {
                self.write_var(name, val);
            } else {
                self.dump_var(&name);
            }
        } else {
            panic!("Expected for, got {expr:?}")
        }
    }
    pub fn eval_merge(&mut self, expr: Expr) -> Save {
        if let Expr::Merge { first, second } = expr {
            self.do_merge = true;
            let firstn;
            let secondn;
            if let Expr::Var { name } = *first {
                firstn = name;
            } else {
                panic!("Expected name to merge, got {first:?}")
            }
            if let Expr::Var { name } = *second {
                secondn = name;
            } else {
                panic!("Expected name to merge, got {second:?}")
            }

            let mut firsts = self.decodedtab.get(&firstn).unwrap_or_else(|| panic!("No such var: {firstn}")).clone();
            let mut seconds = self.decodedtab.get(&secondn).unwrap_or_else(|| panic!("No such var: {secondn}")).clone();

            let flen = firsts.blocks.len() as u32;
            for block in &mut seconds.blocks {
                block.id += flen;
            }
            for con in &mut seconds.connections {
                con.src += flen;
                con.dst += flen;
            }
            if self.constoadd.contains_key(&secondn) {
                for contoadd in self.constoadd.get(&secondn).unwrap() {
                    seconds.connections.push(*contoadd);
                }
            }
            if self.constoadd.contains_key(&firstn) {
                for contoadd in self.constoadd.get(&firstn).unwrap() {
                    firsts.connections.push(*contoadd);
                }
            }
            firsts.blocks.extend(seconds.blocks);
            firsts.connections.extend(seconds.connections);
            firsts.buildings.extend(seconds.buildings);
            
            self.decodedtab.insert(firstn, firsts.clone());

            firsts
        } else {
            panic!("Expected merge, got {expr:?}")
        }
    }
    pub fn eval_log(&mut self, logexpr: Expr ) {
        if let Expr::Log { expr } = logexpr {
            for exp in expr {
                if let Expr::Var { name } = exp {
                    print!("{}", self.get_var(&name).extract_val())
                } else if let Expr::String {value} = exp {
                    print!("{}", &value)
                } else if let Expr::Int {value} = exp {
                    print!("{}", value)
                } else {
                    print!("{}", self.eval_exp(exp))
                }
            }
        } else {
            panic!("Expected log expression, got {logexpr:?}")
        }
    }
    pub fn eval_meme(&mut self, expr: Expr) {
        if let Expr::Meme { dest, value } = expr {
            let x;
            let y;
            let z;
            let dname;
            if let Expr::VexprOff { name, off } = *dest {
                if let Expr::ExprOffset { dx, dy, dz } = *off {
                    x = self.eval_exp(*dx);
                    y = self.eval_exp(*dy);
                    z = self.eval_exp(*dz);
                } else {
                    panic!("Expected expression offset in meme, got {:?}", off)
                }

                dname = name;
            } else {
                panic!("Expected variable expression offset in meme, got {:?}", dest)
            }

            self.memein.entry(dname.clone()).or_default().insert((x.to_bits(), y.to_bits(), z.to_bits()), value);
        } else {
            panic!("Expected meme, got {:?}", expr)
        }
    }
    pub fn get_var(&mut self, name: &str) -> Varval {
        self.symtab.get(name).unwrap_or_else(|| panic!("No such var: {name}")).clone()
    }
    pub fn eval_self(&mut self) {
        self.eval_code(self.ast.clone());
    }
    pub fn get_save(&mut self, name: &str) -> Save {
        self.decodedtab.get(name).unwrap_or_else(|| panic!("No such var: {name}")).clone()
    }
    pub fn dump_var(&mut self, name: &str) {
        self.symtab.remove(name);
    }
}

pub fn execute_string(code: String) -> Evaluator {
    let mut lexer = Lexer::new(code);
    lexer.lex();
    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    let mut ev = Evaluator::new(parser.ast);
    ev.eval_self();
    ev
}