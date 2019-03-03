use crate::lexer::{ Token, BuiltinKeyword };
use crate::scanner::Scanner;

#[derive(Debug)]
pub enum ComparisonOp {
	Lt = 0,
	Gt,
	Le,
	Ge,
	Ne,
	Equ
}

#[derive(Debug)]
pub enum Node {
	Number(f64),
	Boolean(bool),
	Identifier(String),
	Str(String),

	Power(Box<Node>, Box<Node>),
	Factor(char, Box<Node>),
	Term(Box<Node>, char, Box<Node>),
	Arith(Box<Node>, char, Box<Node>),
	Shift(Box<Node>, u8, Box<Node>),
	And(Box<Node>, Box<Node>),
	Xor(Box<Node>, Box<Node>),
	Expr(Box<Node>, Box<Node>),

	Comparison(Box<Node>, ComparisonOp, Box<Node>),
	Not(Box<Node>),
	LogicAnd(Box<Node>, Box<Node>),
	LogicOr(Box<Node>, Box<Node>),
	Test(Box<Node>, Box<Node>, Box<Node>), // Ternary

	Var(String, Option<Box<Node>>),
	Let(Vec<Box<Node>>),

	Assign(Box<Node>, String, Box<Node>),

	Return(Box<Node>),

	Break,
	Continue,
	Semi
}

pub struct Interpreter {
	input: Vec<Token>,
	pos: usize
}

impl Interpreter {
	pub fn new(input: Vec<Token>) -> Interpreter {
		Interpreter {
			pos: 0,
			input
		}
	}

	fn step_back(&mut self) {
		if self.pos > 0 {
			self.pos -= 1;
		}
	}

	fn advance(&mut self) {
		if self.pos < self.input.len() {
			self.pos += 1;
		}
	}

	fn last(&mut self) -> Token {
		if self.pos == 0 {
			self.input[self.pos].clone()
		} else {
			self.input[self.pos - 1].clone()
		}
	}

	fn current(&mut self) -> Token {
		self.input[self.pos].clone()
	}

	fn atom(&mut self) -> Result<Box<Node>, String> {
		match self.current() {
			Token::Identifier(id) => { self.advance(); Ok(Box::new(Node::Identifier(id.to_string()))) },
			Token::Number(n) => { self.advance(); Ok(Box::new(Node::Number(n))) },
			Token::Str(s) => { self.advance(); Ok(Box::new(Node::Str(s.to_string()))) },
			Token::Symbol(s) => {
				match s.as_str() {
					"(" => {
						self.advance();
						let test = self.test();
						match self.current() {
							Token::Symbol(t) => {
								match t.as_str() {
									")" => { return test; },
									_ => { return Err(format!("unexpected token: {:?}", self.current())); }
								}
							},
							_ => { return Err(format!("unexpected token: {:?}", self.current())); }
						}
					},
					_ => { return Err(format!("unexpected token: {:?}", self.current())); }
				}
			},
			_ => { return Err(format!("unexpected token: {:?}", self.current())); }
		}
	}

	fn power(&mut self) -> Result<Box<Node>, String> {
		let a = self.atom();
		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"**" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};
		let b = self.factor();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Power(a.unwrap(), b.unwrap())))
	}

	fn factor(&mut self) -> Result<Box<Node>, String> {
		match self.current() {
			Token::Symbol(s) => {
				match s.as_str() {
					"-" | "~" => {
						self.advance();
						Ok(Box::new(Node::Factor(s.chars().next().unwrap(), self.factor().unwrap())))
					},
					_ => { return self.power(); }
				}
			},
			_ => { return self.power(); }
		}
	}

	fn term(&mut self) -> Result<Box<Node>, String> {
		let a = self.factor();
		if a.is_err() { return a; }

		let op = match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"*" | "/" | "%" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};

		self.advance();
		let b = self.factor();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Term(a.unwrap(), op.chars().next().unwrap(), b.unwrap())))
	}

	fn arith(&mut self) -> Result<Box<Node>, String> {
		let a = self.term();
		if a.is_err() { return a; }

		let op = match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"+" | "-" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};

		self.advance();
		let b = self.term();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Arith(a.unwrap(), op.chars().next().unwrap(), b.unwrap())))
	}

	fn shift(&mut self) -> Result<Box<Node>, String> {
		let a = self.arith();
		if a.is_err() { return a; }

		let op = match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"<<" | ">>" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};

		self.advance();
		let b = self.arith();
		if b.is_err() { return b; }

		let dir = match op.as_str() {
			"<<" => 0,
			">>" => 1,
			_ => 0
		};
		Ok(Box::new(Node::Shift(a.unwrap(), dir, b.unwrap())))
	}

	fn binand(&mut self) -> Result<Box<Node>, String> {
		let a = self.shift();
		if a.is_err() { return a; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"&" => {},
				_ => { return a; }
			},
			_ => { return a; }
		}

		self.advance();
		let b = self.shift();
		if b.is_err() { return b; }

		Ok(Box::new(Node::And(a.unwrap(), b.unwrap())))
	}

	fn bixor(&mut self) -> Result<Box<Node>, String> {
		let a = self.binand();
		if a.is_err() { return a; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"^" => {}
				_ => { return a; }
			},
			_ => { return a; }
		}

		self.advance();
		let b = self.binand();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Xor(a.unwrap(), b.unwrap())))
	}

	fn expr(&mut self) -> Result<Box<Node>, String> {
		let a = self.bixor();
		if a.is_err() { return a; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"|" => {}
				_ => { return a; }
			},
			_ => { return a; }
		}

		self.advance();
		let b = self.bixor();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Expr(a.unwrap(), b.unwrap())))
	}

	fn comparison(&mut self) -> Result<Box<Node>, String> {
		let a = self.expr();
		if a.is_err() { return a; }

		let op = match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"<" | ">" | "<=" | ">=" | "!=" | "==" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};

		self.advance();
		let b = self.expr();
		if b.is_err() { return b; }

		let cop = match op.as_str() {
			"<" => ComparisonOp::Lt,
			">" => ComparisonOp::Gt,
			"<=" => ComparisonOp::Le,
			">=" => ComparisonOp::Ge,
			"==" => ComparisonOp::Equ,
			"!=" => ComparisonOp::Ne,
			_ => ComparisonOp::Equ
		};

		Ok(Box::new(Node::Comparison(a.unwrap(), cop, b.unwrap())))
	}

	fn not_test(&mut self) -> Result<Box<Node>, String> {
		match self.current() {
			Token::Symbol(s) => {
				match s.as_str() {
					"!" => {
						self.advance();
						Ok(Box::new(Node::Not(self.comparison().unwrap())))
					},
					_ => { return self.comparison(); }
				}
			},
			_ => { return self.comparison(); }
		}
	}

	fn and_test(&mut self) -> Result<Box<Node>, String> {
		let a = self.not_test();
		if a.is_err() { return a; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"&&" => {},
				_ => { return a; }
			},
			_ => { return a; }
		}

		self.advance();
		let b = self.not_test();
		if b.is_err() { return b; }

		Ok(Box::new(Node::LogicAnd(a.unwrap(), b.unwrap())))
	}

	fn or_test(&mut self) -> Result<Box<Node>, String> {
		let a = self.and_test();
		if a.is_err() { return a; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"||" => {},
				_ => { return a; }
			},
			_ => { return a; }
		}

		self.advance();
		let b = self.and_test();
		if b.is_err() { return b; }

		Ok(Box::new(Node::LogicOr(a.unwrap(), b.unwrap())))
	}

	fn test(&mut self) -> Result<Box<Node>, String> {
		let cond = self.or_test();
		if cond.is_err() { return cond; }

		match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"?" => {
					self.advance();
					let left = self.test();
					if left.is_err() { return left; }
					self.advance();
					let right = self.test();
					if right.is_err() { return right; }

					Ok(Box::new(Node::Test(cond.unwrap(), left.unwrap(), right.unwrap())))
				},
				_ => { return cond; }
			},
			_ => { return cond; }
		}
	}

	fn assignment(&mut self) -> Result<Box<Node>, String> {
		let a = self.test();
		if a.is_err() { return a; }

		let op = match self.current() {
			Token::Symbol(cs) => match cs.as_str() {
				"+=" | "-=" | "*=" | "/=" | "%=" | "&=" |
				"|=" | "^=" | "<<=" | ">>=" | "**=" | "=" => cs.to_owned(),
				_ => { return a; }
			},
			_ => { return a; }
		};

		self.advance();
		let b = self.test();
		if b.is_err() { return b; }

		Ok(Box::new(Node::Assign(a.unwrap(), op, b.unwrap())))
	}

	fn arg_list(&mut self) -> Result<Vec<Box<Node>>, String> {
		let mut ret: Vec<Box<Node>> = Vec::new();
		let arg = self.test();
		if arg.is_err() { return Ok(ret); }

		ret.push(arg.unwrap());
		loop {
			match self.current() {
				Token::Symbol(s) => {
					match s.as_str() {
						"," => {
							ret.push(self.test().unwrap())
						},
						_ => { return Ok(ret); }
					}
				},
				_ => { return Ok(ret); }
			}
		}
	}

	fn single_var(&mut self) -> Result<Box<Node>, String> {
		let name = self.current();
		self.advance();

		match name {
			Token::Identifier(id) => {
				let mut value = None;
				match self.current() {
					Token::Symbol(s) => {
						match s.as_str() {
							"=" => { self.advance(); value = Some(self.test().unwrap()); },
							_ => {}
						}
					},
					_ => {}
				}
				Ok(Box::new(Node::Var(id.to_string(), value)))
			},
			_ => { return Err(format!("expected an identifier, got a {:?}", self.last())); }
		}
	}

	fn var_list(&mut self) -> Result<Vec<Box<Node>>, String> {
		let mut res = Vec::new();

		let var = self.single_var();
		if var.is_err() { return Err("expected variable declaration".to_owned()); }

		res.push(var.unwrap());

		loop {
			match self.current() {
				Token::Symbol(s) => {
					match s.as_str() {
						"," => { self.advance(); res.push(self.single_var().unwrap()); },
						_ => { break; }
					}
				},
				_ => { break; }
			}
		}
		Ok(res)
	}

	fn let_stmt(&mut self) -> Result<Box<Node>, String> {
		let lt = self.current();
		self.advance();

		match lt {
			Token::Keyword(kw) => {
				match kw {
					BuiltinKeyword::Let => {
						let vars = match self.var_list() {
							Ok(lst) => lst,
							Err(e) => { return Err(e); }
						};
						Ok(Box::new(Node::Let(vars)))
					},
					_ => { return Err("expected keyword 'let'".to_owned()); }
				}
			},
			_ => { return Err("expected keyword 'let'".to_owned()); }
		}
	}

	fn stmt(&mut self) -> Result<Box<Node>, String> {
		let tok = self.current();
		match tok {
			Token::Semi => { self.advance(); Ok(Box::new(Node::Semi)) },
			Token::Keyword(kw) => {
				match kw {
					BuiltinKeyword::Break => Ok(Box::new(Node::Break)),
					BuiltinKeyword::Continue => Ok(Box::new(Node::Continue)),
					BuiltinKeyword::Return => Ok(Box::new(Node::Return(self.test().unwrap()))),
					BuiltinKeyword::Let => self.let_stmt(),
					_ => Err("not implemented".to_owned())
				}
			},
			_ => self.assignment()
		}
	}

	fn stmt_list(&mut self) -> Result<Vec<Box<Node>>, String> {
		let mut res = Vec::new();

		let st = self.stmt();
		if st.is_err() { return Err("expected statement".to_owned()); }

		res.push(st.unwrap());

		while self.current() != Token::EOF {
			res.push(self.stmt().unwrap());
		}
		Ok(res)
	}

	pub fn parse(&mut self) {
		println!("{:#?}", self.stmt_list().unwrap());
	}
}
