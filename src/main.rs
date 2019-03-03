#[macro_use] extern crate maplit;

mod scanner;
mod lexer;
mod interp;

use lexer::Lexer;
use interp::Interpreter;

fn main() {
	let mut lex = Lexer::new(r#"
let someVar = 10;
someVar += 100;
"#.to_owned());
	match lex.lex() {
		Err(e) => println!("{}", e),
		Ok(v) => {
			let mut ip = Interpreter::new(v);
			ip.parse();
		}
	}
}
