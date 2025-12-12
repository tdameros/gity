pub mod blob;
pub mod commit;
pub mod tree;

use crate::object::blob::Blob;
use crate::object::commit::Commit;
use crate::object::tree::Tree;
use crate::utils::hash::hash_sha1_hexa;
use crate::utils::zlib;
use std::any::Any;
use std::path::{Path, PathBuf};

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
        }
    }

    #[allow(unused)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "blob" => Some(ObjectType::Blob),
            "tree" => Some(ObjectType::Tree),
            "commit" => Some(ObjectType::Commit),
            _ => None,
        }
    }
}

pub trait Object {
    fn get_type(&self) -> ObjectType;
    fn get_content(&self) -> Vec<u8>;
    fn get_hash(&self) -> &String;
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);
    fn update_hash(&mut self);

    fn hash(&mut self) -> String {
        hash_sha1_hexa(self.get_raw_data())
    }

    fn get_raw_data(&self) -> Vec<u8> {
        [
            self.get_type().as_str().as_bytes(),
            b" ",
            self.get_content().len().to_string().as_bytes(),
            b"\0",
            self.get_content().as_ref(),
        ]
        .concat()
    }

    fn get_compress_content(&self) -> Vec<u8> {
        zlib::compress(self.get_raw_data())
    }

    fn save(&mut self, objects_directory: &Path) -> Result<PathBuf, std::io::Error> {
        let hash = self.get_hash();
        let dir_path = objects_directory.join(&hash[0..2]);
        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path)?;
        }
        let file_path = dir_path.join(&hash[2..]);
        std::fs::write(&file_path, self.get_compress_content())?;
        Ok(file_path)
    }
}

pub trait TreeObject: Object {
    fn clone_box_tree(&self) -> Box<dyn TreeObject>;
    fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn TreeObject> {
    fn clone(&self) -> Box<dyn TreeObject> {
        self.clone_box_tree()
    }
}

pub enum EObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl EObject {
    pub fn as_tree_object_mut(&mut self) -> Option<&mut dyn TreeObject> {
        match self {
            EObject::Blob(blob) => Some(blob),
            EObject::Tree(tree) => Some(tree),
            _ => None,
        }
    }
}

impl TryFrom<Vec<u8>> for EObject {
    type Error = String;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let decompressed = zlib::decompress(data);

        let null_index = decompressed
            .iter()
            .position(|&b| b == 0)
            .ok_or("Missing null byte separator")?;

        let header = &decompressed[..null_index];
        let content = &decompressed[null_index + 1..];

        let header_str = std::str::from_utf8(header).map_err(|_| "Invalid UTF-8 in header")?;
        let mut parts = header_str.split(' ');

        let type_str = parts.next().ok_or("Missing type")?;
        let size_str = parts.next().ok_or("Missing size")?;
        let size: usize = size_str.parse().map_err(|_| "Invalid size")?;

        if content.len() != size {
            return Err("Size mismatch".into());
        }
        println!("type: {}", type_str);
        println!("size: {}", size);
        match type_str {
            "blob" => Ok(EObject::Blob(Blob::try_from(content.to_vec())?)),
            "tree" => Ok(EObject::Tree(Tree::try_from(content.to_vec())?)),
            "commit" => Ok(EObject::Commit(Commit::try_from(content.to_vec())?)),
            _ => Err(format!("Unknown object type: {}", type_str)),
        }
    }
}
