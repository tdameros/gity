use super::{Object, ObjectType, TreeObject};
use crate::context::object::get_object;
use hex;
use std::any::Any;

#[derive(Clone)]
pub struct Tree {
    name: String,
    hash: String,
    objects: Vec<Box<dyn TreeObject>>,
}

impl Tree {
    #[allow(dead_code)]
    pub fn new(name: String, objects: Vec<Box<dyn TreeObject>>) -> Self {
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

    pub fn get_objects(&self) -> &Vec<Box<dyn TreeObject>> {
        &self.objects
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

    fn update_hash(&mut self) {
        self.hash = self.hash();
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
        self.update_hash();
    }
}

impl TreeObject for Tree {
    fn clone_box_tree(&self) -> Box<dyn TreeObject> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Vec<u8>> for Tree {
    type Error = String;
    fn try_from(content: Vec<u8>) -> Result<Self, Self::Error> {
        let mut index = 0;
        println!("content: {:?}", content);
        let mut objects: Vec<Box<dyn TreeObject>> = Vec::new();
        while index < content.len() {
            if let Some(space_index) = content[index..].iter().position(|&b| b == b' ') {
                let mode = String::from_utf8_lossy(&content[index..index + space_index]);
                println!("mode: {}", mode);
                index = index + space_index + 1;
                println!("index: {}", index);
                if let Some(zero_index) = content[index..].iter().position(|&b| b == b'\0') {
                    let rel_zero_index = index + zero_index;
                    let name = String::from_utf8_lossy(&content[index..rel_zero_index]);
                    println!("name: {}", name);
                    index = rel_zero_index + 1;
                    let hash = &content[index..index + 20];
                    let encoded_hash = hex::encode(hash);
                    println!("hash: {}", encoded_hash);
                    let object = get_object(encoded_hash);
                    if let Some(mut object) = object {
                        if let Some(tree_object) = object.as_tree_object_mut() {
                            tree_object.set_name(name.to_string());
                            objects.push(tree_object.clone_box_tree());
                        }
                    }
                    index += 21;
                }
            } else {
                println!("Missing space");
                break;
            }
        }
        Ok(Tree::new("".to_string(), objects))
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
