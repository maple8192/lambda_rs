use crate::parser::expr::Expr;
use crate::parser::program::Program;
use std::cmp::max;
use std::collections::HashMap;

pub fn run(program: Program) {
    let mut expr = program.main;
    let mut id = max_id(&expr) + 1;
    let mut transformed = true;
    let mut id_map;
    let mut first = true;
    while transformed {
        transformed = false;
        id_map = HashMap::new();
        if !first {
            print!("= ");
        }
        first = false;
        println!("{}", show(&expr, &mut id_map));
        expr = calc(expr, &program.functions, &mut id, &mut transformed);
    }
}

fn calc(
    expr: Expr,
    functions: &HashMap<String, Expr>,
    id: &mut isize,
    transformed: &mut bool,
) -> Expr {
    if *transformed {
        return expr;
    }

    match expr {
        Expr::Lambda(var, var_id, expr) => Expr::Lambda(
            var,
            var_id,
            Box::new(calc(*expr, functions, id, transformed)),
        ),
        Expr::Application(lhs, rhs) => match *lhs {
            Expr::Lambda(_, var_id, expr) => {
                *transformed = true;
                beta_transform(*expr, var_id, *rhs, id)
            }
            _ => {
                let lhs = calc(*lhs, functions, id, transformed);
                let rhs = calc(*rhs, functions, id, transformed);
                Expr::Application(Box::new(lhs), Box::new(rhs))
            }
        },
        Expr::FuncCall(name) => {
            let mut id_map = HashMap::new();
            *transformed = true;
            function_expand(&functions[&name], id, &mut id_map)
        }
        _ => expr,
    }
}

fn beta_transform(expr: Expr, var_id: isize, applier: Expr, id: &mut isize) -> Expr {
    match expr {
        Expr::Lambda(var_name, lambda_var_id, expr) => Expr::Lambda(
            var_name,
            lambda_var_id,
            Box::new(beta_transform(*expr, var_id, applier, id)),
        ),
        Expr::Application(lhs, rhs) => Expr::Application(
            Box::new(beta_transform(*lhs, var_id, applier.clone(), id)),
            Box::new(beta_transform(*rhs, var_id, applier, id)),
        ),
        Expr::Variable(_, old_var_id) => {
            return if old_var_id == var_id {
                let mut id_map = HashMap::new();
                function_expand(&applier, id, &mut id_map)
            } else {
                expr
            }
        }
        _ => expr,
    }
}

fn function_expand(expr: &Expr, id: &mut isize, id_map: &mut HashMap<isize, isize>) -> Expr {
    match expr {
        Expr::Lambda(var, old_id, expr) => {
            let var_id = *id;
            *id += 1;
            id_map.insert(*old_id, var_id);
            Expr::Lambda(
                var.clone(),
                var_id,
                Box::new(function_expand(expr.as_ref(), id, id_map)),
            )
        }
        Expr::Application(lhs, rhs) => {
            let lhs = function_expand(lhs.as_ref(), id, id_map);
            let rhs = function_expand(rhs.as_ref(), id, id_map);
            Expr::Application(Box::new(lhs), Box::new(rhs))
        }
        Expr::FuncCall(name) => Expr::FuncCall(name.clone()),
        Expr::Variable(name, old_id) => {
            Expr::Variable(name.clone(), *id_map.get(old_id).unwrap_or(old_id))
        }
        Expr::Ident(name) => Expr::Ident(name.clone()),
    }
}

fn max_id(expr: &Expr) -> isize {
    match expr {
        Expr::Lambda(_, id, expr) => max(*id, max_id(expr.as_ref())),
        Expr::Application(lhs, rhs) => max(max_id(lhs.as_ref()), max_id(rhs.as_ref())),
        _ => -1,
    }
}

fn show(expr: &Expr, id_map: &mut HashMap<isize, String>) -> String {
    match expr {
        Expr::Lambda(var, id, expr) => {
            let mut new_var = var.clone();
            while id_map.iter().any(|x| *x.1 == new_var) {
                new_var.push('\'');
            }
            id_map.insert(*id, new_var.clone());
            let str = format!("\\{new_var}. {}", show(expr.as_ref(), id_map));
            id_map.remove(id);
            str
        }
        Expr::Application(lhs, rhs) => {
            let lhs = match **lhs {
                Expr::Lambda(..) => format!("({})", show(lhs, id_map)),
                _ => show(lhs.as_ref(), id_map),
            };
            let rhs = match **rhs {
                Expr::Lambda(..) | Expr::Application(..) => format!("({})", show(rhs, id_map)),
                _ => show(rhs.as_ref(), id_map),
            };
            format!("{lhs} {rhs}")
        }
        Expr::FuncCall(func_name) => format!("{func_name}"),
        Expr::Variable(_, var_id) => format!("{}", id_map[var_id]),
        Expr::Ident(ident) => format!("{ident}"),
    }
}
