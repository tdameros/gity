use crate::config::OBJECTS_PATH;
use crate::object::EObject;

pub fn get_object(hash: String) -> Option<EObject> {
    let path = OBJECTS_PATH.join(&hash[..2]).join(&hash[2..]);
    println!("Path is {}", path.clone().display());
    let bytes = std::fs::read(path).unwrap();
    let obj = EObject::try_from(bytes);
    if let Ok(object) = obj {
        return Some(object);
    }
    None
}
