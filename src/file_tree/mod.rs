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

    // used only for test code
    #[allow(dead_code)]
    fn root_path(&self) -> &Path {
        self.root.path()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn expands_relative_path_correctly() {
        let cwd = PathBuf::from("/hoge");

        let tree1 = FileTree::resolve("/hoge/fuga/moge/1", &cwd);
        let tree2 = FileTree::resolve("./fuga/moge/1", &cwd);

        assert_eq!(tree1.root_path(), tree2.root_path());
    }
}
