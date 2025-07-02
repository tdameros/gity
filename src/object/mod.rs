pub mod blob;
pub mod commit;
pub mod tree;

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

    fn hash(&mut self) -> String {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(self.get_raw_data());
        format!("{:x}", hasher.finalize())
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
        use libz_sys::{compress2, compressBound, uLong, uLongf, Z_BEST_SPEED};
        let raw = self.get_raw_data();
        let src_len = raw.len() as uLong;
        let mut dst_len = unsafe { compressBound(src_len) };
        let mut dst = vec![0u8; dst_len as usize];
        let status = unsafe {
            compress2(
                dst.as_mut_ptr(),
                &mut dst_len as *mut uLongf,
                raw.as_ptr(),
                src_len,
                Z_BEST_SPEED,
            )
        };
        if status != 0 {
            panic!("Zlib compression failed with status: {}", status);
        }
        dst.truncate(dst_len as usize);
        dst
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
}

impl Clone for Box<dyn TreeObject> {
    fn clone(&self) -> Box<dyn TreeObject> {
        self.clone_box_tree()
    }
}
