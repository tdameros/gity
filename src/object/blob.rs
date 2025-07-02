use super::{Object, ObjectType};

#[derive(Clone)]
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

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
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
