use super::file_node::FileNode;
use std::ffi::OsString;
use std::fmt::Display;
use std::path::Path;

pub struct FileTree {
    root: FileNode,
}

impl FileTree {
    pub fn resolve(path: impl AsRef<Path>, cwd: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            cwd.as_ref().join(path)
        };

        Self::new(OsString::from(path))
    }

    pub fn new(path: OsString) -> Self {
        Self {
            root: FileNode::visit(&path),
        }
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.root.display(vec![]) {
            Some(lines) => write!(f, "{}", lines),
            None => write!(f, ""),
        }
    }
}
