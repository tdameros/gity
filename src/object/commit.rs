use crate::config::user::User;
use crate::object::tree::Tree;
use crate::object::{Object, ObjectType};
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Signature {
    pub user: User,
    pub date_time: DateTime<Local>,
}

#[derive(Clone)]
pub struct Commit {
    author: Signature,
    committer: Signature,
    hash: String,
    content: String,
    tree: Tree,
    parent: Option<Box<Commit>>,
}

impl Commit {
    #[allow(dead_code)]
    pub fn new(content: String, tree: &Tree, user: Signature, parent: Option<Commit>) -> Commit {
        let mut obj = Self {
            author: user.clone(),
            committer: user,
            hash: String::new(),
            content,
            tree: tree.clone(),
            parent: parent.map(Box::new),
        };
        obj.hash = obj.hash();
        obj
    }

    fn get_raw_content(&self) -> Vec<u8> {
        let tree_row = format!("tree {}\n", self.tree.get_hash());
        let author_row = format!(
            "author {} <{}> {} {}\n",
            self.author.user.username,
            self.author.user.email,
            self.author.date_time.timestamp(),
            self.get_formatted_utc_offset(&self.author.date_time)
        );
        let mut parent_row = String::new();
        if let Some(parent) = &self.parent {
            parent_row = format!("parent {}\n", parent.get_hash());
        }
        let committer_row = format!(
            "committer {} <{}> {} {}\n",
            self.committer.user.username,
            self.committer.user.email,
            self.author.date_time.timestamp(),
            self.get_formatted_utc_offset(&self.committer.date_time)
        );
        [
            tree_row.as_bytes(),
            parent_row.as_bytes(),
            author_row.as_bytes(),
            committer_row.as_bytes(),
            format!("\n{}\n", self.content).as_bytes(),
        ]
        .concat()
    }

    pub fn get_formatted_utc_offset(&self, date: &DateTime<Local>) -> String {
        let offset_seconds = date.offset().utc_minus_local();
        let total_seconds = -offset_seconds;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds.abs() % 3600) / 60;
        let sign = if total_seconds >= 0 { '+' } else { '-' };
        format!("{}{:02}{:02}", sign, hours.abs(), minutes)
    }
}

impl Object for Commit {
    fn get_type(&self) -> ObjectType {
        ObjectType::Commit
    }

    fn get_content(&self) -> Vec<u8> {
        self.get_raw_content()
    }

    fn get_hash(&self) -> &String {
        &self.hash
    }

    fn get_name(&self) -> &String {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::blob::Blob;
    use chrono::{FixedOffset, TimeZone};

    fn get_local_datetime_from_unix_timestamp(
        unix_timestamp: i64,
        offset_in_hours: i32,
    ) -> DateTime<Local> {
        let offset = FixedOffset::east_opt(offset_in_hours * 3600).unwrap();
        offset.timestamp_opt(unix_timestamp, 0).unwrap().into()
    }

    #[test]
    fn simple_commit() {
        let blob = Blob::new("file.txt".to_string(), "".to_string());
        let tree = Tree::new("".to_string(), vec![Box::new(blob)]);
        let user = Signature {
            user: User {
                username: "tdameros".to_string(),
                email: "tdameros@something.com".to_string(),
            },
            date_time: get_local_datetime_from_unix_timestamp(1751551520, 2),
        };
        let commit = Commit::new("first commit".to_string(), &tree, user, None);
        assert_eq!(
            commit.get_hash(),
            "14a16a8d50b0bea47b6277b3500dde4f5e9be43a"
        );
    }

    #[test]
    fn multiple_commits() {
        let blob_file_txt = Blob::new("file.txt".to_string(), "".to_string());
        let tree = Tree::new("".to_string(), vec![Box::new(blob_file_txt.clone())]);
        let signature = Signature {
            user: User {
                username: "tdameros".to_string(),
                email: "tdameros@something.com".to_string(),
            },
            date_time: get_local_datetime_from_unix_timestamp(1751551520, 2),
        };
        let commit = Commit::new("first commit".to_string(), &tree, signature.clone(), None);
        assert_eq!(
            commit.get_hash(),
            "14a16a8d50b0bea47b6277b3500dde4f5e9be43a"
        );
        let blob_hello_py = Blob::new("hello.py".to_string(), "".to_string());
        let second_tree = Tree::new(
            "".to_string(),
            vec![Box::new(blob_file_txt), Box::new(blob_hello_py)],
        );
        let second_signature = Signature {
            user: User {
                username: "tdameros".to_string(),
                email: "tdameros@something.com".to_string(),
            },
            date_time: get_local_datetime_from_unix_timestamp(1751551598, 2),
        };
        let second_commit = Commit::new(
            "second commit".to_string(),
            &second_tree,
            second_signature,
            Some(commit),
        );
        assert_eq!(
            second_commit.get_hash(),
            "87d97756f779ec23e8f46aa0b73e0f0423d2d5ea"
        );
    }
}
