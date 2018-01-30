pub mod cfg;
mod arch;
mod function;
mod program;
mod types;
mod symtbl;
mod util;

use clap::ArgMatches;
use super::parser::ast::*;
use self::function::Function;
use self::program::Program;
use self::cfg::CFG;
use self::cfg::inst::*;
use self::cfg::operand::{Operand};
use self::symtbl::{SymTbl, Symbol};

pub fn compile(ast: Ast, _args: &ArgMatches) -> Program {
    /*
    match args.value_of("o").unwrap_or("") {
        "1" => unimplemented!(),
        "2" => unimplemented!(),
        "3" => unimplemented!(),
        _ => ()
    }
    */
    let mut funcs = gather_funcs(&ast);
    let main = gather_main(&ast);
    funcs.insert(0, main);

    Program { funcs }
}

fn gather_funcs(ast: &Ast) -> Vec<Function> {
    let body = match *ast {
        Ast::Module { ref body } => body
    };
    let mut functions = Vec::new();

    for stmt in body.iter() {
        match *stmt {
            Statement::FunctionDef { .. } => {
                unimplemented!()
            },
            _ => ()
        }
    }

    functions
}

fn gather_main(ast: &Ast) -> Function {
    let body = match *ast {
        Ast::Module { ref body } => body
    };
    let mut cfg = CFG::new();
    let mut cur_block = cfg.entry_block.clone();

    for stmt in body.iter() {
        match *stmt {
            Statement::FunctionDef { .. } | Statement::ClassDef { .. } => (),
            _ => {
                cur_block = compile_stmt(&mut cfg, cur_block, stmt);
            }
        }
    }

    let exit_block = cfg.exit_block.clone();
    if cur_block != exit_block {
        cfg.connect_blocks(&cur_block, exit_block.clone());
        cfg.add_inst(&cur_block, Instruction::Branch(
            BranchStruct::new(None, exit_block.clone(), None)))
    }
    // Add the void return to the exit block
    cfg.add_inst(&exit_block, Instruction::Return(
        ReturnStruct { return_type: "void".to_string(), value: None }));
    Function { name: "main".to_string(),
        return_type: "void".to_string(), graph: cfg }
}

fn compile_stmts(cfg: &mut CFG, mut cur_block: String, stmts: &Vec<Statement>)
    -> String {
    for stmt in stmts.iter() {
        cur_block = compile_stmt(cfg, cur_block, stmt);
    }

    cur_block
}

fn compile_stmt(cfg: &mut CFG, cur_block: String, stmt: &Statement)
    -> String {
    match *stmt {
        Statement::Expr { .. } => compile_stmt_expr(cfg, cur_block, stmt),
        _ => unimplemented!()
    }
}

fn compile_stmt_expr(cfg: &mut CFG, cur_block: String, stmt: &Statement)
    -> String {
    let expr = match *stmt {
        Statement::Expr { ref value } => value,
        _ => unreachable!()
    };
    let reg = compile_expr(cfg, cur_block.clone(), expr);

    cur_block
}

fn compile_expr(cfg: &mut CFG, cur_block: String, expr: &Expression)
    -> Operand {
    match *expr {
        Expression::BinOp { .. } => compile_expr_binop(cfg, cur_block, expr),
        Expression::Num { ref n } => util::gen_imm_num(cfg, cur_block, n),
        _ => unimplemented!()
    }
}

fn compile_expr_binop(cfg: &mut CFG, cur_block: String, expr: &Expression)
    -> Operand {
    let (left, op, right) = match *expr {
        Expression::BinOp { ref left, ref op, ref right } => (left, op, right),
        _ => unreachable!()
    };

    let lft_oper = compile_expr(cfg, cur_block.clone(), left);
    let rht_oper = compile_expr(cfg, cur_block.clone(), right);
    util::gen_bin_inst(cfg, cur_block, op, lft_oper, rht_oper)
}
