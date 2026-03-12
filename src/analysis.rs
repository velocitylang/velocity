use crate::grammar::{Expr, TypeBinding, TypeEnv, TypeKind};

fn infer_type(expr: &Expr, env: &TypeEnv, expected: Option<&TypeKind>) -> TypeKind {
    match expr {
        Expr::Negate(inner) => {
            let ty = infer_type(inner, env, expected);

            match ty {
                TypeKind::I8 | TypeKind::I16 | TypeKind::I32 | TypeKind::I64 |
                TypeKind::F32 | TypeKind::F64 => ty,
                TypeKind::U8 | TypeKind::U16 | TypeKind::U32 | TypeKind::U64 =>
                    panic!("Cannot negate unsigned type {:?}", ty),
                _ => panic!("Cannot negate {:?}", ty),
            }
        },
        Expr::String(_) => TypeKind::String,
        Expr::Bool(_) => TypeKind::Bool,
        Expr::NumberLiteral(n) => {
            match expected {
                Some(ty) => {
                    match ty {
                        TypeKind::I8 => { n.parse::<i8>().expect(&format!("'{n}' does not fit in i8")); },
                        TypeKind::I16 => { n.parse::<i16>().expect(&format!("'{n}' does not fit in i16")); },
                        TypeKind::I32 => { n.parse::<i32>().expect(&format!("'{n}' does not fit in i32")); },
                        TypeKind::I64 => { n.parse::<i64>().expect(&format!("'{n}' does not fit in i64")); },
                        TypeKind::U8 => { n.parse::<u8>().expect(&format!("'{n}' does not fit in u8")); },
                        TypeKind::U16 => { n.parse::<u16>().expect(&format!("'{n}' does not fit in u16")); },
                        TypeKind::U32 => { n.parse::<u32>().expect(&format!("'{n}' does not fit in u32")); },
                        TypeKind::U64 => { n.parse::<u64>().expect(&format!("'{n}' does not fit in u64")); },
                        TypeKind::F32 => { n.parse::<f32>().expect(&format!("'{n}' does not fit in f32")); },
                        TypeKind::F64 => { n.parse::<f64>().expect(&format!("'{n}' does not fit in f64")); },
                        _ => panic!("Expected numeric type, got {:?}", ty),
                    }
                    ty.clone()
                },
                None => TypeKind::I64,
            }
        },
        Expr::Var(name) => env.idents.get(name)
            .expect(&format!("Undefined variable '{name}'"))
            .ty.clone(),
        Expr::Add(left, right) => {
            let l = infer_type(left, env, expected);
            let r = infer_type(right, env, expected);
            if l != r {
                panic!("Cannot add {:?} and {:?}", l, r);
            }
            l
        },
        Expr::Sub(left, right) => {
            let l = infer_type(left, env, expected);
            let r = infer_type(right, env, expected);
            if l != r {
                panic!("Cannot subtract {:?} and {:?}", l, r);
            }
            l
        },
        Expr::Mul(left, right) => {
            let l = infer_type(left, env, expected);
            let r = infer_type(right, env, expected);
            if l != r {
                panic!("Cannot multiply {:?} and {:?}", l, r);
            }
            l
        },
        Expr::Div(left, right) => {
            let l = infer_type(left, env, expected);
            let r = infer_type(right, env, expected);
            if l != r {
                panic!("Cannot divide {:?} and {:?}", l, r);
            }
            l
        },
        _ => panic!("Cannot infer type for {:?}", expr),
    }
}

pub fn check_types(expr: Expr, env: &mut TypeEnv) {
    match &expr {
        Expr::Add(_, _) => {
            infer_type(&expr, env, None);
        },
        Expr::Sub(_, _) => {
            infer_type(&expr, env, None);
        },
        Expr::Mul(_, _) => {
            infer_type(&expr, env, None);
        },
        Expr::Div(_, _) => {
            infer_type(&expr, env, None);
        },
        Expr::Var(ident) => {
            let value = env.idents.get(ident);
            match value {
                Some(_) => {},
                _ => panic!("Identifier {ident} not found")
            }
        },
        Expr::LetDecl(ident, expr, ty) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let inferred_type = infer_type(&expr, env, ty.as_ref());

            if let Some(t) = ty {
                if inferred_type != *t {
                    panic!("Value for {:?} does not match declared type {:?}", ident, t);
                }
                env.idents.insert(String::from(ident), TypeBinding { ty: t.clone(), mutable: true });
            } else {
                env.idents.insert(String::from(ident), TypeBinding { ty: inferred_type, mutable: true });
            }
        },
        Expr::MakeDecl(ident, expr, ty) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let inferred_type = infer_type(&expr, env, ty.as_ref());

            if let Some(t) = ty {
                if inferred_type != *t {
                    panic!("Value for {:?} does not match declared type {:?}", ident, t);
                }
                env.idents.insert(String::from(ident), TypeBinding { ty: t.clone(), mutable: false });
            } else {
                env.idents.insert(String::from(ident), TypeBinding { ty: inferred_type, mutable: false });
            }
        },
        Expr::Print(expr) => {
            infer_type(&expr, env, None);
        },
        Expr::Reassign(ident, expr) => {
            let binding = env.idents.get(ident)
                .expect(&format!("Cannot reassign undefined identifier {ident}"));

            if !binding.mutable {
                panic!("Cannot reassign immutable identifier {ident}");
            }

            let expected_type = binding.ty.clone();
            let inferred_type = infer_type(&expr, env, Some(&expected_type));

            if inferred_type != expected_type {
                panic!("Value for {:?} does not match declared type {:?}", ident, expected_type);
            }

            env.idents.insert(String::from(ident), TypeBinding { ty: expected_type, mutable: true });
        },
        _ => {},
    }
}