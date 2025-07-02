use crate::object::{Object, ObjectType};

#[derive(Clone)]
pub struct Commit {
    name: String,
    hash: String,
}

impl Commit {}

impl Object for Commit {
    fn get_type(&self) -> ObjectType {
        ObjectType::Commit
    }

    fn get_content(&self) -> Vec<u8> {
        vec![]
    }

    fn get_hash(&self) -> &String {
        &self.hash
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
