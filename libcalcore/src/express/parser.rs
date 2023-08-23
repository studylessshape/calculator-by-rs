pub mod lookahead;

use std::fmt::Display;

use super::{
    error::{CalculateError, ParserError},
    lexer::Token,
};

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    None,
    Expr,
    UnionOp(OpSymbol),
    UnionExpr,
    PhExpr,
    AddExpr(OpSymbol),
    MulExpr(OpSymbol),
    ExponExpr,
    Number(f64),
    Token(Token),
    EOF,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum OpSymbol {
    Unknown,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Caret,
}

impl From<Token> for OpSymbol {
    fn from(value: Token) -> Self {
        match value {
            Token::Plus => OpSymbol::Add,
            Token::Minus => OpSymbol::Subtract,
            Token::Multiply => OpSymbol::Multiply,
            Token::Division => OpSymbol::Divide,
            Token::Exponential => OpSymbol::Caret,
            Token::Percent => OpSymbol::Mod,
            _ => OpSymbol::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.children.len() {
            0 => format!("Node {}{:?}{}", '{', self.node_type, '}'),
            _ => format!(
                "Node {}{:?}, {:?}{}",
                '{', self.node_type, self.children, '}'
            ),
        };
        write!(f, "{}", s)
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::None,
            children: Vec::new(),
        }
    }

    pub fn new_type(node_type: NodeType) -> Self {
        let mut node = Self::new();
        node.node_type = node_type;
        node
    }

    pub fn from_token(token: Token) -> Self {
        Self::new_type(NodeType::Token(token))
    }

    pub fn calculate(&self) -> Result<f64, CalculateError> {
        match &self.node_type {
            NodeType::Expr => self.children[0].calculate(),
            NodeType::UnionExpr => {
                if self.children.len() == 2 {
                    if let NodeType::UnionOp(ops) = &self.children[0].node_type {
                        match ops {
                            OpSymbol::Add => self.children[1].calculate(),
                            OpSymbol::Subtract => Ok(-self.children[1].calculate()?),
                            _ => Err(CalculateError),
                        }
                    } else {
                        Err(CalculateError)
                    }
                } else {
                    self.children[0].calculate()
                }
            }
            NodeType::PhExpr => self.children[0].calculate(),
            NodeType::AddExpr(add_ops) => {
                if self.children.len() == 2 {
                    match add_ops {
                        OpSymbol::Add => {
                            Ok(self.children[0].calculate()? + self.children[1].calculate()?)
                        }
                        OpSymbol::Subtract => {
                            Ok(self.children[0].calculate()? - self.children[1].calculate()?)
                        }
                        _ => Err(CalculateError),
                    }
                } else {
                    self.children[0].calculate()
                }
            }
            NodeType::MulExpr(mul_ops) => {
                if self.children.len() == 2 {
                    match mul_ops {
                        OpSymbol::Multiply => {
                            Ok(self.children[0].calculate()? * self.children[1].calculate()?)
                        }
                        // todo: check the second num is valid for calculating
                        OpSymbol::Divide => {
                            Ok(self.children[0].calculate()? / self.children[1].calculate()?)
                        }
                        OpSymbol::Mod => {
                            Ok(self.children[0].calculate()? % self.children[1].calculate()?)
                        }
                        _ => Err(CalculateError),
                    }
                } else {
                    self.children[0].calculate()
                }
            }
            NodeType::ExponExpr => {
                if self.children.len() == 2 {
                    Ok(self.children[0]
                        .calculate()?
                        .powf(self.children[1].calculate()?))
                } else {
                    self.children[0].calculate()
                }
            }
            NodeType::Number(num) => Ok(*num),
            _ => Err(CalculateError),
        }
    }
}

#[derive(Debug)]
pub struct AST {
    root_node: Node,
}

impl AST {
    /// The action ofNodeError parsing token is a process, which can be described by BNF:
    /// ```BNF
    /// Expr ::= AddExpr;
    /// AddExpr ::= MulExpr {("+"|"-") AddExpr};
    /// MulExpr ::= ExponExpr {("*"|"/"|"%") ExponExpr};
    /// ExponExpr ::= UnionExpr | UnionExpr "^" UnionExpr;
    /// UnionExpr ::= PhExpr | UnionOp UnionExpr;
    /// UnionOp ::= "+" | "-";
    /// PhExpr ::= "(" AddExpr ")" | NUMBER;
    /// ```
    pub fn parse(tokens: Vec<Token>) -> Result<AST, ParserError> {
        let mut nodes = tokens.into_iter().map(Node::from_token).collect();
        Ok(Self { root_node: parse_expr(&mut nodes)? })
    }

    pub fn eval(&self) -> Result<f64, CalculateError> {
        // keep the precision
        self.root_node.calculate().map(|mut num| {
            let percision = 10_i32.pow(8) as f64;
            num = (num * percision).round() / percision;
            num
        })
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node)
    }
}

// ------- Parser ---------

/// Expr ::= AddExpr;
fn parse_expr(nodes: &mut Vec<Node>) -> Result<Node, ParserError> {
    // this line is the part `AddExpr` of `Expr ::= AddExpr;`
    parse_add_expr(nodes)?;
    // back the first node
    Ok(nodes[0].clone())
}

/// AddExpr ::= MulExpr {("+"|"-") AddExpr};
fn parse_add_expr(nodes: &mut Vec<Node>) -> Result<(), ParserError> {
    parse_mul_expr(nodes)?;
    let mut add_node = Node::new_type(NodeType::AddExpr(OpSymbol::Unknown));
    add_node.children.push(nodes[0].clone());
    // check if has symbol that is "+", "-"
    if let Some(node) = nodes.get(1) {
        if let NodeType::Token(token) = &node.node_type {
            match token {
                Token::Plus | Token::Minus => {
                    if let Token::Plus = token {
                        add_node.node_type = NodeType::AddExpr(OpSymbol::Add);
                    } else if let Token::Minus = token {
                        add_node.node_type = NodeType::AddExpr(OpSymbol::Subtract);
                    }
                    // remove symbol
                    nodes.drain(..=1);
                    // parse next node
                    parse_add_expr(nodes)?;
                    add_node.children.push(nodes[0].clone());
                }
                _ => {}
            }
        }
    }
    nodes[0] = add_node;
    return Ok(());
}

/// MulExpr ::= ExponExpr {("*"|"/"|"%") ExponExpr};
fn parse_mul_expr(nodes: &mut Vec<Node>) -> Result<(), ParserError> {
    parse_expon_expr(nodes)?;
    let mut mul_node = Node::new_type(NodeType::MulExpr(OpSymbol::Unknown));
    mul_node.children.push(nodes[0].clone());
    nodes[0] = mul_node.clone();
    // check if has symbol that is "*", "/", "%"
    while let Some(node) = nodes.get(1) {
        if let NodeType::Token(token) = &node.node_type {
            match token {
                Token::Multiply | Token::Division | Token::Percent => {
                    if let Token::Multiply = token {
                        mul_node.node_type = NodeType::MulExpr(OpSymbol::Multiply);
                    } else if let Token::Division = token {
                        mul_node.node_type = NodeType::MulExpr(OpSymbol::Divide);
                    } else if let Token::Percent = token {
                        mul_node.node_type = NodeType::MulExpr(OpSymbol::Mod);
                    }
                    // remove symbol
                    nodes.drain(..=1);
                    // parse next node
                    parse_expon_expr(nodes)?;
                    mul_node.children.push(nodes[0].clone());
                    // put node in to nodes
                    nodes[0] = mul_node.clone();
                    // pack node
                    mul_node = Node::new_type(NodeType::MulExpr(OpSymbol::Unknown));
                    mul_node.children.push(nodes[0].clone());
                }
                _ => break,
            }
        }
    }
    Ok(())
}

/// ExponExpr ::= UnionExpr {"^" PhExpr};
fn parse_expon_expr(nodes: &mut Vec<Node>) -> Result<(), ParserError> {
    parse_union_expr(nodes)?;
    let mut expon_node = Node::new_type(NodeType::ExponExpr);
    expon_node.children.push(nodes[0].clone());

    if let Some(node) = nodes.get(1) {
        if let NodeType::Token(Token::Exponential) = &node.node_type {
            // remove symbol
            nodes.drain(..=1);
            // parse next node
            parse_union_expr(nodes)?;
            expon_node.children.push(nodes[0].clone());
        }
    }
    nodes[0] = expon_node;
    Ok(())
}

/// UnionExpr ::= PhExpr | UnionOp UnionExpr;
fn parse_union_expr(nodes: &mut Vec<Node>) -> Result<(), ParserError> {
    match &nodes[0].node_type {
        NodeType::Token(token) => {
            let mut union_node = Node {
                node_type: NodeType::UnionExpr,
                children: vec![],
            };

            match token {
                Token::OpenPh | Token::Number(_) => parse_ph_expr(nodes)?,
                Token::Plus | Token::Minus => {
                    if let Token::Plus = token {
                        union_node
                            .children
                            .push(Node::new_type(NodeType::UnionOp(OpSymbol::Add)))
                    } else if let Token::Minus = token {
                        union_node
                            .children
                            .push(Node::new_type(NodeType::UnionOp(OpSymbol::Subtract)))
                    }
                    // remove the symbol
                    nodes.remove(0);
                    parse_union_expr(nodes)?;
                }
                _ => return Err(ParserError::UnionExpr(1)),
            }

            // set child
            union_node.children.push(nodes[0].clone());
            // put current node on nodes[0]
            nodes[0] = union_node;
            Ok(())
        }
        _ => Err(ParserError::UnionExpr(2)),
    }
}

/// PhExpr ::= "(" AddExpr ")" | NUMBER;
fn parse_ph_expr(nodes: &mut Vec<Node>) -> Result<(), ParserError> {
    match nodes[0].node_type {
        NodeType::Token(Token::OpenPh) => {
            // remove the first "("
            nodes.remove(0);
            // the expression between "(" and ")"
            parse_add_expr(nodes)?;
            // check if the second node is the symbol ")"
            if let Some(node) = nodes.get(1) {
                if let NodeType::Token(Token::ClosePh) = node.node_type {
                    nodes.remove(1);

                    let add_node = nodes[0].clone();

                    // create `PhExpr` node and replace nodes[0]
                    nodes[0] = Node {
                        node_type: NodeType::PhExpr,
                        children: vec![add_node],
                    };
                }
            } else {
                return Err(ParserError::PhExpr);
            }
        }
        NodeType::Token(Token::Number(fnum)) => {
            nodes[0] = Node {
                node_type: NodeType::Number(fnum),
                children: vec![],
            }
        }
        _ => return Err(ParserError::PhExpr),
    }
    Ok(())
}