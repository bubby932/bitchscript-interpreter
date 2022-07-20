use std::{fs, env};
use std::collections::HashMap;
struct Lexer {
    contents : String,
    index : usize
}
#[derive(Debug)]
#[derive(Clone)]
struct Token {
    token_type : TokenType,
    literal : String,
}
impl Token {
    fn create(token : TokenType, literal : String) -> Self {
        Self { token_type: token, literal }
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum TokenType {
    Let,
    Identifier,
    Assign,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    PrintCall,
    ScopeDrop,
    ScopeJump,
    ExpressionStart,
    ExpressionClose,
    Conditional,
    ConditionalElse,
    Goto
}
impl Lexer {
    fn create(contents : String) -> Self {
        Lexer {
            contents,
            index : 0
        }
    }

    fn tokenize(&mut self) -> Vec::<Token> {
        let chars = self.contents.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut tokens = Vec::<Token>::new();

        while self.index < len {
            let c = chars[self.index];
            match c {
                '\'' | '\"' => {
                    let mut literal = String::new();
                    self.index += 1;
                    while self.index < len {
                        if chars[self.index] != c {
                            literal.push(chars[self.index]);
                            self.index += 1;
                        } else {
                            self.index += 1;
                            break;
                        }
                    }

                    tokens.push(Token::create(TokenType::StringLiteral, literal));
                },
                '=' => {
                    self.index += 1;
                    tokens.push(Token::create(TokenType::Assign, "=".to_owned()));
                },
                '{' => {
                    self.index += 1;
                    tokens.push(Token::create(TokenType::ScopeDrop, "{".to_owned()));
                },
                '}' => {
                    self.index += 1;
                    tokens.push(Token::create(TokenType::ScopeJump, "}".to_owned()));
                },
                '(' => {
                    self.index += 1;
                    tokens.push(Token::create(TokenType::ExpressionStart, "(".to_owned()));
                },
                ')' => {
                    self.index += 1;
                    tokens.push(Token::create(TokenType::ExpressionClose, ")".to_owned()));
                },
                _ if c.is_alphabetic() => {
                    let mut literal = String::new();
                    while self.index < len && chars[self.index].is_alphabetic()  {
                        literal.push(chars[self.index]);
                        self.index += 1;
                    }
                    
                    match literal.as_str() {
                        "let" => {
                            tokens.push(Token::create(TokenType::Let, literal));
                        },
                        "print" => {
                            tokens.push(Token::create(TokenType::PrintCall, literal));
                        },
                        "if" => {
                            tokens.push(Token::create(TokenType::Conditional, literal));
                        },
                        "else" => {
                            tokens.push(Token::create(TokenType::ConditionalElse, literal));
                        },
                        "true" => {
                            tokens.push(Token::create(TokenType::BooleanLiteral, literal));
                        },
                        "false" => {
                            tokens.push(Token::create(TokenType::BooleanLiteral, literal));
                        },
                        "goto" => {
                            tokens.push(Token::create(TokenType::Goto, literal));
                        },
                        _ => {
                            tokens.push(Token::create(TokenType::Identifier, literal));
                        }
                    }
                },
                _ if c.is_numeric() => {
                    let mut literal = String::new();
                    while self.index < len && chars[self.index].is_numeric() {
                        literal.push(chars[self.index]);
                        self.index += 1;
                    }
                    tokens.push(Token::create(TokenType::NumberLiteral, literal));
                },
                _ if c.is_whitespace() => {
                    self.index += 1;
                },
                _ => {
                    panic!("Unexpected character!");
                }
            }
        }
        tokens
    }
}
struct Interpreter {
    tokens : Vec<Token>,
    variables : HashMap<String, VariableValue>,
    index : usize
}
impl Interpreter {
    fn new(tokens : Vec<Token>) -> Self {
        Self {
            tokens,
            variables: HashMap::new(),
            index: 0
        }
    }

    fn run(&mut self) {
        let len = self.tokens.len();

        while self.index < len {
            match self.tokens[self.index].token_type {
                TokenType::Let => {
                    self.increment_index(1);
                    if self.tokens[self.index].token_type != TokenType::Identifier {
                        panic!("Invalid token after `let` keyword - must be a valid alphabetic identifier.");
                    }
                    let name = self.tokens[self.index].literal.to_owned();
                    self.increment_index(1);
                    if self.tokens[self.index].token_type != TokenType::Assign {
                        panic!("Invalid token after variable identifier - must be Assignment Operator.");
                    }
                    self.increment_index(1);
                    match self.tokens[self.index].token_type {
                        TokenType::StringLiteral => {
                            let value = self.tokens[self.index].literal.to_owned();
                            self.variables.insert(name, VariableValue::String(value));
                            self.increment_index(1);
                        },
                        TokenType::NumberLiteral => {
                            match (&self.tokens[self.index].literal).parse::<f64>() {
                                Ok(val) => {
                                    self.variables.insert(name, VariableValue::Number(val));
                                    self.increment_index(1);
                                },
                                Err(err) => {
                                    panic!("Failed to parse numeric literal!\n{}", err);
                                }
                            }
                        },
                        TokenType::BooleanLiteral => {
                            match (&self.tokens[self.index].literal).parse::<bool>() {
                                Ok(val) => {
                                    self.variables.insert(name, VariableValue::Boolean(val));
                                    self.increment_index(1);
                                },
                                Err(err) => {
                                    panic!("Failed to parse boolean literal!\n{}", err);
                                }
                            }
                        },
                        TokenType::Identifier => {
                            let val = match self.variables.get(&self.tokens[self.index].literal) {
                                Some(v) => v.to_owned(),
                                None => panic!("use of undefined variable")
                            };
                            
                            
                            match val {
                                VariableValue::String(str) => {
                                    self.variables.insert(name, VariableValue::String(str.to_owned()));
                                    self.increment_index(1);
                                }
                                VariableValue::Number(num) => {
                                    self.variables.insert(name, VariableValue::Number(num.to_owned()));
                                    self.increment_index(1);
                                },
                                VariableValue::Boolean(bool) => {
                                    self.variables.insert(name, VariableValue::Boolean(bool.to_owned()));
                                    self.increment_index(1);
                                }
                            }
                        },
                        _ => unimplemented!()
                    }
                },
                TokenType::PrintCall => {
                    self.increment_index(1);
                    match self.tokens[self.index].token_type {
                        TokenType::Identifier => 
                        {
                            let val = match self.variables.get(&self.tokens[self.index].literal) {
                                None => {
                                    panic!("Use of undefined variable in print statement.");
                                },
                                Some(val) => match val {
                                    VariableValue::String(str) => str.to_owned(),
                                    VariableValue::Boolean(bool) => bool.to_string(),
                                    VariableValue::Number(num) => num.to_string()
                                }
                            };
                            println!("Print call : {}", val);
                            self.increment_index(1);
                        },
                        TokenType::StringLiteral    |
                        TokenType::BooleanLiteral   |
                        TokenType::NumberLiteral => 
                        {
                            println!("Print call : {}", self.tokens[self.index].literal);
                            
                            self.increment_index(1);
                        },
                        _ => {
                            panic!("Invalid token after print call - must be String Literal or Variable Identifier.");
                        }
                    };
                },
                TokenType::Goto => {
                    self.increment_index(1);
                    match self.tokens[self.index].token_type {
                        TokenType::Identifier => {
                            let var = self.variables.get(&self.tokens[self.index].literal);
                            if var.is_none() {
                                panic!("Use of undefined variable in goto call!");
                            }
                            
                            match var.unwrap() {
                                VariableValue::Boolean(_) => panic!("Boolean variable in goto statement - must be numeric!"),
                                VariableValue::String(_) => panic!("String variable in goto statement - must be numeric!"),
                                VariableValue::Number(num) => {
                                    self.index = num.to_owned() as usize;
                                }
                            }
                        },
                        TokenType::NumberLiteral => {
                            self.index = self.tokens[self.index].literal.parse::<usize>().unwrap();
                            println!("GOTO - went to line {}", self.index);
                        },
                        _ => {
                            panic!("Invalid token after goto call - must be a Numeric Variable Identifier or Numeric Literal");
                        }
                    }
                }
                TokenType::Conditional => {
                    self.increment_index(1);
                    let next = self.tokens[self.index].clone();
                    match next.token_type {
                        TokenType::ExpressionStart => todo!(),
                        TokenType::Identifier => {
                            let var = match self.variables.get(&next.literal) {
                                Some(a) => a.to_owned(),
                                None => panic!("use of undefined variable")
                            };

                            let val = match var {
                                VariableValue::Boolean(a) => a,
                                _ => panic!("non boolean variable in if statement")
                            };

                            match val {
                                true => {
                                    self.increment_index(2);
                                },
                                false => {
                                    let next = match self.find_index_of_token(self.index, TokenType::ScopeJump) {
                                        Some(i) => i,
                                        None => panic!("Unclosed conditional statement!")
                                    };
                                    self.index = next + 1;
                                    if self.tokens[self.index].token_type == TokenType::ConditionalElse {
                                        self.index += 2;
                                    }
                                }
                            }
                        }
                        TokenType::BooleanLiteral => {
                            match next.literal.as_str() {
                                "true" => {
                                    self.increment_index(2);
                                },
                                "false" => {
                                    let next = match self.find_index_of_token(self.index, TokenType::ScopeJump) {
                                        Some(i) => i,
                                        None => panic!("Unclosed conditional statement!")
                                    };
                                    self.index = next + 1;
                                    if self.tokens[self.index].token_type == TokenType::ConditionalElse {
                                        self.index += 2;
                                    }
                                },
                                _ => panic!()
                            }
                        }
                        _ => unimplemented!()
                    }
                
                
                },
                TokenType::ConditionalElse => {
                    self.index += 1;
                    if self.tokens[self.index].token_type != TokenType::ScopeDrop {
                        panic!("Conditional Else statement with no scoping.");
                    }

                    let next = match self.find_index_of_token(self.index, TokenType::ScopeJump) {
                        Some(x) => x,
                        None => panic!("Unfinished else block.")
                    };
                    self.index = next + 1;
                },
                TokenType::ScopeJump => self.increment_index(1),
                _ => panic!("Invalid token {:?}!", self.tokens[self.index])
            }
        }
    }

    // Yes, this is unnecessary for a single line of code, but it makes things clearer to read.
    fn increment_index(&mut self, n : usize) {
        self.index += n;
    }

    fn find_index_of_token(&self, current : usize, token_type : TokenType) -> Option<usize> {
        let len = self.tokens.len();
        let mut i : usize = current;
        while i < len {
            let t = match self.tokens.get(i) {
                Some(s) => s.to_owned(),
                None => panic!()
            };

            if t.token_type == token_type {
                return Some(i);
            }
            i += 1;
        }

        None
    }
}
#[derive(Debug)]
#[derive(Clone)]
enum VariableValue {
    String(String),
    Number(f64),
    Boolean(bool)
}

fn main() {
    let mut argv = env::args();
    let filename = match argv.nth(1) {
        Some(filename) => filename,
        None => "index.bs".to_owned()
    };
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => panic!("Failed to read file!\nError:\n{}", err)
    };
    let mut lex : Lexer = Lexer::create(contents);

    let tokens = lex.tokenize();
    // println!("Tokens:\n{:?}", tokens);

    let mut interp : Interpreter = Interpreter::new(tokens);
    interp.run();
    // println!("{:?}", interp.variables);
}