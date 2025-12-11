use super::{Object, ObjectType, TreeObject};
use std::any::Any;

#[derive(Clone)]
pub struct Blob {
    name: String,
    content: String,
    hash: String,
}

impl Blob {
    #[allow(dead_code)]
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

    fn update_hash(&mut self) {
        self.hash = self.hash();
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
        self.update_hash();
    }
}

impl TreeObject for Blob {
    fn clone_box_tree(&self) -> Box<dyn TreeObject> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Vec<u8>> for Blob {
    type Error = String;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let content = String::from_utf8(value).map_err(|e| e.to_string())?;
        Ok(Blob::new("".to_string(), content))
    }
}

#[cfg(test)]
mod tests {
    use crate::object::blob::Blob;
    use crate::object::Object;

    #[test]
    fn empty_blob() {
        let blob = Blob::new("file.txt".to_string(), "".to_string());
        assert_eq!(blob.get_hash(), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
    }

    #[test]
    fn hello_world_blob() {
        let blob = Blob::new("file.txt".to_string(), "hello world".to_string());
        assert_eq!(blob.get_hash(), "95d09f2b10159347eece71399a7e2e907ea3df4f");
    }

    #[test]
    fn readme_blob() {
        let blob = Blob::new("README.md".to_string(), "#42-ssl_md5\nThis is a project from 42 School that aims to reimplement the md5 and sha256 hashing function. The goal is to understand the internal workings of cryptographic hash functions and implement them from scratch in C.".to_string());
        assert_eq!(blob.get_hash(), "0eacb9f1b5e88fc03fe5130bc1fe636f66c2efd8");
    }
}
