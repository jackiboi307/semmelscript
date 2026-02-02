use crate::prelude::*;
use crate::syntax::*;

pub struct Parser {
    chars: Box<[char]>,
    pub i: usize,
}

impl Parser {
    pub fn new(buffer: String) -> Self {
        Self {
            chars: buffer.chars().collect::<Vec<_>>().into(),
            i: 0,
        }
    }

    // helper functions

    fn step(&mut self) {
        self.i += 1;
    }

    fn stepn(&mut self, n: usize) {
        self.i += n;
    }

    fn peek(&mut self) -> Result<&char> {
        self.chars.get(self.i).ok_or(EOF.into())
    }

    fn next(&mut self) -> Result<&char> {
        self.step();
        self.chars.get(self.i - 1).ok_or(EOF.into())
    }

    fn expect(&mut self, ch: char) -> Result<()> {
        if let Ok(next_ch) = self.next() {
            if *next_ch != ch {
                Err(ExpectedToken(ch.to_string()).into())
            } else {
                Ok(())
            }
        } else {
            Err(EOF.into())
        }
    }

    fn remaining(&mut self) -> &[char] {
        &self.chars[self.i..]
    }

    fn peek_from_chars(&mut self, chars: &'static str) -> String {
        self.remaining().iter()
            .take_while(|c| chars.contains(**c))
            .map(|c| *c)
            .collect()
    }

    fn next_from_chars(&mut self, chars: &'static str) -> String {
        let result = self.peek_from_chars(chars);
        self.stepn(result.chars().count());
        result
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            if let Ok(ch) = self.peek() {
                if ch.is_whitespace() {
                    self.step();
                } else {
                    break Ok(())
                }
            } else {
                break Err(EOF.into())
            }
        }
    }

    // parse nodes

    fn read_integer(&mut self) -> Result<Node> {
        let int = Node::Integer(self.next_from_chars(DIGITS).parse().unwrap());
        Ok(int)
    }

    fn read_string(&mut self) -> Result<Node> {
        let terminator = *self.next()?;
        let result: String = self.remaining().iter()
            .take_while(|c| **c != terminator)
            .map(|c| *c)
            .collect();
        self.stepn(result.chars().count());

        if *self.next()? != terminator {
            Err(ExpectedToken(terminator.to_string()).into())
        } else {
            Ok(Node::String(result))
        }
    }

    fn read_identifier(&mut self) -> Result<String> {
        Ok(self.next_from_chars(IDENTIFIER_CHARS))
    }

    fn read_operator(&mut self) -> Result<Operator> {
        use Operator::*;

        let op = &*self.next_from_chars(OPERATOR_CHARS);

        Ok(match op {
            OP_ADD => Add,
            OP_SUB => Sub,
            OP_MUL => Mul,
            OP_DIV => Div,
            OP_POW => Pow,
            OP_FIELD_ACCESS => FieldAccess,
            OP_PAREN => Paren,
            _ => { return Err(InvalidOperator(op.to_string()).into()); }
        })
    }

    fn read_value(&mut self) -> Result<Node> {
        let ch = self.peek()?;

        let value = if DIGITS.contains(*ch) {
            self.read_integer()?
        } else if STRING_TERMINATORS.contains(*ch) {
            self.read_string()?
        } else if LETTERS.contains(*ch) {
            Node::Identifier(self.read_identifier()?)
        } else {
            return Err(UnexpectedCharacter(*ch).into());
        };

        if self.skip_whitespace().is_err() {
            return Ok(value);
        }

        if VALUE_EXTENSION_OPERATOR_CHARS.contains(*self.peek()?) {
            todo!()
        }
        
        Ok(value)
    }

    fn read_expression(&mut self) -> Result<Node> {
        let mut values: Vec<Node> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new();

        loop {
            values.push(self.read_value()?);

            if self.skip_whitespace().is_err() {
                break
            }

            let ch = self.peek()?;

            if OPERATOR_CHARS.contains(*ch) {
                let op = self.read_operator()?;

                self.skip_whitespace()?;

                operators.push(op);

            } else {
                break
            }
        }

        // TODO this is probably pretty inefficient:

        while 0 < operators.len() {
            'levels: for level in OPERATOR_ORDER {
                for target_op in level.iter() {
                    for (i, op) in operators.clone().iter().enumerate() {
                        if op == target_op {
                            let [a, b] = values.splice(i..=i+1, []).collect::<Vec<_>>().try_into().unwrap();
                            let _ = operators.remove(i);
                            values.insert(i, Node::BinaryOp(Box::new(BinaryOp {
                                op: *op,
                                a,
                                b,
                            })));
                            break 'levels
                        }
                    }
                }
            }
        }

        assert!(values.len() == 1);

        Ok(values[0].clone())
    }

    fn read_let(&mut self) -> Result<Node> {
        let ident = self.read_identifier()?;
        self.skip_whitespace()?;

        self.expect('=')?;
        self.skip_whitespace()?;

        let value = self.read_expression()?;

        Ok(Node::Statement(Statement::SetVariable(ident, Box::new(value))))
    }

    fn read_statement(&mut self) -> Result<Node> {
        let potential_keyword = &*self.peek_from_chars(LOWERCASE_LETTERS);

        if !potential_keyword.is_empty() {
            // is there something after the keyword?
            if let Some(ch) = self.chars.get(self.i + potential_keyword.chars().count()) {
                // is that something whitespace?
                if ch.is_whitespace() {
                    let _ = self.next_from_chars(LOWERCASE_LETTERS);
                    let _ = self.skip_whitespace();

                    match potential_keyword {
                        KW_LET => return self.read_let(),
                        _ => {}
                    }
                }
            }
        }

        self.read_expression()
    }

    // parse the whole buffer

    pub fn parse(&mut self) -> Result<()> {
        let mut stack = Vec::new();

        stack.push(self.read_statement()?);

        if self.peek().is_ok() {
            panic!("Unread chars starting at index {}", self.i);
        }

        println!("parsed result:");
        for i in stack {
            println!("{i}");
        }

        Ok(())
    }
}
