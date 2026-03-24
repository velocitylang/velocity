use std::{collections::HashMap};

use crate::grammar::{Expr, FnDecl, Stmt, TypeKind};

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

#[derive(Clone, Debug)]
pub struct Vir {
    pub functions: Vec<VirFunction>,
}

#[derive(Clone, Debug)]
pub struct VirFunction {
    pub block_params: Vec<BlockParamData>,
    pub blocks: Vec<Block>,
    pub entry: BlockId,
    pub insts: Vec<Inst>,
    pub name: String,
    pub sig: FunctionSig,
}

impl VirFunction {
    pub fn new(name: String) -> VirFunction {
        return VirFunction {
            name,
            sig: FunctionSig {
                param_tys: Vec::new(),
                ret_ty: None,
            },
            entry: BlockId(0),
            blocks: vec![Block { insts: Vec::new(), params: Vec::new() }],
            insts: Vec::new(),
            block_params: Vec::new(),
        }
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.blocks.len() as u32);
        self.blocks.push(Block {
            insts: Vec::new(),
            params: Vec::new(),
        });
        id
    }

    pub fn append_inst(&mut self, block: BlockId, inst: Inst) -> InstId {
        let id = InstId(self.insts.len() as u32);
        self.insts.push(inst);
        self.blocks[block.0 as usize].insts.push(id);
        id
    }

    pub fn emit_add(&mut self, block: BlockId, lhs: ValueId, rhs: ValueId, ty: TypeKind) -> ValueId {
        let inst = Inst::Value(ValueInst {
            kind: ValueInstKind::Add { lhs, rhs },
            ty
        });
        let id = self.append_inst(block, inst);
        ValueId::Inst(id)
    }

    pub fn emit_sub(&mut self, block: BlockId, lhs: ValueId, rhs: ValueId, ty: TypeKind) -> ValueId {
        let inst = Inst::Value(ValueInst {
            kind: ValueInstKind::Sub { lhs, rhs },
            ty
        });
        let id = self.append_inst(block, inst);
        ValueId::Inst(id)
    }

    pub fn emit_neg(&mut self, block: BlockId, value: ValueId, ty: TypeKind) -> ValueId {
        let id = self.append_inst(block, Inst::Value(ValueInst {
            kind: ValueInstKind::Neg { value },
            ty,
        }));
        ValueId::Inst(id)
    }

    pub fn emit_const(&mut self, block: BlockId, value: Constant) -> ValueId {
        let ty = value.ty();
        let inst = Inst::Value(ValueInst {
            kind: ValueInstKind::Const { value },
            ty,
        });
        let id = self.append_inst(block, inst);
        ValueId::Inst(id)
    }

    pub fn emit_print(&mut self, block: BlockId, value: ValueId) -> InstId {
        self.append_inst(
            block, 
            Inst::Effect(EffectInst::Print { value })
        )
    }

    pub fn emit_return(&mut self, block: BlockId, value: Option<ValueId>) -> InstId {
        self.append_inst(
            block, 
            Inst::Effect(EffectInst::Return { value })
        )
    }

    pub fn value_ty(&self, value: ValueId) -> &TypeKind {
        match value {
            ValueId::Inst(inst_id) => match &self.insts[inst_id.0 as usize] {
                Inst::Value(ValueInst { ty, .. }) => ty,
                Inst::Effect(_) => {
                    panic!("ValueId::Inst({inst_id:?}) refers to non-value instruction")
                }
            },
            ValueId::Param(param_id) => &self.block_params[param_id.0 as usize].ty,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub insts: Vec<InstId>,
    pub params: Vec<ParamId>,
}

#[derive(Clone, Debug)]
pub enum Inst {
    Value(ValueInst),
    Effect(EffectInst),
}

#[derive(Clone, Debug)]
pub struct FunctionSig {
    pub param_tys: Vec<TypeKind>,
    pub ret_ty: Option<TypeKind>,
}

#[derive(Clone, Debug)]
pub struct ValueInst {
    pub kind: ValueInstKind,
    pub ty: TypeKind,
}

#[derive(Clone, Debug)]
pub enum ValueInstKind {
    Const { value: Constant },
    Add { lhs: ValueId, rhs: ValueId },
    Sub { lhs: ValueId, rhs: ValueId },
    Mul { lhs: ValueId, rhs: ValueId },
    Div { lhs: ValueId, rhs: ValueId },
    Neg { value: ValueId },
}

#[derive(Clone, Debug)]
pub enum EffectInst {
    Print { value: ValueId },
    Return { value: Option<ValueId> },
    Jump { target: BlockId, args: Vec<ValueId> },
    Branch {
        cond: ValueId,
        then_block: BlockId,
        then_args: Vec<ValueId>,
        else_block: BlockId,
        else_args: Vec<ValueId>,
    },
}

#[derive(Clone, Debug)]
pub struct BlockParamData {
    pub block: BlockId,
    pub ty: TypeKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParamId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueId {
    Inst(InstId),
    Param(ParamId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

pub struct LowerCtx<'a> {
    pub func: &'a mut VirFunction,
    pub current_block: BlockId,
    pub locals: HashMap<String, LocalBinding>
}

#[derive(Clone, Debug)]
pub struct LocalBinding {
    pub value: ValueId,
    pub ty: TypeKind,
    pub mutable: bool,
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

    println!("Lowering statements:");
    let global_fn = &mut vir.functions[0];
    let mut ctx = LowerCtx {
        func: global_fn,
        current_block: BlockId(0),
        locals: HashMap::new(),
    };

    for item in &ast.items {
        match item {
            Item::Stmt(stmt) => {
                lower_stmt_to_vir(&mut ctx, stmt);
            },
            _ => {},
        }
    }

    ctx.func.emit_return(ctx.current_block, None);

    vir
}

fn lower_fnct_to_vir(fnct: &FnDecl) {
    println!("Lowering function {:?}", fnct);
}

fn lower_stmt_to_vir(ctx: &mut LowerCtx, stmt: &Stmt) {
    println!("{:?}", stmt);

    match stmt {
        Stmt::Print(expr) => {
            let value = lower_expr_to_vir(ctx, expr, None);
            ctx.func.emit_print(ctx.current_block, value);
        }

        Stmt::Let(name, expr, is_mutable, declared_ty) => {
            let ty = declared_ty
                .as_ref()
                .expect("type inference should have assigned a type to every let");
            let value = lower_expr_to_vir(ctx, expr, Some(ty));
            let actual_ty = ctx.func.value_ty(value).clone();

            if &actual_ty != ty {
                panic!(
                    "let '{}' expected type {:?}, got {:?}",
                    name, ty, actual_ty
                );
            }

            ctx.locals.insert(
                name.clone(), 
                LocalBinding {
                    value,
                    ty: actual_ty,
                    mutable: *is_mutable,
                });
        }

        Stmt::ExprStmt(expr) => {
            let _ = lower_expr_to_vir(ctx, expr, None);
        }

        Stmt::Reassign(name, expr) => {
            let binding = ctx
                .locals
                .get(name)
                .unwrap_or_else(|| panic!("unknown variable '{name}'"))
                .clone();

            if !binding.mutable {
                panic!("cannot reassign immutable variable '{name}'");
            }

            let value = lower_expr_to_vir(ctx, expr, Some(&binding.ty));
            let actual_ty = ctx.func.value_ty(value).clone();

            if actual_ty != binding.ty {
                panic!(
                    "cannot assign value of type {:?} to variable '{}' of type {:?}",
                    actual_ty, name, binding.ty
                );
            }

            ctx.locals.insert(
                name.clone(),
                LocalBinding {
                    value,
                    ty: binding.ty,
                    mutable: true,
                },
            );
        }
    }
}

fn lower_expr_to_vir(ctx: &mut LowerCtx<'_>, expr: &Expr, expected_ty: Option<&TypeKind>) -> ValueId {
    match expr {
        Expr::NumberLiteral(n) => {
            let ty = expected_ty.cloned().unwrap_or(TypeKind::I64);
            let value = parse_number_literal(n, &ty);
            ctx.func.emit_const(ctx.current_block, value)
        }

        Expr::String(s) => {
            if let Some(expected) = expected_ty {
                if *expected != TypeKind::String {
                    panic!(
                        "string literal does not match expected type {:?}",
                        expected
                    );
                }
            }
            ctx.func
                .emit_const(ctx.current_block, Constant::String(s.clone()))
        }

        Expr::Bool(b) => {
            if let Some(expected) = expected_ty {
                if *expected != TypeKind::Bool {
                    panic!(
                        "bool literal does not match expected type {:?}",
                        expected
                    );
                }
            }
            ctx.func.emit_const(ctx.current_block, Constant::Bool(*b))
        }

        Expr::Var(name) => ctx
                .locals
                .get(name)
                .unwrap_or_else(|| panic!("unknown variable '{name}'"))
                .value,

        Expr::Add(lhs, rhs) => {
            let lhs_v = lower_expr_to_vir(ctx, lhs, expected_ty);

            let ty = expected_ty
                .cloned()
                .unwrap_or_else(|| ctx.func.value_ty(lhs_v).clone());

            let rhs_v = lower_expr_to_vir(ctx, rhs, Some(&ty));

            let lhs_ty = ctx.func.value_ty(lhs_v).clone();
            let rhs_ty = ctx.func.value_ty(rhs_v).clone();

            if lhs_ty != ty || rhs_ty != ty {
                panic!(
                    "type mismatch in add: lhs={:?}, rhs={:?}, expected={:?}",
                    lhs_ty, rhs_ty, ty
                );
            }

            ctx.func.emit_add(ctx.current_block, lhs_v, rhs_v, ty)
        }

        Expr::Sub(lhs, rhs) => {
            let lhs_v = lower_expr_to_vir(ctx, lhs, expected_ty);

            let ty = expected_ty
                .cloned()
                .unwrap_or_else(|| ctx.func.value_ty(lhs_v).clone());

            let rhs_v = lower_expr_to_vir(ctx, rhs, Some(&ty));

            let lhs_ty = ctx.func.value_ty(lhs_v).clone();
            let rhs_ty = ctx.func.value_ty(rhs_v).clone();

            if lhs_ty != ty || rhs_ty != ty {
                panic!(
                    "type mismatch in sub: lhs={:?}, rhs={:?}, expected={:?}",
                    lhs_ty, rhs_ty, ty
                );
            }

            ctx.func.emit_sub(ctx.current_block, lhs_v, rhs_v, ty)
        }

        Expr::Negate(inner) => {
            let inner_v = lower_expr_to_vir(ctx, inner, expected_ty);
            let ty = ctx.func.value_ty(inner_v).clone();
            ctx.func.emit_neg(ctx.current_block, inner_v, ty)
        }

        _ => todo!("lower_expr_to_vir for {:?}", expr),
    }
}

pub fn parse_number_literal(text: &str, ty: &TypeKind) -> Constant {
    match ty {
        TypeKind::I8 => Constant::I8(
            text.parse::<i8>().expect("invalid i8 literal"),
        ),
        TypeKind::I16 => Constant::I16(
            text.parse::<i16>().expect("invalid i16 literal"),
        ),
        TypeKind::I32 => Constant::I32(
            text.parse::<i32>().expect("invalid i32 literal"),
        ),
        TypeKind::I64 => Constant::I64(
            text.parse::<i64>().expect("invalid i64 literal"),
        ),
        TypeKind::U8 => Constant::U8(
            text.parse::<u8>().expect("invalid u8 literal"),
        ),
        TypeKind::U16 => Constant::U16(
            text.parse::<u16>().expect("invalid u16 literal"),
        ),
        TypeKind::U32 => Constant::U32(
            text.parse::<u32>().expect("invalid u32 literal"),
        ),
        TypeKind::U64 => Constant::U64(
            text.parse::<u64>().expect("invalid u64 literal"),
        ),
        TypeKind::F32 => Constant::F32(
            text.parse::<f32>().expect("invalid f32 literal"),
        ),
        TypeKind::F64 => Constant::F64(
            text.parse::<f64>().expect("invalid f64 literal"),
        ),
        other => panic!("numeric literal cannot have type {:?}", other),
    }
}
