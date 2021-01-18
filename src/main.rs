extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "risp.pest"]
struct RISPParser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

// #[derive(Debug, Clone, PartialEq)]
// struct BinOp {
// 	op: String,
// 	a: Num,
// 	b: Num,
// }

#[derive(Debug, Clone, PartialEq)]
enum LVal {
	Num(i64),
	Sym(String),
	SExpr(Vec<Box<LVal>>)
}

fn eval_op(op: &str) {
	match op {
		"+" | "add" => {
            println!("builtin_op: Add");
        }
        "-" | "sub" => {
            println!("builtin_op: Subtract");
        }
        "*" | "mul" => {
            println!("builtin_op: Multiply");
        }
        "/" | "div" => {
			println!("builtin_op: Divide");
        }

		"defn" => {
			println!("builtin_op: Function Def.");
        }

		"=" => {
			println!("builtin_op: Equals");
        }

		"!" => {
			println!("builtin_op: Not Equals");
        }

		">" => {
			println!("builtin_op: Greater Than");
        }

		"<" => {
			println!("builtin_op: Less Than");
        }

		_ => {
			println!("unknown!");
		}
	}
}

fn is_bracket_or_eoi(parsed: &Pair<Rule>) -> bool {
    if parsed.as_rule() == Rule::EOI {
        return true;
    }
    let c = parsed.as_str();
    c == "(" || c == ")" || c == "{" || c == "}"
}

fn parse_sexpr(line: Pair<Rule>, mut invoke: i64, parent: &mut LVal) {
	println!("Invocation: {}", invoke);
	println!("Rule:   {:?}", line.as_rule());
	println!("Line:   {}", line.as_str());

	
	for child in line.into_inner() {
		match child.as_rule() {
			Rule::symbol => {
				println!("Found Symbol!");
				println!("");
				let symb = Box::new(LVal::Sym(child.as_str().to_string()));
				match parent {
					LVal::SExpr(list) => list.push(symb),
					_ => println!("UH!"),
				}
			}

			Rule::num => {
				println!("Found Num!");
				println!("");
				let n = child.as_str().parse::<i64>().unwrap();
				let num = Box::new(LVal::Num(n));
				match parent {
					LVal::SExpr(list) => list.push(num),
					_ => println!("UH!"),
				}
			}

			Rule::sexpr => {
				println!("Found SExpr!");
				println!("");
				let mut sexpr = Box::new(LVal::SExpr(Vec::new()));
				parse_sexpr(child, invoke, &mut sexpr);

				match parent {
					LVal::SExpr(list) => list.push(sexpr),
					_ => println!("UH!"),
				}
			}
			_ => {
				println!("");
				invoke += 1;
				parse_sexpr(child, invoke, parent);
			}
		}

	}
}

fn eval_sexpr(sexpr: &LVal) {
	match sexpr {
		LVal::SExpr(list) => {
			let top = &*list[0];
			//list.remove(0);
			match top {
				LVal::Sym(sym) => {
					let op = sym.as_str();
					eval_op(op);
				}
				_ => {
					// Return an Error here
					println!("unsupported SExpr");
				}
			}
		}

		_ => println!("Not an SExpr")
	}
}

fn main() {
	println!("Welcome to RISP!");
	let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
				let risp = RISPParser::parse(Rule::sexpr, line.as_str())
					.unwrap_or_else(|e| panic!("{}", e));
				let mut sexpr = Box::new(LVal::SExpr(Vec::new()));
				for expr in risp {
					parse_sexpr(expr, 0, &mut sexpr);
				}

				// match *sexpr {
				// 	LVal::SExpr(list) => {
				// 		for s in list {
				// 			println!("{:?}", s);
				// 		}

				// 	}
				// 	_ => println!("UH!")
				// }
				eval_sexpr(&sexpr)
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
