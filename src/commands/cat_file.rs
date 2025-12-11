use clap::Args;

use crate::context::object::get_object;
use crate::object::blob::Blob;
use crate::object::tree::Tree;
use crate::object::{EObject, Object, TreeObject};

#[derive(Args)]
pub struct CatFileArgs {
    pub object_hash: String,
}

pub fn run(args: &CatFileArgs) {
    let object = get_object(args.object_hash.clone());
    match object {
        Some(EObject::Blob(blob)) => {
            // println!("{}", blob.get_hash())
        }
        Some(EObject::Tree(tree)) => {
            println!("{}", tree.get_hash());
            for object in tree.get_objects() {
                if let Some(blob) = object.as_any().downcast_ref::<Blob>() {
                    println!("Blob {} {}", blob.get_hash(), blob.get_name());
                } else if let Some(subtree) = object.as_any().downcast_ref::<Tree>() {
                    println!("Tree {} {}", subtree.get_hash(), subtree.get_name());
                }
            }
        }
        Some(EObject::Commit(commit)) => {
            println!("COMMMIT");
            println!("{:?}", commit.author);
            println!("{:?}", commit.tree.get_hash());
            println!("{}", commit.content);
            println!("{}", commit.get_hash());
            println!("{}", String::from_utf8(commit.get_raw_content()).unwrap());
        }
        None => {}
    }
}
