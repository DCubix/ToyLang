use crate::scanner::Scanner;
use std::i64;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinKeyword {
	Let = 0,
	Const,
	If,
	Else,
	Func,
	Return,
	Break,
	Continue,
	While,
	Do,
	In,
	For,
	Has,
	True,
	False
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Identifier(String),
	Keyword(BuiltinKeyword),
	Paren(char),
	Symbol(String),
	Number(f64),
	Str(String),
	Semi,
	EOF
}

pub struct Lexer {
	input: String
}

impl Lexer {
	pub fn new(input: String) -> Lexer {
		Lexer {
			input: input
		}
	}

	pub fn lex(&mut self) -> Result<Vec<Token>, String> {
		let KEYWORDS: HashMap<&str, BuiltinKeyword> = hashmap!{
			"let" => BuiltinKeyword::Let,
			"const" => BuiltinKeyword::Const,
			"if" => BuiltinKeyword::If,
			"else" => BuiltinKeyword::Else,
			"func" => BuiltinKeyword::Func,
			"break" => BuiltinKeyword::Break,
			"return" => BuiltinKeyword::Return,
			"continue" => BuiltinKeyword::Continue,
			"while" => BuiltinKeyword::While,
			"do" => BuiltinKeyword::Do,
			"in" => BuiltinKeyword::In,
			"for" => BuiltinKeyword::For,
			"has" => BuiltinKeyword::Has,
			"true" => BuiltinKeyword::True,
			"false" => BuiltinKeyword::False
		};

		let mut prev = '\0';
		let mut res: Vec<Token> = Vec::new();
		let mut sc = Scanner::from_str(&self.input);

		while sc.has_next() {
			let c = sc.current();
			match c {
				'0'...'9' | '.' => {
					let num = sc.eat_until_cond(|ch| {
						!ch.is_digit(10) && ch != '.' &&
						match ch {
							'a'...'f' | 'A'...'F' => false,
							_ => true
						}
					});
					let num_f: f64 =
						if num.to_lowercase().starts_with("0x") {
							let n = match i64::from_str_radix(num.to_lowercase().trim_start_matches("0x"), 16) {
								Ok(n) => n,
								Err(e) => return Err(format!("invalid number '{}', {}", num, e))
							};
							n as f64
						} else {
							match num.parse::<f64>() {
								Ok(n) => n,
								Err(e) => return Err(format!("invalid number '{}', {}", num, e))
							}
						};
					res.push(Token::Number(num_f));
				},
				'a'...'z' | 'A'...'Z' | '_' => {
					let id = sc.eat_until_cond(|ch| {
						!ch.is_alphabetic() && !ch.is_digit(10) && ch != '_'
					});
					let id_str = id.as_str();

					if KEYWORDS.contains_key(id_str) {
						res.push(Token::Keyword(KEYWORDS[id_str]));
					} else {
						res.push(Token::Identifier(id));
					}
				},
				'(' | ')' | '[' | ']' | '{' | '}' => res.push(Token::Paren(sc.eat().unwrap())),
				',' => { sc.eat(); res.push(Token::Symbol(",".to_owned())); },
				'+' | '-' | '*' | '/' | '=' | '^' | '|' | '&' | '~' | ':' | '%' | '<' | '>' | '?' => {
					let mut val = String::new();
					loop {
						match sc.current() {
							'+' | '-' | '*' | '/' | '=' | '^' | '|' | '&' | '~' | ':' | '%' | '<' | '>' | '?' => {}
							_ => { break; }
						}
						val.push(sc.eat().unwrap());
					}
					res.push(Token::Symbol(val));
				},
				';' => { sc.eat(); res.push(Token::Semi); },
				'"' => {
					sc.eat();

					let mut val = String::new();
					while sc.current() != '"' {
						let c = sc.eat().unwrap();
						match c {
							'\\' => {
								let k = sc.eat().unwrap_or('\0');
								match k {
									'n' => val.push('\n'),
									't' => val.push('\t'),
									'r' => val.push('\r'),
									'\'' => val.push('\''),
									'"' => val.push('\"'),
									'\\' => val.push('\\'),
									_ => return Err(format!("unknown escape sequence \\{}", sc.current()))
								}
							}
							_ => {
								val.push(c);
							}
						}
					}
					sc.eat();
					res.push(Token::Str(val));
				},
				' ' | '\n' | '\r' | '\t' => { sc.eat(); },
				'#' => {
					sc.eat_until('\n');
				},
				_ => {
					return Err(format!("unexpected token '{}'", c));
				}
			}
			prev = c;
		}

		// println!("{:#?}", res);

		res.push(Token::EOF);
		Ok(res)
	}
}