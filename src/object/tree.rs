use super::{Object, ObjectType};
use hex;

#[derive(Clone)]
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
            objects: objects.clone(),
        };
        obj.objects.sort_by(|a, b| a.get_name().cmp(b.get_name()));
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

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::object::blob::Blob;
    use crate::object::tree::Tree;
    use crate::object::Object;

    #[test]
    fn hello_world_tree() {
        let blob = Blob::new("file.txt".to_string(), "hello world".to_string());
        let tree = Tree::new("mydir".to_string(), vec![Box::new(blob)]);
        assert_eq!(tree.get_hash(), "6c6a54b9bfc715ac30dae119b85cdad3df15e5b2");
    }

    #[test]
    fn multiple_blobs() {
        let blob1 = Blob::new("file.txt".to_string(), "hello world".to_string());
        let blob2 = Blob::new("file2.txt".to_string(), "hello world2".to_string());
        let tree = Tree::new("mydir".to_string(), vec![Box::new(blob1), Box::new(blob2)]);
        assert_eq!(tree.get_hash(), "f1c42276c6120e25f284e73392108dc75d670fe7");
    }

    #[test]
    fn multiple_blobs_reverse() {
        let blob1 = Blob::new("file.txt".to_string(), "hello world".to_string());
        let blob2 = Blob::new("file2.txt".to_string(), "hello world2".to_string());
        let tree = Tree::new("mydir".to_string(), vec![Box::new(blob2), Box::new(blob1)]);
        assert_eq!(tree.get_hash(), "f1c42276c6120e25f284e73392108dc75d670fe7");
    }

    #[test]
    fn subdirectories() {
        let blob1 = Blob::new("number.txt".to_string(), "123".to_string());
        let blob2 = Blob::new("hello.py".to_string(), "hello python".to_string());
        let subdir = Tree::new("subdir".to_string(), vec![Box::new(blob2)]);
        let tree = Tree::new("mydir".to_string(), vec![Box::new(subdir), Box::new(blob1)]);
        assert_eq!(tree.get_hash(), "f2221879a80b2554253e773fe73f22b91ba53caa");
    }
}
