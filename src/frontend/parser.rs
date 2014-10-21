
use super::ast::Stmt;

use self::toy::body;

pub fn parse_routine(routine_body_text: &str) -> Result<Vec<Stmt>,String> {
  return body(routine_body_text);
}

peg! toy(r#"

use super::super::ast::*;

#[pub]
body -> Vec<Stmt>
  = __ ss:(statement ** (__ ";" __)) __ {ss} ;

statement -> Stmt
  = "while" __ cond:expr "{" body:body "}" { While(cond, body) }
  / "if" __ cond:expr "{" ifbranch:body "}" __ elsebranch_opt:( "else" __ "{" branch:body "}" { branch } )? { Condition(cond, ifbranch, elsebranch_opt.unwrap_or_else(|| Vec::new())) }
  / lhs:identifier __ "<-" __ rhs:expr  { Assign(lhs, rhs) }

expr -> Expr
  = e:logicalUnaryExpr { e }

logicalUnaryExpr -> Expr
  = "not" __ e:logicalUnaryExpr __ { Not(box e) }
  / e:sumExpr { e }

sumExpr -> Expr
  = lhs:productExpr __ "+" __ rhs:sumExpr __ { Addition(box lhs, box rhs) }
  / lhs:productExpr __ "-" __ rhs:sumExpr __ { Subtraction(box lhs, box rhs) }
  / e:productExpr __ { e };

productExpr -> Expr
  = lhs:binBinExpr __ "*" __ rhs:productExpr __ { Multiplication(box lhs, box rhs) }
  / lhs:binBinExpr __ "/" __ rhs:productExpr __ { Division(box lhs, box rhs) }
  / lhs:binBinExpr __ "%" __ rhs:productExpr __ { Remainder(box lhs, box rhs) }
  / e:binBinExpr __ { e }

binBinExpr -> Expr
  = e:unBinExpr __ { e }

unBinExpr -> Expr
  = "!" __ e:unBinExpr __ { BinaryNot(box e) }
  / e:atomExpr __ { e }

atomExpr -> Expr
  = n:number __ { Constant(n) }
  / id:identifier __ { Variable(id) }
  / "(" __ e:expr ")" __ { e }

number -> int
  = [0-9_]+ { from_str::<int>(match_str).unwrap() }

identifier -> String
  = ([a-zA-Z_][a-zA-Z0-9_]*)!("while"/"if"/"else"/"not") { match_str.to_string() }

// Taken from https://github.com/kevinmehall/rust-peg/blob/master/src/grammar.rustpeg
__ = (whitespace / eol / comment)*

comment
  = singleLineComment
  / multiLineComment

singleLineComment
  = "//" (!eolChar .)*

multiLineComment
  = "/*" (!"*/" .)* "*/"

/* Modeled after ECMA-262, 5th ed., 7.3. */
eol
  = "\n"
  / "\r\n"
  / "\r"
  / "\u2028"
  / "\u2029"

eolChar
  = [\n\r\u2028\u2029]

/* Modeled after ECMA-262, 5th ed., 7.2. */
whitespace
  = [ \t\u00A0\uFEFF\u1680\u180E\u2000-\u200A\u202F\u205F\u3000] // \v\f removed

"#)



