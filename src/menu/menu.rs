use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Node;

pub struct Menu {
    language_code: u32,
    schooling_id: String,
    user_planning_id: String,
    groups_planning_id: String,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl Menu {
    pub fn new<S: Into<String>, U: Into<String>, G: Into<String>>(
        language_code: u32,
        schooling_id: S,
        user_planning_id: U,
        groups_planning_id: G,
    ) -> Self {
        let schooling_id = schooling_id.into();
        let user_planning_id = user_planning_id.into();
        let groups_planning_id = groups_planning_id.into();

        let schooling_node = Rc::new(RefCell::new(Node::new(
            schooling_id.clone(),
            "Schooling",
            None,
        )));
        let groups_planning_node = Rc::new(RefCell::new(Node::new(
            groups_planning_id.clone(),
            "Groups",
            None,
        )));

        let mut nodes = HashMap::new();
        nodes.insert(schooling_id.clone(), Rc::clone(&schooling_node));
        nodes.insert(groups_planning_id.clone(), Rc::clone(&groups_planning_node));
        Self {
            language_code,
            schooling_id,
            user_planning_id,
            groups_planning_id,
            nodes,
        }
    }

    pub fn language_code(&self) -> u32 {
        self.language_code
    }

    pub fn schooling_id(&self) -> &str {
        &self.schooling_id
    }

    pub fn user_planning_id(&self) -> &str {
        &self.user_planning_id
    }

    pub fn groups_planning_id(&self) -> &str {
        &self.groups_planning_id
    }

    pub fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        let menu_id = node.borrow().id.clone();
        self.nodes.insert(menu_id, node);
    }

    pub fn get_menu_node<T: Into<String>>(&self, menu_id: T) -> Option<Rc<RefCell<Node>>> {
        self.nodes.get(&menu_id.into()).cloned()
    }

    pub fn is_node_loaded<T: Into<String>>(&self, menu_id: T) -> bool {
        let menu_id = menu_id.into();
        match self.nodes.get(&menu_id) {
            Some(node) => node.borrow().is_loaded(),
            None => false,
        }
    }
}
