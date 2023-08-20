use std::{cell::RefCell, ops::RangeBounds, rc::Rc};

use super::{error::LexerError, token::Token};

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

#[derive(Debug, PartialEq, Clone)]
pub enum OpSymbol {
    UnKnow,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Caret,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::None,
            children: Vec::new(),
        }
    }

    pub fn new_type(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: Vec::new(),
        }
    }

    pub fn set_tokens(tokens: Vec<Token>) -> Vec<Rc<RefCell<Self>>> {
        let mut nodes = Vec::new();
        for token in tokens {
            nodes.push(Rc::new(RefCell::new(Self::new_type(NodeType::Token(
                token,
            )))));
        }
        nodes
    }
}

#[derive(Debug)]
pub struct Lexer {
    root_node: Node,
}

impl Lexer {
    pub fn new(node: Node) -> Self {
        Self { root_node: node }
    }
}

/// The action ofNodeError parsing token is a process, while can be described by BNF:
/// ```BNF
/// Expr ::= AddExpr;
/// AddExpr ::= MulExpr {("+"|"-") AddExpr};
/// MulExpr ::= ExponExpr {("*"|"/"|"%") MulExpr};
/// ExponExpr ::= UnionExpr {"^" UnionExpr};
/// UnionExpr ::= PhExpr | UnionOp UnionExpr;
/// UnionOp ::= "+" | "-";
/// PhExpr ::= "(" AddExpr ")" | NUMBER;
/// ```
pub fn parse_token(tokens: Vec<Token>) -> Result<Lexer, LexerError> {
    let nodes = Rc::new(RefCell::new(Node::set_tokens(tokens)));
    match expr(nodes) {
        Ok(node) => Ok(Lexer::new(node)),
        Err(err) => Err(err),
    }
}

/// Expr ::= AddExpr;
fn expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<Node, LexerError> {
    // this line is the part `AddExpr` of `Expr ::= AddExpr;`
    add_expr(nodes.clone())?;
    // back the first node
    Ok(nodes.borrow()[0].borrow().clone())
}

/// AddExpr ::= MulExpr {("+"|"-") AddExpr};
fn add_expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<(), LexerError> {
    mul_expr(nodes.clone())?;
    let mut add_node = Node::new_type(NodeType::AddExpr(OpSymbol::UnKnow));
    add_node.children.push(nodes.borrow()[0].clone());
    // check if has symbol that is "+", "-"
    if let Some(node) = nodes.borrow().get(1) {
        if let NodeType::Token(token) = &node.borrow().node_type {
            match token {
                Token::Plus | Token::Minus => {
                    if let Token::Plus = token {
                        add_node.node_type = NodeType::AddExpr(OpSymbol::Add);
                    } else if let Token::Minus = token {
                        add_node.node_type = NodeType::AddExpr(OpSymbol::Subtract);
                    }
                    // remove symbol
                    remove_node(..=1, nodes.clone());
                    // parse next node
                    add_expr(nodes.clone())?;
                    add_node.children.push(nodes.borrow()[0].clone());
                }
                _ => {}
            }
        }
    }
    nodes.borrow_mut()[0] = Rc::new(RefCell::new(add_node));
    return Ok(());
}

/// MulExpr ::= ExponExpr {("*"|"/"|"%") MulExpr};
fn mul_expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<(), LexerError> {
    expon_expr(nodes.clone())?;
    let mut mul_node = Node::new_type(NodeType::MulExpr(OpSymbol::UnKnow));
    mul_node.children.push(nodes.borrow()[0].clone());
    // check if has symbol that is "*", "/", "%"
    if let Some(node) = nodes.borrow().get(1) {
        if let NodeType::Token(token) = &node.borrow().node_type {
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
                    remove_node(..=1, nodes.clone());
                    // parse next node
                    mul_expr(nodes.clone())?;
                    mul_node.children.push(nodes.borrow()[0].clone());
                }
                _ => {}
            }
        }
    }
    nodes.borrow_mut()[0] = Rc::new(RefCell::new(mul_node));
    Ok(())
}

/// ExponExpr ::= UnionExpr {"^" PhExpr};
fn expon_expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<(), LexerError> {
    union_expr(nodes.clone())?;
    let mut expon_node = Node::new_type(NodeType::ExponExpr);
    expon_node.children.push(nodes.borrow()[0].clone());

    if let Some(node) = nodes.borrow().get(1) {
        if let NodeType::Token(Token::Exponential) = &node.borrow().node_type {
            // remove symbol
            remove_node(..=1, nodes.clone());
            // parse next node
            union_expr(nodes.clone())?;
            expon_node.children.push(nodes.borrow()[0].clone());
        }
    }
    nodes.borrow_mut()[0] = Rc::new(RefCell::new(expon_node));
    Ok(())
}

/// UnionExpr ::= PhExpr | UnionOp UnionExpr;
fn union_expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<(), LexerError> {
    match &nodes.borrow()[0].borrow().node_type {
        NodeType::Token(token) => {
            let mut union_node = Node {
                node_type: NodeType::UnionExpr,
                children: vec![],
            };

            match token {
                Token::OpenPh => ph_expr(nodes.clone())?,
                Token::Plus | Token::Minus => {
                    if let Token::Plus = token {
                        union_node
                            .children
                            .push(Rc::new(RefCell::new(Node::new_type(NodeType::UnionOp(
                                OpSymbol::Add,
                            )))))
                    } else if let Token::Minus = token {
                        union_node
                            .children
                            .push(Rc::new(RefCell::new(Node::new_type(NodeType::UnionOp(
                                OpSymbol::Subtract,
                            )))))
                    }
                    // remove the symbol
                    let mut node_mut_borrow = nodes.borrow_mut();
                    node_mut_borrow.remove(0);
                    union_expr(nodes.clone())?;
                }
                _ => return Err(LexerError),
            }

            // set child
            union_node.children.push(nodes.borrow()[0].clone());
            // put current node on nodes[0]
            let mut node_mut_borrow = nodes.borrow_mut();
            node_mut_borrow[0] = Rc::new(RefCell::new(union_node));
            Ok(())
        }
        _ => Err(LexerError),
    }
}

/// PhExpr ::= "(" AddExpr ")" | NUMBER;
fn ph_expr(nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>) -> Result<(), LexerError> {
    match nodes.borrow()[0].borrow().node_type {
        NodeType::Token(Token::OpenPh) => {
            // remove the first "("
            remove_node(..=0, nodes.clone());
            // the expression between "(" and ")"
            add_expr(nodes.clone())?;
            // check if the second node is the symbol ")"
            if let Some(node) = nodes.borrow().get(1) {
                if let NodeType::Token(Token::ClosePh) = node.borrow().node_type {
                    remove_node(1..=1, nodes.clone());

                    let mut node_mut_borrow = nodes.borrow_mut();
                    let add_node = node_mut_borrow[0].clone();

                    // create `PhExpr` node and replace nodes[0]
                    node_mut_borrow[0] = Rc::new(RefCell::new(Node {
                        node_type: NodeType::PhExpr,
                        children: vec![add_node],
                    }));
                }
            } else {
                return Err(LexerError);
            }
        }
        NodeType::Token(Token::Number(fnum)) => {
            nodes.borrow_mut()[0] = Rc::new(RefCell::new(Node {
                node_type: NodeType::Number(fnum),
                children: vec![],
            }))
        }
        _ => return Err(LexerError),
    }
    Ok(())
}

fn remove_node<R>(idx: R, nodes: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>)
where
    R: RangeBounds<usize>,
{
    let mut nodes_mut_borrow = nodes.borrow_mut();
    nodes_mut_borrow.drain(idx);
}
