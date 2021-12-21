extern crate proc_macro;
use proc_macro::TokenStream;

use rust_ad_core::utils::*;
use rust_ad_core::*;
use syn::spanned::Spanned;

use std::collections::HashMap;

pub fn add_insert(map: &mut HashMap<String, usize>, string: String) -> usize {
    if let Some(val) = map.get_mut(&string) {
        let c = *val;
        *val += 1;
        c
    } else {
        map.insert(string, 1);
        0
    }
}

pub fn reverse_derivative(stmt: &syn::Stmt) -> Option<syn::Stmt> {
    if let syn::Stmt::Local(local) = stmt {
        // eprintln!("local: {:#?}",local);
        // panic!("just stop here");
        if let Some(init) = &local.init {
            let init_expr = &*init.1;
            if let syn::Expr::Binary(bin_expr) = init_expr {
                return Some(match bin_expr.op {
                    syn::BinOp::Add(_) => reverse_add(&stmt),
                    syn::BinOp::Sub(_) => reverse_sub(&stmt),
                    syn::BinOp::Mul(_) => reverse_mul(&stmt),
                    syn::BinOp::Div(_) => reverse_div(&stmt),
                    _ => panic!("Uncovered operation"),
                });
            } else if let syn::Expr::Macro(macro_expr) = init_expr {
                if macro_expr
                    .mac
                    .path
                    .segments
                    .last()
                    .expect("reverse empty macro")
                    .ident
                    .to_string()
                    == "dup"
                {
                    let macro_token_stream =
                        proc_macro::TokenStream::from(macro_expr.mac.tokens.clone())
                            .into_iter()
                            .collect::<Vec<_>>();
                    let ident = macro_token_stream[0]
                        .ident()
                        .expect("reverse: not ident")
                        .to_string();
                    let num: usize = macro_token_stream[2]
                        .literal()
                        .expect("reverse: not literal")
                        .to_string()
                        .parse()
                        .unwrap();
                    let token_str = format!(
                        "let {} = {};",
                        der!(ident),
                        (0..num)
                            .map(|c| format!("{}{}", der!(ident), c))
                            .intersperse(String::from("+"))
                            .collect::<String>()
                    );
                    let der_dup_stmt: syn::Stmt =
                        syn::parse_str(&token_str).expect("reverse: failed to parse dup");
                    // eprintln!("der_dup_stmt: {:?}", der_dup_stmt);
                    return Some(der_dup_stmt);
                }
            }
        }
    }
    None
}
fn reverse_add(stmt: &syn::Stmt) -> syn::Stmt {
    let local = stmt.local().expect("reverse_add: not local");
    let init = local.init.as_ref().unwrap();
    let init_expr = &*init.1;
    let bin_expr = init_expr.binary().expect("reverse_add: not binary");
    let lis = local
        .pat
        .ident()
        .expect("reverse_add: not ident")
        .ident
        .to_string();

    let (a, b): (String, String) = match (&*bin_expr.left, &*bin_expr.right) {
        (syn::Expr::Path(expr_path_l), syn::Expr::Path(expr_path_r)) => (
            der!(expr_path_l.path.segments[0].ident.to_string()),
            der!(expr_path_r.path.segments[0].ident.to_string()),
        ),
        (syn::Expr::Path(expr_path_l), syn::Expr::Lit(_)) => (
            der!(expr_path_l.path.segments[0].ident.to_string()),
            String::from("_"),
        ),
        (syn::Expr::Lit(_), syn::Expr::Path(expr_path_r)) => (
            String::from("_"),
            der!(expr_path_r.path.segments[0].ident.to_string()),
        ),
        _ => panic!("reverse_add: Uncovered `syn::BinOp::Add(_)` binary expression combination"),
    };
    let stmt_str = format!("let ({},{}) = rust_ad::dup!({},2);", a, b, lis);
    let new_stmt: syn::Stmt = syn::parse_str(&stmt_str).expect("reverse_add: parse fail");
    new_stmt
}
fn reverse_sub(stmt: &syn::Stmt) -> syn::Stmt {
    let local = stmt.local().expect("reverse_sub: not local");
    let init = local.init.as_ref().unwrap();
    let init_expr = &*init.1;
    let bin_expr = init_expr.binary().expect("reverse_sub: not binary");
    let lis = local
        .pat
        .ident()
        .expect("reverse_sub: not ident")
        .ident
        .to_string();

    let (a, b): (String, String) = match (&*bin_expr.left, &*bin_expr.right) {
        (syn::Expr::Path(expr_path_l), syn::Expr::Path(expr_path_r)) => (
            der!(expr_path_l.path.segments[0].ident.to_string()),
            format!("-{}", der!(expr_path_r.path.segments[0].ident.to_string())),
        ),
        (syn::Expr::Path(expr_path_l), syn::Expr::Lit(_)) => (
            der!(expr_path_l.path.segments[0].ident.to_string()),
            String::from("_"),
        ),
        (syn::Expr::Lit(_), syn::Expr::Path(expr_path_r)) => (
            String::from("_"),
            format!("-{}", der!(expr_path_r.path.segments[0].ident.to_string())),
        ),
        _ => panic!("reverse_sub: Uncovered `syn::BinOp::Sub(_)` binary expression combination"),
    };
    let stmt_str = format!("let ({},{}) = rust_ad::dup!({},2);", a, b, lis);
    let new_stmt: syn::Stmt = syn::parse_str(&stmt_str).expect("reverse_sub: parse fail");
    new_stmt
}
fn reverse_mul(stmt: &syn::Stmt) -> syn::Stmt {
    let local = stmt.local().expect("reverse_mul: not local");
    let init = local.init.as_ref().unwrap();
    let init_expr = &*init.1;
    let bin_expr = init_expr.binary().expect("reverse_mul: not binary");
    let lis = local
        .pat
        .ident()
        .expect("reverse_mul: not ident")
        .ident
        .to_string();

    let stmt_str = match (&*bin_expr.left, &*bin_expr.right) {
        (syn::Expr::Path(expr_path_l), syn::Expr::Path(expr_path_r)) => {
            let (l, r) = (
                expr_path_l.path.segments[0].ident.to_string(),
                expr_path_r.path.segments[0].ident.to_string(),
            );
            format!(
                "let ({},{}) = ({}*{},{}*{});",
                der!(l),
                der!(r),
                r,
                lis,
                l,
                lis
            )
        }
        (syn::Expr::Path(expr_path_l), syn::Expr::Lit(expr_lit_r)) => {
            let (l, r) = (
                expr_path_l.path.segments[0].ident.to_string(),
                expr_lit_r
                    .lit
                    .float()
                    .expect("reverse_mul: right not literal")
                    .to_string(),
            );
            format!("let {} = {}*{};", der!(l), r, lis)
        }
        (syn::Expr::Lit(expr_lit_l), syn::Expr::Path(expr_path_r)) => {
            let (l, r) = (
                expr_lit_l
                    .lit
                    .float()
                    .expect("reverse_mul: left not literal")
                    .to_string(),
                expr_path_r.path.segments[0].ident.to_string(),
            );
            format!("let {} = {}*{};", der!(r), l, lis)
        }
        _ => panic!("reverse_mul: Uncovered `syn::BinOp::Mul(_)` binary expression combination"),
    };
    let new_stmt: syn::Stmt = syn::parse_str(&stmt_str).expect("reverse_mul: parse fail");
    new_stmt
}
fn reverse_div(stmt: &syn::Stmt) -> syn::Stmt {
    let local = stmt.local().expect("reverse_div: not local");
    let init = local.init.as_ref().unwrap();
    let init_expr = &*init.1;
    let bin_expr = init_expr.binary().expect("reverse_div: not binary");
    let lis = local
        .pat
        .ident()
        .expect("reverse_div: not ident")
        .ident
        .to_string();

    let stmt_str = match (&*bin_expr.left, &*bin_expr.right) {
        (syn::Expr::Path(expr_path_l), syn::Expr::Path(expr_path_r)) => {
            let (l, r) = (
                expr_path_l.path.segments[0].ident.to_string(),
                expr_path_r.path.segments[0].ident.to_string(),
            );
            format!(
                "let ({},{}) = ({}/{},{}*{});",
                der!(l),
                der!(r),
                lis,
                r,
                l,
                lis
            )
        }
        (syn::Expr::Path(expr_path_l), syn::Expr::Lit(expr_lit_r)) => {
            let (l, r) = (
                expr_path_l.path.segments[0].ident.to_string(),
                expr_lit_r
                    .lit
                    .float()
                    .expect("reverse_div: right not literal")
                    .to_string(),
            );
            format!("let {} = {}/{};", der!(l), lis, r)
        }
        (syn::Expr::Lit(expr_lit_l), syn::Expr::Path(expr_path_r)) => {
            let (l, r) = (
                expr_lit_l
                    .lit
                    .float()
                    .expect("reverse_div: left not literal")
                    .to_string(),
                expr_path_r.path.segments[0].ident.to_string(),
            );
            format!("let {} = {}*{};", der!(r), l, lis)
        }
        _ => panic!("Uncovered `syn::BinOp::Mul(_)` binary expression combination"),
    };
    let new_stmt: syn::Stmt = syn::parse_str(&stmt_str).expect("reverse_div: parse fail");
    new_stmt
}

/// Validates and updates function signature.
pub fn reverse_update_signature(function: &mut syn::ItemFn) -> Result<syn::Stmt, TokenStream> {
    // If there is return statement, return user code, this will leader to compile error about no return function.
    match &mut function.sig.output {
        syn::ReturnType::Type(_, return_type_box) => {
            // eprintln!("here 1");
            if let Some(mut last_stmt) = function.block.stmts.pop() {
                // eprintln!("here 2: {:#?}",last_stmt);
                if last_stmt.is_semi() {
                    let expr = last_stmt
                        .semi_mut()
                        .expect("reverse_update_signature: not semi");
                    // eprintln!("here 3");
                    if expr.is_return() {
                        let expr_return = expr
                            .return_mut()
                            .expect("reverse_update_signature: not return");
                        // eprintln!("here 4");
                        if expr_return.expr.is_some() {
                            let return_expr = expr_return.expr.as_mut().unwrap();
                            // eprintln!("here 5");
                            if return_expr.is_path() {
                                // eprintln!("here 6");

                                // Updates function output signature.
                                // ---------------------------------------
                                let return_type = &mut **return_type_box;
                                let return_type_ident_string = &return_type
                                    .path()
                                    .expect("reverse_update_signature: not path")
                                    .path
                                    .segments[0]
                                    .ident
                                    .to_string();
                                let function_input_types = function
                                    .sig
                                    .inputs
                                    .iter()
                                    .map(|fn_arg| {
                                        fn_arg
                                            .typed()
                                            .expect("reverse_update_signature: arg not typed 1")
                                            .ty
                                            .path()
                                            .expect("reverse_update_signature: arg not path 1")
                                            .path
                                            .segments[0]
                                            .ident
                                            .to_string()
                                    })
                                    .intersperse(String::from(","))
                                    .collect::<String>();
                                let output = format!(
                                    "({},{})",
                                    return_type_ident_string, function_input_types
                                );
                                let new_rtn: syn::Type = syn::parse_str(&output).unwrap();
                                *return_type = new_rtn;
                                // Updates return statement.
                                // ---------------------------------------
                                let out = return_expr
                                    .path()
                                    .expect("reverse_update_signature: not path")
                                    .path
                                    .segments[0]
                                    .ident
                                    .to_string();
                                // Iter over idents of inputs.
                                let input_idents_iter = function.sig.inputs.iter().map(|fn_arg| {
                                    &fn_arg
                                        .typed()
                                        .expect("reverse_update_signature: arg not typed 2")
                                        .pat
                                        .ident()
                                        .expect("reverse_update_signature: arg not path 2")
                                        .ident
                                });
                                let inputs_output_str = input_idents_iter
                                    .map(|ident| format!("{},", der!(ident)))
                                    .collect::<String>();
                                let return_string = format!("({},{})", out, inputs_output_str);
                                let return_tuple: syn::Expr =
                                    syn::parse_str(&return_string).expect("unique 3");
                                expr_return.expr = Some(Box::new(return_tuple));
                                // Updates function input signature.
                                // ---------------------------------------
                                let new_fn_arg_str =
                                    format!("{}: {}", der!(out), return_type_ident_string);
                                let new_fn_arg: syn::FnArg =
                                    syn::parse_str(&new_fn_arg_str).unwrap();
                                function.sig.inputs.push(new_fn_arg);

                                return Ok(last_stmt);
                            }
                        }
                    }
                }
            }
            // If return statement does not match the conditions, then simply returning the function should give the user an error.
            Err(TokenStream::from(quote::quote! { #function }))
        }
        syn::ReturnType::Default => Err(TokenStream::from(quote::quote_spanned! {
            function.sig.span() => compile_error!("Expected return type `f32`");
        })),
    }
}
