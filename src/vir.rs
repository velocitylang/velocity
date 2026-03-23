use std::collections::HashMap;

use crate::grammar::{FnDecl, Stmt, TypeKind};

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Stmt(Stmt),
    Function(FnDecl),
}

#[derive(Clone, Debug)]
pub struct Block {
    pub insts: Vec<InstId>,
}

#[derive(Clone, Debug)]
pub struct Vir {
    pub functions: Vec<VirFunction>,
}

#[derive(Clone, Debug)]
pub struct VirFunction {
    pub name: String,
    pub entry: BlockId,
    pub blocks: Vec<Block>,
    pub values: Vec<ValueData>,
    pub insts: Vec<InstData>,
}

impl VirFunction {
    fn new(name: String) -> VirFunction {
        return VirFunction {
            name,
            entry: BlockId(0),
            blocks: vec![Block { insts: Vec::new() }],
            values: Vec::new(),
            insts: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Constant {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
}

impl Constant {
    pub fn ty(&self) -> TypeKind {
        match self {
            Constant::I8(_) => TypeKind::I8,
            Constant::I16(_) => TypeKind::I16,
            Constant::I32(_) => TypeKind::I32,
            Constant::I64(_) => TypeKind::I64,
            Constant::U8(_) => TypeKind::U8,
            Constant::U16(_) => TypeKind::U16,
            Constant::U32(_) => TypeKind::U32,
            Constant::U64(_) => TypeKind::U64,
            Constant::F32(_) => TypeKind::F32,
            Constant::F64(_) => TypeKind::F64,
            Constant::Bool(_) => TypeKind::Bool,
            Constant::String(_) => TypeKind::String,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Clone, Debug)]
pub struct ValueData {
    pub ty: TypeKind,
    pub def_inst: Option<InstId>,
}

#[derive(Clone, Debug)]
pub enum InstData {
    Const {
        value: Constant,
        ty: TypeKind,
        out: ValueId,
    },
    Add {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Sub {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Mul {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Div {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Print {
        value: ValueId,
    },
    Return {
        value: Option<ValueId>,
    },
}

pub fn lower_program(ast: &Program) -> Vir {
    let mut vir = Vir { 
        functions: vec![VirFunction::new(String::from("global"))], 
    };
    let mut functions: HashMap<String, FnDecl> = HashMap::new();

    // register user-defined functions
    // so we have a record of them
    // when we iterate through globals.
    for item in &ast.items {
        match item {
            Item::Function(fnct) => {
                // check if function name already exists
                if let Some(fn_decl) = functions.get(&fnct.name) {
                    panic!("Function named '{:?}' already exists", fn_decl.name);
                }

                functions.insert(fnct.name.clone(), fnct.clone());
            },
            _ => {} // ignore all other item types
        }
    }

    for (_, fnct_decl) in functions {
        let _fnct_vir = lower_fnct_to_vir(&fnct_decl);

        // insert function VIR into vir.functions
    }

    println!("Lowering the following statements:");
    for item in &ast.items {
        match item {
            Item::Stmt(stmt) => {
                let _stmt_vir = lower_stmt_to_vir(stmt);

                // insert statement VIR into "global" vir.functions
            },
            _ => {},
        }
    }

    vir
}

fn lower_fnct_to_vir(fnct: &FnDecl) {
    println!("Lowering function {:?}", fnct);
}

fn lower_stmt_to_vir(stmt: &Stmt) {
    println!("{:?}", stmt);

    match stmt {
        Stmt::Print(expr) => {

        },
        Stmt::Let(ident, expr, mutable, ty) => {

        },
        Stmt::ExprStmt(expr) => {

        },
        Stmt::Reassign(ident, expr) => {

        }
    }
}

// fn lower_expr_to_vir(expr: &Expr) -> VirFunction {

// }
