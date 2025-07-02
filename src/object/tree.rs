use super::{Object, ObjectType};
use hex;

pub struct Tree {
    name: String,
    hash: String,
    objects: Vec<Box<dyn Object>>,
}

impl Tree {
    pub fn new(name: String, objects: Vec<Box<dyn Object>>) -> Self {
        let mut obj = Self {
            name,
            hash: String::new(),
            objects,
        };
        obj.hash = obj.hash();
        obj
    }

    fn get_raw_content(&self) -> Vec<u8> {
        let mut content = Vec::new();
        for object in &self.objects {
            let hash_bytes = hex::decode(object.get_hash()).unwrap();
            let mode = match object.get_type() {
                ObjectType::Tree => "40000",
                ObjectType::Blob => "100644",
                _ => panic!("Unsupported type"),
            };
            content.extend_from_slice(mode.as_bytes());
            content.push(b' ');
            content.extend_from_slice(object.get_name().as_bytes());
            content.push(0);
            content.extend_from_slice(&hash_bytes);
        }
        content
    }
}

impl Object for Tree {
    fn get_type(&self) -> ObjectType {
        ObjectType::Tree
    }

    fn get_content(&self) -> Vec<u8> {
        self.get_raw_content()
    }

    fn get_hash(&self) -> &String {
        &self.hash
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
