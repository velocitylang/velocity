use crate::grammar::{Expr, Stmt, TypeBinding, TypeEnv, TypeKind};

fn infer_type(expr: &Expr, env: &TypeEnv, expected: Option<&TypeKind>) -> TypeKind {
    match expr {
        Expr::If { condition, then_branch, else_branch } => {
            let cond_ty = infer_type(condition, env, Some(&TypeKind::Bool));
            if cond_ty != TypeKind::Bool {
                panic!("If-condition must be Bool, got {:?}", cond_ty);
            }

            let then_ty = infer_type(then_branch, env, expected);

            match else_branch {
                Some(else_expr) => {
                    let else_ty = infer_type(else_expr, env, expected);
                    if then_ty == else_ty {
                        then_ty  // if both equal, the whole if expression yields this type
                    } else {
                        panic!(
                            "Type mismatch in if branches: then = {:?}, else = {:?}",
                            then_ty, else_ty
                        );
                    }
                }
                None => {
                    // No else
                    TypeKind::Unit
                }
            }
        },
        Expr::Block(stmts) => {
            let mut ty = TypeKind::Unit;
            for stmt in stmts {
                ty = match stmt {
                    Stmt::ExprStmt(expr) => infer_type(expr, env, None),
                    Stmt::Let(_, expr, _, _) => infer_type(expr, env, None),
                    Stmt::Reassign(_, expr) => infer_type(expr, env, None),
                    Stmt::Return(expr) => return infer_type(expr, env, None),
                    Stmt::Print(expr) => { infer_type(expr, env, None); TypeKind::Unit },
                };
            }

            ty
        },
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

pub fn check_stmt_types(stmt: &mut Stmt, env: &mut TypeEnv) {
    match stmt {
        Stmt::ExprStmt(Expr::Var(ident)) => {
            let value = env.idents.get(ident);
            match value {
                Some(_) => {},
                _ => panic!("Identifier {ident} not found")
            }
        },
        Stmt::ExprStmt(expr) => {
            infer_type(&expr, env, None);
        },
        Stmt::Let(ident, expr, mutable, ty) => {
            if env.idents.contains_key(ident) {
                panic!("Cannot redeclare existing identifier {}", ident);
            }

            let inferred_type = infer_type(expr, env, ty.as_ref());

            if ty.is_none() {
                *ty = Some(inferred_type.clone());
            }

            if let Some(t) = ty {
                if inferred_type != *t {
                    panic!("Value for {:?} does not match declared type {:?}", ident, t);
                }
                env.idents.insert(ident.to_string(), TypeBinding { ty: t.clone(), mutable: *mutable });
            } else {
                env.idents.insert(ident.to_string(), TypeBinding { ty: inferred_type, mutable: *mutable });
            }
        },
        Stmt::Print(expr) => {
            infer_type(&expr, env, None);
        },
        Stmt::Reassign(ident, expr) => {
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

            env.idents.insert(ident.to_string(), TypeBinding { ty: expected_type, mutable: true });
        },
        Stmt::Return(expr) => {
            infer_type(&expr, env, None);
        }
    }
}