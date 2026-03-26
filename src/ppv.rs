use crate::{grammar::TypeKind, vir::{Constant, EffectInst, Inst, InstId, ValueId, ValueInstKind, Vir, VirFunction}};

pub fn dump_vir(vir: &Vir) {
    for func in &vir.functions {
        dump_function(func);
    }
}

pub fn dump_function(func: &VirFunction) {
    println!("fn {}() {{", func.name);

    for (block_idx, block) in func.blocks.iter().enumerate() {
        println!("b{}:", block_idx);

        for inst_id in &block.insts {
            println!("  {}", format_inst(func, *inst_id));
        }
    }

    println!("}}");
}

fn format_inst(func: &VirFunction, inst_id: InstId) -> String {
    let inst = &func.insts[inst_id.0 as usize];

    match inst {
        Inst::Value(v) => {
            let lhs = format!("i{}:{}", inst_id.0, format_type(&v.ty));
            let rhs = format_value_inst_kind(&v.kind);
            format!("{lhs} = {rhs}")
        }
        Inst::Effect(e) => format_effect_inst(e),
    }
}

fn format_value_inst_kind(kind: &ValueInstKind) -> String {
    match kind {
        ValueInstKind::Const { value } => {
            format!("const {}", format_constant(value))
        }
        ValueInstKind::Add { lhs, rhs } => {
            format!("add {}, {}", format_value_id(*lhs), format_value_id(*rhs))
        }
        ValueInstKind::Sub { lhs, rhs } => {
            format!("sub {}, {}", format_value_id(*lhs), format_value_id(*rhs))
        }
        ValueInstKind::Mul { lhs, rhs } => {
            format!("mul {}, {}", format_value_id(*lhs), format_value_id(*rhs))
        }
        ValueInstKind::Div { lhs, rhs } => {
            format!("div {}, {}", format_value_id(*lhs), format_value_id(*rhs))
        }
        ValueInstKind::Neg { value } => {
            format!("neg {}", format_value_id(*value))
        }
    }
}

fn format_effect_inst(inst: &EffectInst) -> String {
    match inst {
        EffectInst::Print { value } => {
            format!("print {}", format_value_id(*value))
        }
        EffectInst::Return { value } => match value {
            Some(v) => format!("return {}", format_value_id(*v)),
            None => "return".to_string(),
        }
        EffectInst::Branch {
            cond,
            then_block,
            then_args,
            else_block,
            else_args,
        } => {
            let then_args_str = format_block_args(then_args);
            let else_args_str = format_block_args(else_args);

            format!(
                "branch {}, b{}{}, b{}{}",
                format_value_id(*cond),
                then_block.0,
                then_args_str,
                else_block.0,
                else_args_str
            )
        }

        EffectInst::Jump { target, args } => {
            let args_str = format_block_args(args);
            format!("jump b{}{}", target.0, args_str)
        }
    }
}

fn format_value_id(value: ValueId) -> String {
    match value {
        ValueId::Inst(inst_id) => format!("i{}", inst_id.0),
        ValueId::Param(param_id) => format!("p{}", param_id.0),
    }
}

fn format_constant(value: &Constant) -> String {
    match value {
        Constant::I8(v) => v.to_string(),
        Constant::I16(v) => v.to_string(),
        Constant::I32(v) => v.to_string(),
        Constant::I64(v) => v.to_string(),
        Constant::U8(v) => v.to_string(),
        Constant::U16(v) => v.to_string(),
        Constant::U32(v) => v.to_string(),
        Constant::U64(v) => v.to_string(),
        Constant::F32(v) => v.to_string(),
        Constant::F64(v) => v.to_string(),
        Constant::Bool(v) => v.to_string(),
        Constant::String(s) => format!("{s:?}"),
    }
}

fn format_block_args(args: &[ValueId]) -> String {
    if args.is_empty() {
        String::new()
    } else {
        let joined = args
            .iter()
            .map(|a| format_value_id(*a))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", joined)
    }
}

fn format_type(ty: &TypeKind) -> &'static str {
    match ty {
        TypeKind::I8 => "i8",
        TypeKind::I16 => "i16",
        TypeKind::I32 => "i32",
        TypeKind::I64 => "i64",
        TypeKind::U8 => "u8",
        TypeKind::U16 => "u16",
        TypeKind::U32 => "u32",
        TypeKind::U64 => "u64",
        TypeKind::Unit => "unit",
        TypeKind::F32 => "f32",
        TypeKind::F64 => "f64",
        TypeKind::Bool => "bool",
        TypeKind::String => "string",
    }
}
