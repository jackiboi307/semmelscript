use crate::prelude::*;
use crate::syntax::*;

pub struct Parser {
    chars: Box<[char]>,
    i: usize,
    row: usize,
    col: usize,
}

// TODO create some tokens module, with this enum and Operator

#[derive(Debug)]
pub enum Keyword {
    Let,
    If,
    Else,
}

impl Parser {
    pub fn new(buffer: String) -> Self {
        Self {
            chars: buffer.chars().collect::<Vec<_>>().into(),
            i: 0,
            row: 0,
            col: 0,
        }
    }

    pub fn row(&self) -> usize {
        self.row + 1
    }

    pub fn col(&self) -> usize {
        self.col
    }

    // helper functions

    fn step(&mut self) {
        self.i += 1;
        self.col += 1;
    }

    fn stepn(&mut self, n: usize) {
        self.i += n;
        self.col += n;
    }

    fn peek(&self) -> Result<&char> {
        self.chars.get(self.i).ok_or(EOF.into())
    }

    fn peekn(&self, n: usize) -> Result<String> {
        Ok(self.chars.get(self.i..self.i + n).ok_or(EOF)?
            .iter().collect::<String>())
    }

    fn next(&mut self) -> Result<&char> {
        self.step();
        self.chars.get(self.i - 1).ok_or(EOF.into())
    }

    fn expect(&mut self, string: &str) -> Result<()> {
        let len = string.len();
        if self.peekn(len)? == string {
            self.i += len;
            Ok(())
        } else {
            Err(ExpectedToken(string.to_string()).into())
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
            // TODO fix retarded cloning?
            if let Ok(ch) = self.peek().cloned() {
                if ch.is_whitespace() {
                    self.step();
                    if ch == '\n' {
                        self.col = 0;
                        self.row += 1;
                    }
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
            "." => FieldAccess,
            "(" => Paren,
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
        } else if *ch == '(' {
            self.step();
            let value = self.read_expression()?;
            self.expect(")")?;
            value
        // } else if *ch == '{' {
        //     self.read_block(true)?
        } else {
            return Err(UnexpectedCharacter(*ch).into());
        };

        if self.skip_whitespace().is_err() {
            return Ok(value);
        }

        if let Ok(ch) = self.peek() { match ch {
            '(' => {
                self.step();
                return Ok(Node::ParenArgs(Box::new(value), self.read_args()?));
            }
            _ => {}
        }}
        
        Ok(value)
    }

    fn read_args(&mut self) -> Result<Vec<Node>> {
        let _ = self.skip_whitespace();
        let mut args = Vec::new();

        loop {
            args.push(self.read_expression()?);

            match self.next()? {
                ')' => { return Ok(args) }
                ',' => {}
                _ => { return Err(ExpectedTokens(&[",", ")"]).into()) }
            }

            let _ = self.skip_whitespace();
        }
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

            if EXPR_TERMINATORS.contains(*ch) {
                break

            } if OPERATOR_CHARS.contains(*ch) {
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
                            let [a, b]: [Node; 2] = values.splice(i..=i+1, [])
                                .collect::<Vec<_>>().try_into().unwrap();
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

        self.expect("=")?;
        self.skip_whitespace()?;

        let value = self.read_expression()?;

        Ok(Node::Statement(Statement::SetVariable(ident, Box::new(value))))
    }

    fn read_if(&mut self) -> Result<Node> {
        let condition = self.read_expression()?;
        let block = self.read_block(true)?;
        let ext = if let Some(kw) = self.read_keyword() {
            match kw {
                Keyword::Else => Some(Box::new(self.read_block(true)?)),
                /*Keyword::If => {
                    
                }*/
                _ => None
            }
        } else {
            None
        };
        Ok(Node::Statement(Statement::If(Box::new(condition), Box::new(block), ext)))
    }

    fn read_keyword(&mut self) -> Option<Keyword> {
        let potential_keyword = &*self.peek_from_chars(LOWERCASE_LETTERS);

        if !potential_keyword.is_empty() {
            // is there something after the keyword?
            if let Some(ch) = self.chars.get(self.i + potential_keyword.chars().count()) {
                // is that something whitespace?
                if ch.is_whitespace() {
                    let _ = self.next_from_chars(LOWERCASE_LETTERS);
                    let _ = self.skip_whitespace();

                    return Some(match potential_keyword {
                        KW_LET => Keyword::Let,
                        KW_IF => Keyword::If,
                        KW_ELSE => Keyword::Else,
                    })
                }
            }

        }

        None
    }

    fn read_statement(&mut self) -> Result<Node> {
        if let Some(keyword) = self.read_keyword() {
            match keyword {
                Keyword::Let => return self.read_let(),
                Keyword::If => return self.read_if(),
                _ => { return Err(UnexpectedKeyword(keyword).into()) }
            }
        }

        self.read_expression()
    }

    fn read_block(&mut self, inner: bool) -> Result<Node> {
        if inner {
            self.expect("{")?;
        }

        let mut nodes = Vec::new();

        if self.skip_whitespace().is_ok() {
            loop {
                nodes.push(self.read_statement()?);

                if let Ok(ch) = self.next().cloned() {
                    match ch {
                        ';' => {}
                        '\n' => {} // TODO improve?
                        _ => {
                            return Err(UnexpectedCharacter(ch).into())
                        }
                    }

                } else {
                    break
                }

                if self.skip_whitespace().is_err() {
                    break

                } else if inner {
                    if let Ok(ch) = self.peek() {
                        if *ch == '}' {
                            self.step();
                            break
                        }
                    }
                }
            }
        }

        let _ = self.skip_whitespace();

        return Ok(Node::Block(Block::new(nodes)))
    }

    // parse the whole buffer

    pub fn parse(&mut self) -> Result<()> {
        let block = self.read_block(false)?;

        if self.peek().is_ok() {
            panic!("Unread chars starting at index {}", self.i);
        }

        match block {
            Node::Block(block) => {
                println!("{}", block.format(0));
            }
            _ => unreachable!()
        }

        Ok(())
    }
}
