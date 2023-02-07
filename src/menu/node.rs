use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new<I: Into<String>, N: Into<String>>(id: I, name: N, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            children: Vec::new(),
            parent: match parent {
                Some(node) => Some(Rc::clone(&node)),
                None => None,
            },
        }
    }

    /// Add a child to the current node.
    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    /// Get the children of the current node.
    pub fn get_children(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.children
    }

    /// Check if the node has been loaded
    pub fn is_loaded(&self) -> bool {
        !(&self.id.starts_with("submenu_") ^ !self.children.is_empty())
    }

    /// Check if the node is a leaf
    pub fn is_leaf(&self) -> bool {
        !self.id.starts_with("submenu_") && self.children.is_empty()
    }
}
