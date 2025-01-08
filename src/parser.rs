use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Str(String),
    Num(f64),
    Bool(bool),
    Obj(Object),
    Arr(Array),
}

#[derive(Debug, Clone)]
pub struct Root {
    pub child: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub children: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Array {
    pub children: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub lhs: String,
    pub rhs: Value,
}

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    curr_tok: Option<Token>,
    curr_pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            curr_tok: Some(tokens[0].clone()),
            curr_pos: 0,
            tokens,
        }
    }

    fn advance(&mut self) {
        self.curr_pos += 1;
        self.curr_tok = if self.curr_pos < self.tokens.len() {
            Some(self.tokens[self.curr_pos].clone())
        } else {
            None
        }
    }

    fn parse_array(&mut self) -> Array {
        let mut arr = Array { children: vec![] };
        self.advance();
        'outer: while let Some(tok) = self.curr_tok.clone() {
            match tok {
                Token::RightBracket => break 'outer,
                Token::Null => arr.children.push(Value::Null),
                Token::Bool(val) => arr.children.push(Value::Bool(val)),
                Token::Number(val) => arr.children.push(Value::Num(val)),
                Token::Str(val) => arr.children.push(Value::Str(val)),
                Token::LeftBracket => {
                    let inner_array = self.parse_array();
                    arr.children.push(Value::Arr(inner_array));
                }
                Token::LeftBrace => {
                    let object = self.parse_object();
                    arr.children.push(Value::Obj(object));
                }
                _ => {}
            }

            self.advance();
        }

        arr
    }

    fn parse_property(&mut self, key: String) -> Property {
        let mut prop = Property {
            lhs: key,
            rhs: Value::Null,
        };

        self.advance();
        'outer: while let Some(tok) = self.curr_tok.clone() {
            match tok {
                Token::Comma => break 'outer,
                Token::RightBrace => break 'outer,
                Token::Null => prop.rhs = Value::Null,
                Token::Bool(val) => prop.rhs = Value::Bool(val),
                Token::Number(val) => prop.rhs = Value::Num(val),
                Token::Str(val) => prop.rhs = Value::Str(val),
                Token::LeftBracket => {
                    let inner_array = self.parse_array();
                    prop.rhs = Value::Arr(inner_array);
                }
                Token::LeftBrace => {
                    let object = self.parse_object();
                    prop.rhs = Value::Obj(object);
                }
                _ => {}
            }

            self.advance();
        }

        prop
    }

    fn parse_object(&mut self) -> Object {
        let mut obj = Object { children: vec![] };
        self.advance();

        'outer: while let Some(tok) = self.curr_tok.clone() {
            match tok {
                Token::RightBrace => break 'outer,
                Token::Eof => break 'outer,
                Token::Quote => {
                    self.advance();
                    let Some(key_tok) = self.curr_tok.clone() else {
                        break 'outer;
                    };

                    if let Token::Str(key) = key_tok {
                        let prop = self.parse_property(key);
                        obj.children.push(prop);
                        continue;
                    }
                }
                _ => {}
            }

            self.advance();
        }

        obj
    }

    pub(crate) fn parse(&mut self) -> Root {
        let mut root: Root = Root { child: None };

        while let Some(tok) = self.curr_tok.clone() {
            match tok {
                Token::Null => root.child = Some(Value::Null),
                Token::Bool(val) => root.child = Some(Value::Bool(val)),
                Token::Number(val) => root.child = Some(Value::Num(val)),
                Token::Str(val) => root.child = Some(Value::Str(val)),
                Token::LeftBracket => {
                    let array = self.parse_array();
                    root.child = Some(Value::Arr(array));
                }
                Token::LeftBrace => {
                    let object = self.parse_object();
                    root.child = Some(Value::Obj(object));
                }
                _ => {}
            }

            self.advance();
        }

        root
    }
}
