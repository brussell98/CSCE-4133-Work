use std::io;
use std::io::BufRead;

/// Evaluate the expressions given through stdin
pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();

	for line in lines { // While there are more test cases
		let mut exp = Expression::new(line);

		println!("{}", exp.evaluate());
	}
}

#[derive(PartialEq, Clone, Debug)]
enum TokenType { Int, Op, Open, Close }

/// Represents a number, operator, or parenthesis as part of an expression
#[derive(Clone, Debug)]
struct Token {
	t_type: TokenType,
	value: Option<i32>,
	operator: Option<String>
}

impl Token {
	fn new(source: &str) -> Token {
		let t = if source == "(" {
			TokenType::Open
		} else if source == ")" {
			TokenType::Close
		} else if source == "+" || source == "-" || source == "*" {
			TokenType::Op
		} else {
			TokenType::Int
		};

		// Create a Token with fields set depending on the type of token
		Token {
			value: if t == TokenType::Int {
				Some(source.parse::<i32>().unwrap())
			} else {
				None
			},
			operator: if t == TokenType::Op {
				Some(String::from(source))
			} else {
				None
			},
			t_type: t // Assigned last because t can't be used after this
		}
	}
}

/// A simple math expression with irregular operator precedence. Can not be updated after calling evaluate()
struct Expression {
	tokenized: Vec<Token>, // The tokens in the order they appear
	postfix: Vec<Token>, // A "stack" of the tokens in postfix order
	value: Option<i32> // The resulting value of the expression
}

impl Expression {
	fn new(source: String) -> Expression {
		let mut exp = Expression {
			tokenized: Vec::new(),
			postfix: Vec::new(),
			value: None
		};

		let mut i = 0;
		while i < source.len() {
			// If it is a number, find out how long it is and create a token with the full number
			if source.chars().nth(i).unwrap().is_numeric() {
				let mut p = i + 1;
				while p < source.len() && source.chars().nth(p).unwrap().is_numeric() {
					p += 1;
				}

				exp.tokenized.push(Token::new(&source[i..p]));

				i = p;
			} else {
				exp.tokenized.push(Token::new(&source[i..=i]));

				i += 1;
			}
		}

		exp
	}

	/// Build a postfix order vector of the tokenized expression
	fn build_postfix(&mut self) {
		let mut op_stack = Vec::new();

		for token in self.tokenized.clone() {
			if token.t_type == TokenType::Int {
				self.postfix.push(token);
			} else if token.t_type == TokenType::Open {
				op_stack.push(token);
			} else { // Close parenthesis or operator
				// Push all operators to postfix until an opening parenthesis is found, indicating the end of this level
				while !op_stack.is_empty() && op_stack.last().unwrap().t_type != TokenType::Open {
					self.postfix.push(op_stack.pop().unwrap());
				}

				if token.t_type == TokenType::Close {
					op_stack.pop(); // Remove the opening parenthesis
				} else { // If an operator push it to the stack
					op_stack.push(token);
				}
			}
		}

		while !op_stack.is_empty() { // Push the remaining operators
			self.postfix.push(op_stack.pop().unwrap());
		}
	}

	/// Evaluate the expression using the postfix order, and store the result
	fn evaluate(&mut self) -> i32 {
		if self.value.is_some() {
			return self.value.unwrap();
		}

		self.build_postfix();

		let mut operands = Vec::new(); // A stack of numbers

		for token in self.postfix.clone() {
			if token.t_type == TokenType::Int {
				operands.push(token.value.unwrap());
			} else {
				// If an operator, get the top two numbers and perform the operation, then push back to the stack
				let val2 = operands.pop().unwrap();
				let val1 = operands.pop().unwrap();

				if token.operator.as_ref().unwrap().as_str() == "+" {
					operands.push(val1 + val2);
				} else if token.operator.as_ref().unwrap().as_str() == "-" {
					operands.push(val1 - val2);
				} else if token.operator.as_ref().unwrap().as_str() == "*" {
					operands.push(val1 * val2);
				}
			}
		}

		self.value = operands.pop(); // The one remaining item in the stack (all ops done) is the final value
		self.value.unwrap()
	}
}
