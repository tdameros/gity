use super::{Object, ObjectType};

pub struct Blob {
    name: String,
    content: String,
    hash: String,
}

impl Blob {
    pub fn new(name: String, content: String) -> Self {
        let mut obj = Self {
            name,
            content,
            hash: String::new(),
        };
        obj.hash = obj.hash();
        obj
    }
}

impl Object for Blob {
    fn get_type(&self) -> ObjectType {
        ObjectType::Blob
    }

    fn get_content(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    fn get_hash(&self) -> &String {
        &self.hash
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
