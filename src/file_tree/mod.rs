use super::file_node::FileNode;
use std::ffi::OsString;
use std::fmt::Display;

pub struct FileTree {
    root: FileNode,
}

impl FileTree {
    pub fn new(path: String) -> Self {
        FileTree {
            root: FileNode::visit(&OsString::from(path)),
        }
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.root.display(0).unwrap();
        write!(f, "{}", lines)
    }
}
