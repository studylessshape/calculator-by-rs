use std::rc::Rc;

use super::error::NodeError;

pub trait ExpressNode {
    fn ln(&self) -> Option<Rc<dyn ExpressNode>>;
    fn rn(&self) -> Option<Rc<dyn ExpressNode>>;

    fn set_ln(&mut self, new_left_node: Option<Rc<dyn ExpressNode>>) -> Result<(), NodeError>;

    fn result(&self) -> f64;
}

pub struct ZeroNode {
    left_node: Option<Rc<dyn ExpressNode>>,
    right_node: Option<Rc<dyn ExpressNode>>,
}

impl ZeroNode {
    pub fn new () -> Self {
        Self { left_node: None, right_node: None }
    }
}

impl ExpressNode for ZeroNode {
    fn ln(&self) -> Option<Rc<dyn ExpressNode>> {
        self.left_node.clone()
    }

    fn rn(&self) -> Option<Rc<dyn ExpressNode>> {
        self.right_node.clone()
    }

    fn result(&self) -> f64 {
        0.0
    }

    fn set_ln(&mut self, _new_left_node: Option<Rc<dyn ExpressNode>>) -> Result<(), NodeError> {
        todo!()
    }
}