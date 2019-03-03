pub struct Scanner {
	input: Vec<char>,
	pos: usize
}

impl Scanner {
	pub fn new(input: Vec<char>) -> Scanner {
		Scanner {
			pos: 0,
			input: input
		}
	}

	pub fn from_str(input: &str) -> Scanner {
		Scanner::new(input.chars().collect())
	}

	pub fn current(&self) -> char {
		self.input[self.pos]
	}

	pub fn next(&self) -> char {
		self.input[self.pos + 1]
	}

	pub fn prev(&self) -> char {
		if self.pos == 0 {
			return '\0';
		}
		self.input[self.pos - 1]
	}

	pub fn eat(&mut self) -> Option<char> {
		if !self.has_next() {
			None
		} else {
			self.pos += 1;
			Some(self.input[self.pos-1])
		}
	}

	pub fn has_next(&self) -> bool {
		self.pos < self.input.len()
	}

	pub fn eat_until(&mut self, value: char) -> String {
		let mut ret = String::new();
		loop {
			if !self.has_next() || self.current() == value {
				break;
			}

			let c: char = self.eat().unwrap_or('\0');
			if c != value {
				ret.push(c);
			}
		}
		ret
	}

	pub fn eat_until_cond(&mut self, cond: fn(char) -> bool) -> String {
		let mut ret = String::new();
		loop {
			if !self.has_next() || cond(self.current()) {
				break;
			}

			let c: char = self.eat().unwrap_or('\0');
			ret.push(c);
		}
		ret
	}

	pub fn eat_until_any(&mut self, value: Vec<char>) -> String {
		let mut ret = String::new();
		loop {
			if !self.has_next() || value.contains(&self.current()) {
				break;
			}

			let c: char = self.eat().unwrap_or('\0');
			if !value.contains(&c) {
				ret.push(c);
			}
		}
		ret
	}

}
