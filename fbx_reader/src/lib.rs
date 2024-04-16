extern crate fbx;

use fbx::format::{Node, Object, Type};

#[derive(Debug)]
pub struct FBXReader {
    object: Object,
}

impl FBXReader {
    pub fn from(object: Object) -> Self {
        FBXReader { object }
    }

    pub fn get(&self, name: &str) -> Option<FBXNodeReader> {
        for x in self.object.children().iter() {
            if x.name() == name {
                return Some(FBXNodeReader::from(x.clone()));
            }
        }

        return None;
    }
}

#[derive(Debug)]
pub struct FBXNodeReader {
    node: Node,
}

impl FBXNodeReader {
    pub fn from(node: Node) -> Self {
        FBXNodeReader { node }
    }

    pub fn get_nodes(&self, name: &str) -> Vec<FBXNodeReader> {
        self.node.children().iter().map(|w| FBXNodeReader::from(w.clone())).collect()
    }

    pub fn get_node(&self, name: &str) -> Option<FBXNodeReader> {
        for x in self.node.children().iter() {
            if x.name() == name {
                return Some(FBXNodeReader::from(x.clone()));
            }
        }

        None
    }

    pub fn get_values(&self) -> Vec<Type> {
        self.node.attributes()
    }

    pub fn get_value(&self) -> Option<Type> {
        let values = self.get_values();
        if values.iter().count() != 1 {
            return None;
        }

        Some(values.get(0).unwrap().clone())
    }
}