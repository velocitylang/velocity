use crate::grammar::{Expr, Stmt, TypeBinding, TypeEnv, TypeKind};

fn infer_type(expr: &Expr, env: &TypeEnv, expected: Option<&TypeKind>) -> TypeKind {
    match expr {
        Expr::Array(items) => {
            if items.is_empty() {
                panic!("Cannot infer type of empty array");
            }

            let expected_elem_ty = match expected {
                Some(TypeKind::FixedArray(elem_ty, size)) => {
                    if *size != items.len() {
                        panic!(
                            "Expected array of size {}, found array of size {}",
                            size,
                            items.len()
                        );
                    }
                    Some(elem_ty.as_ref())
                }
                Some(other) => panic!("Expected array type, got {:?}", other),
                None => None,
            };

            let first_item_ty = infer_type(&items[0], env, expected_elem_ty);

            for item in &items[1..] {
                let item_ty = infer_type(item, env, expected_elem_ty);
                if item_ty != first_item_ty {
                    panic!(
                        "Array items must be the same type. Should be {:?} but found {:?}",
                        first_item_ty, item_ty
                    );
                }
            }

            if expected_elem_ty == None {
                return TypeKind::Array(Box::new(first_item_ty))
            } else {
                return TypeKind::FixedArray(Box::new(first_item_ty), items.len())
            }
        }
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
        Expr::NumberLiteral(n) => match expected {
            Some(ty) => match ty {
                TypeKind::I8 => {
                    n.parse::<i8>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in i8"));
                    TypeKind::I8
                }
                TypeKind::I16 => {
                    n.parse::<i16>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in i16"));
                    TypeKind::I16
                }
                TypeKind::I32 => {
                    n.parse::<i32>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in i32"));
                    TypeKind::I32
                }
                TypeKind::I64 => {
                    n.parse::<i64>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in i64"));
                    TypeKind::I64
                }
                TypeKind::U8 => {
                    n.parse::<u8>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in u8"));
                    TypeKind::U8
                }
                TypeKind::U16 => {
                    n.parse::<u16>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in u16"));
                    TypeKind::U16
                }
                TypeKind::U32 => {
                    n.parse::<u32>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in u32"));
                    TypeKind::U32
                }
                TypeKind::U64 => {
                    n.parse::<u64>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in u64"));
                    TypeKind::U64
                }
                TypeKind::F32 => {
                    n.parse::<f32>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in f32"));
                    TypeKind::F32
                }
                TypeKind::F64 => {
                    n.parse::<f64>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in f64"));
                    TypeKind::F64
                }
                TypeKind::Unit => {
                    n.parse::<i64>()
                        .unwrap_or_else(|_| panic!("'{n}' does not fit in i64"));
                    TypeKind::I64
                }
                _ => panic!("Expected numeric type, got {:?}", ty),
            },
            None => {
                n.parse::<i64>()
                    .unwrap_or_else(|_| panic!("'{n}' does not fit in i64"));
                TypeKind::I64
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

            let final_type = match ty.as_ref() {
                Some(declared) => resolve_declared_type(declared, &inferred_type),
                None => inferred_type,
            };

            *ty = Some(final_type.clone());

            env.idents.insert(
                ident.to_string(),
                TypeBinding {
                    ty: final_type,
                    mutable: *mutable,
                },
            );
        }
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

fn resolve_declared_type(declared: &TypeKind, inferred: &TypeKind) -> TypeKind {
    match (declared, inferred) {
        (TypeKind::Unit, other) => other.clone(),

        (TypeKind::FixedArray(decl_elem, decl_size), TypeKind::FixedArray(inf_elem, inf_size)) => {
            if decl_size != inf_size {
                panic!(
                    "Array size mismatch: declared {:?}, inferred {:?}",
                    declared, inferred
                );
            }

            TypeKind::FixedArray(
                Box::new(resolve_declared_type(decl_elem, inf_elem)),
                *decl_size,
            )
        }

        (a, b) if a == b => a.clone(),

        _ => {
            panic!(
                "Declared type {:?} does not match inferred type {:?}",
                declared, inferred
            );
        }
    }
}
