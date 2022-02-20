use crate::parser::ComplexToken;
use crate::parser::ComplexToken::*;
use crate::parser::Expression;

fn ReachLine(cline: &mut usize, line: usize) -> String {
    let mut result = String::new();
    for _ in *cline..line {
        result += "\n";
        *cline += 1;
    }
    result
}

fn CompileList<T>(list: Vec<T>, tostring: &mut impl FnMut(T) -> String) -> String {
    let mut result = String::new();
    let end = list.iter().count();
    let mut start = 0usize;
    for element in list {
        result += &(tostring(element));
        start += 1;
        if start < end {
            result += ", "
        }
    }
    result
}

fn CompileIdentifiers(names: Vec<String>) -> String {
    CompileList(names, &mut |name| {name})
}

fn CompileExpressions(cline: &mut usize, values: Vec<Expression>) -> String {
    CompileList(values, &mut |expr| {CompileExpression(cline, expr)})
}

fn CompileExpression(cline: &mut usize, expr: Expression) -> String {
    let mut result = String::new();
    for t in expr {
        match t {
            SYMBOL {lexeme, line} => {
                *cline += lexeme.matches("\n").count();
                result += &format!("{}{} ", ReachLine(cline, line), lexeme);
            }
            _ => {panic!("Unexpected ComplexToken found")}
        }
    }
    result
}

pub fn CompileTokens(ctokens: Vec<ComplexToken>) -> Result<String, String> {
    let mut result = String::new();
    let cline = &mut 1usize;
    for t in ctokens.into_iter() {
        match t {
            VARIABLE {local, names, values, line} => {
                let mut pre = ReachLine(cline, line);
                if local {pre += "local "}
                let names = CompileIdentifiers(names);
                let values = CompileExpressions(cline, values);
                result += &format!("{}{} = {};", pre, names, values);
            }
            _ => {panic!("Unexpected ComplexToken found")}
        }
    }
    Ok(result)
}