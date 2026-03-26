use std::{collections::HashMap};

use crate::{grammar::{Expr, FnDecl, Stmt, TypeKind}, ppv::dump_vir};

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

    fn emit_return(&mut self, block: BlockId, value: Option<ValueId>) -> InstId {
        self.append_inst(
            block, 
            Inst::Effect(EffectInst::Return { value })
        )
    }

    pub fn ensure_final_return(&mut self) {
        let last_block_id = BlockId(self.blocks.len() as u32 - 1);

        // Check last instruction of last block
        let last_is_terminator = self.blocks[last_block_id.0 as usize]
            .insts
            .last()
            .map_or(false, |inst_id| {
                matches!(
                    self.insts[inst_id.0 as usize],
                    Inst::Effect(EffectInst::Return { .. })
                        | Inst::Effect(EffectInst::Branch { .. })
                        | Inst::Effect(EffectInst::Jump { .. })
                )
            });

        if !last_is_terminator {
            self.emit_return(last_block_id, None);
        }
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

    ctx.func.ensure_final_return();

    println!("\nVIR is {:?}\n", vir);
    println!("Pretty Printed VIR:");
    dump_vir(&vir);

    // Verify VIR
    if let Err(err) = verify_vir(&vir) {
        panic!("{err}");
    } else {
        println!("\nVIR verified ✅")
    }

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
            lower_expr_to_vir(ctx, expr, None);
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
        },
        Stmt::Return(expr) => {
            lower_expr_to_vir(ctx, expr, None);
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

        Expr::If { condition, then_branch, else_branch } => {
            let cond_value = lower_expr_to_vir(ctx, condition, Some(&TypeKind::Bool));

            let then_block = ctx.func.new_block();
            let else_block = ctx.func.new_block();
            let join_block = ctx.func.new_block();

            ctx.func.append_inst(
                ctx.current_block,
                Inst::Effect(EffectInst::Branch {
                    cond: cond_value,
                    then_block,
                    then_args: vec![],
                    else_block,
                    else_args: vec![],
                }),
            );

            ctx.current_block = then_block;
            let then_value = lower_expr_to_vir(ctx, then_branch, None);
            let result_ty = ctx.func.value_ty(then_value).clone();

            ctx.func.append_inst(
                ctx.current_block,
                Inst::Effect(EffectInst::Jump {
                    target: join_block,
                    args: vec![then_value],
                }),
            );

            ctx.current_block = else_block;
            let else_value = if let Some(else_expr) = else_branch {
                lower_expr_to_vir(ctx, else_expr, Some(&result_ty))
            } else {
                ctx.func.emit_const(ctx.current_block, Constant::I32(0))
            };

            ctx.func.append_inst(
                ctx.current_block,
                Inst::Effect(EffectInst::Jump {
                    target: join_block,
                    args: vec![else_value],
                }),
            );

            let join_param_id = ParamId(ctx.func.block_params.len() as u32);
            ctx.func.block_params.push(BlockParamData {
                block: join_block,
                ty: result_ty.clone(),
            });
            ctx.func.blocks[join_block.0 as usize]
                .params
                .push(join_param_id);

            ctx.current_block = join_block;

            ValueId::Param(join_param_id)
        }

        Expr::Block(stmts) => {
            lower_block_expr_to_vir(ctx, stmts, expected_ty)
        }

        _ => todo!("lower_expr_to_vir for {:?}", expr),
    }
}

fn lower_block_expr_to_vir(
    ctx: &mut LowerCtx<'_>,
    stmts: &[Stmt],
    expected_ty: Option<&TypeKind>,
) -> ValueId {
    let mut last_value = ctx.func.emit_const(ctx.current_block, Constant::I64(0));

    for (i, stmt) in stmts.iter().enumerate() {
        match stmt {
            Stmt::ExprStmt(expr) => {
                let val = lower_expr_to_vir(ctx, expr, expected_ty);
                last_value = val;
            }

            Stmt::Let(name, expr, mutable, ty_opt) => {
                let ty = ty_opt
                    .as_ref()
                    .unwrap_or_else(|| panic!("Block local let '{name}' missing type"));
                let val = lower_expr_to_vir(ctx, expr, Some(ty));

                ctx.locals.insert(
                    name.clone(),
                    LocalBinding {
                        value: val,
                        ty: ty.clone(),
                        mutable: *mutable,
                    },
                );

                last_value = val;
            }

            Stmt::Reassign(name, expr) => {
                let (binding_ty, binding_mutable) = match ctx.locals.get(name) {
                    Some(b) => (b.ty.clone(), b.mutable),
                    None => panic!("unknown variable '{name}'"),
                };

                if !binding_mutable {
                    panic!("Cannot reassign immutable variable '{name}' inside block");
                }

                let val = lower_expr_to_vir(ctx, expr, Some(&binding_ty));
                let actual_ty = ctx.func.value_ty(val).clone();

                if actual_ty != binding_ty {
                    panic!(
                        "cannot assign value of type {:?} to variable '{}' of type {:?}",
                        actual_ty, name, binding_ty
                    );
                }

                ctx.locals.insert(
                    name.clone(),
                    LocalBinding {
                        value: val,
                        ty: binding_ty.clone(),
                        mutable: true,
                    },
                );

                last_value = val;
            }

            Stmt::Return(expr) => {
                let val = lower_expr_to_vir(ctx, expr, expected_ty);
                ctx.func.emit_return(ctx.current_block, Some(val));
                return val;
            }

            Stmt::Print(expr) => {
                let val = lower_expr_to_vir(ctx, expr, None);
                ctx.func.emit_print(ctx.current_block, val);
            }
        }

        // Each statement that is not the last keeps executing, so we keep updating local context.
        if i == stmts.len() - 1 {
            // If this was the last statement, last_value is the block’s result
        }
    }

    last_value
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

pub fn verify_vir(vir: &Vir) -> Result<(), String> {
    for (func_idx, func) in vir.functions.iter().enumerate() {
        verify_function(func).map_err(|err| {
            format!(
                "VIR verification failed for function #{} ('{}'): {}",
                func_idx, func.name, err
            )
        })?;
    }

    Ok(())
}

pub fn verify_function(func: &VirFunction) -> Result<(), String> {
    let entry_idx = func.entry.0 as usize;
    if entry_idx >= func.blocks.len() {
        return Err(format!(
            "entry block {:?} does not exist (blocks.len() = {})",
            func.entry,
            func.blocks.len()
        ));
    }

    verify_block_params(func)?;
    verify_block_structure(func)?;
    verify_insts(func)?;

    Ok(())
}

fn verify_block_params(func: &VirFunction) -> Result<(), String> {
    for (block_idx, block) in func.blocks.iter().enumerate() {
        let block_id = BlockId(block_idx as u32);

        for param_id in &block.params {
            let param_idx = param_id.0 as usize;

            if param_idx >= func.block_params.len() {
                return Err(format!(
                    "block {:?} references invalid ParamId({})",
                    block_id, param_id.0
                ));
            }

            let param = &func.block_params[param_idx];

            if param.block != block_id {
                return Err(format!(
                    "block {:?} contains ParamId({}), but that param belongs to \
                     block {:?}",
                    block_id, param_id.0, param.block
                ));
            }
        }
    }

    Ok(())
}

fn verify_block_structure(func: &VirFunction) -> Result<(), String> {
    let mut inst_owner: Vec<Option<BlockId>> = vec![None; func.insts.len()];

    for (block_idx, block) in func.blocks.iter().enumerate() {
        let block_id = BlockId(block_idx as u32);

        if block.insts.is_empty() {
            return Err(format!("block {:?} is empty", block_id));
        }

        let mut saw_terminator = false;

        for inst_id in &block.insts {
            let inst_idx = inst_id.0 as usize;

            if inst_idx >= func.insts.len() {
                return Err(format!(
                    "block {:?} references invalid InstId({})",
                    block_id, inst_id.0
                ));
            }

            if saw_terminator {
                return Err(format!(
                    "block {:?} contains instruction {:?} after a terminator",
                    block_id, inst_id
                ));
            }

            if let Some(owner) = inst_owner[inst_idx] {
                return Err(format!(
                    "instruction {:?} appears in multiple blocks: {:?} and {:?}",
                    inst_id, owner, block_id
                ));
            }

            inst_owner[inst_idx] = Some(block_id);

            let inst = &func.insts[inst_idx];
            if is_terminator(inst) {
                saw_terminator = true;
            }
        }

        let last_inst_id = block.insts.last().copied().unwrap();
        let last_inst = &func.insts[last_inst_id.0 as usize];

        if !is_terminator(last_inst) {
            return Err(format!(
                "block {:?} does not end in a terminator",
                block_id
            ));
        }
    }

    for (inst_idx, owner) in inst_owner.iter().enumerate() {
        if owner.is_none() {
            return Err(format!(
                "instruction InstId({}) is not placed in any block",
                inst_idx
            ));
        }
    }

    Ok(())
}

fn verify_insts(func: &VirFunction) -> Result<(), String> {
    for (inst_idx, inst) in func.insts.iter().enumerate() {
        let inst_id = InstId(inst_idx as u32);

        match inst {
            Inst::Value(value_inst) => {
                verify_value_inst(func, inst_id, value_inst)?;
            }
            Inst::Effect(effect_inst) => {
                verify_effect_inst(func, inst_id, effect_inst)?;
            }
        }
    }

    Ok(())
}

fn verify_value_inst(
    func: &VirFunction,
    inst_id: InstId,
    inst: &ValueInst,
) -> Result<(), String> {
    match &inst.kind {
        ValueInstKind::Const { value } => {
            let actual_ty = value.ty();
            if actual_ty != inst.ty {
                return Err(format!(
                    "instruction {:?} is const {:?} but has result type {:?}",
                    inst_id, actual_ty, inst.ty
                ));
            }
        }

        ValueInstKind::Add { lhs, rhs }
        | ValueInstKind::Sub { lhs, rhs }
        | ValueInstKind::Mul { lhs, rhs }
        | ValueInstKind::Div { lhs, rhs } => {
            let lhs_ty = value_ty_checked(func, *lhs)?;
            let rhs_ty = value_ty_checked(func, *rhs)?;

            if lhs_ty != rhs_ty {
                return Err(format!(
                    "instruction {:?} has mismatched operand types: \
                     lhs={:?}, rhs={:?}",
                    inst_id, lhs_ty, rhs_ty
                ));
            }

            if *lhs_ty != inst.ty {
                return Err(format!(
                    "instruction {:?} result type {:?} does not match operand \
                     type {:?}",
                    inst_id, inst.ty, lhs_ty
                ));
            }

            if !is_numeric_ty(lhs_ty) {
                return Err(format!(
                    "instruction {:?} uses non-numeric type {:?} in arithmetic",
                    inst_id, lhs_ty
                ));
            }
        }

        ValueInstKind::Neg { value } => {
            let value_ty = value_ty_checked(func, *value)?;

            if *value_ty != inst.ty {
                return Err(format!(
                    "instruction {:?} neg result type {:?} does not match \
                     operand type {:?}",
                    inst_id, inst.ty, value_ty
                ));
            }

            if !is_numeric_ty(value_ty) {
                return Err(format!(
                    "instruction {:?} neg uses non-numeric type {:?}",
                    inst_id, value_ty
                ));
            }
        }
    }

    Ok(())
}

fn verify_effect_inst(
    func: &VirFunction,
    inst_id: InstId,
    inst: &EffectInst,
) -> Result<(), String> {
    match inst {
        EffectInst::Print { value } => {
            let _ = value_ty_checked(func, *value)?;
        }

        EffectInst::Return { value } => match (&func.sig.ret_ty, value) {
            (None, None) => {}

            (None, Some(v)) => {
                let ty = value_ty_checked(func, *v)?;
                return Err(format!(
                    "instruction {:?} returns value of type {:?}, but function \
                     returns no value",
                    inst_id, ty
                ));
            }

            (Some(ret_ty), None) => {
                return Err(format!(
                    "instruction {:?} returns no value, but function expects \
                     return type {:?}",
                    inst_id, ret_ty
                ));
            }

            (Some(ret_ty), Some(v)) => {
                let value_ty = value_ty_checked(func, *v)?;
                if value_ty != ret_ty {
                    return Err(format!(
                        "instruction {:?} returns value of type {:?}, but \
                         function expects {:?}",
                        inst_id, value_ty, ret_ty
                    ));
                }
            }
        }

        EffectInst::Branch {
            cond,
            then_block,
            then_args,
            else_block,
            else_args,
        } => {
            // Condition type must be bool.
            let cond_ty = value_ty_checked(func, *cond)?;
            if *cond_ty != TypeKind::Bool {
                return Err(format!(
                    "branch {:?} has non-bool condition type {:?}",
                    inst_id, cond_ty
                ));
            }

            verify_jump_args(func, inst_id, *then_block, then_args)?;
            verify_jump_args(func, inst_id, *else_block, else_args)?;
        }

        EffectInst::Jump { target, args } => {
            verify_jump_args(func, inst_id, *target, args)?;
        }
    }

    Ok(())
}

fn verify_jump_args(
    func: &VirFunction,
    inst_id: InstId,
    target: BlockId,
    args: &[ValueId],
) -> Result<(), String> {
    if (target.0 as usize) >= func.blocks.len() {
        return Err(format!(
            "instruction {:?} jumps to invalid block {:?}", inst_id, target
        ));
    }

    let block = &func.blocks[target.0 as usize];
    if block.params.len() != args.len() {
        return Err(format!(
            "instruction {:?} jumps to block {:?} passing {} args, \
             but block expects {}",
            inst_id,
            target,
            args.len(),
            block.params.len()
        ));
    }

    for (param_id, arg) in block.params.iter().zip(args.iter()) {
        let param = &func.block_params[param_id.0 as usize];
        let arg_ty = value_ty_checked(func, *arg)?;
        if *arg_ty != param.ty {
            return Err(format!(
                "instruction {:?} jump to {:?} passes arg of type {:?} \
                 to param {:?} of type {:?}",
                inst_id, target, arg_ty, param_id, param.ty
            ));
        }
    }

    Ok(())
}

fn value_ty_checked<'a>(
    func: &'a VirFunction,
    value: ValueId,
) -> Result<&'a TypeKind, String> {
    match value {
        ValueId::Inst(inst_id) => {
            let inst_idx = inst_id.0 as usize;

            if inst_idx >= func.insts.len() {
                return Err(format!(
                    "ValueId::Inst({}) refers to non-existent instruction",
                    inst_id.0
                ));
            }

            match &func.insts[inst_idx] {
                Inst::Value(value_inst) => Ok(&value_inst.ty),
                Inst::Effect(_) => Err(format!(
                    "ValueId::Inst({}) refers to an effect instruction, not a \
                     value-producing instruction",
                    inst_id.0
                )),
            }
        }

        ValueId::Param(param_id) => {
            let param_idx = param_id.0 as usize;

            if param_idx >= func.block_params.len() {
                return Err(format!(
                    "ValueId::Param({}) refers to non-existent block param",
                    param_id.0
                ));
            }

            Ok(&func.block_params[param_idx].ty)
        }
    }
}

fn is_terminator(inst: &Inst) -> bool {
    matches!(
        inst,
        Inst::Effect(EffectInst::Return { .. })
            | Inst::Effect(EffectInst::Branch { .. })
            | Inst::Effect(EffectInst::Jump { .. })
    )
}

fn is_numeric_ty(ty: &TypeKind) -> bool {
    matches!(
        ty,
        TypeKind::I8
            | TypeKind::I16
            | TypeKind::I32
            | TypeKind::I64
            | TypeKind::U8
            | TypeKind::U16
            | TypeKind::U32
            | TypeKind::U64
            | TypeKind::F32
            | TypeKind::F64
    )
}
