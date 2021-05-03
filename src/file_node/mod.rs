use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;

pub enum FileNode {
    Directory(OsString, Vec<FileNode>),
    File(OsString),
}

impl FileNode {
    pub fn visit(os_string: &OsString) -> FileNode {
        let path = Path::new(os_string);

        if !path.is_dir() {
            return FileNode::File(OsString::from(path));
        }

        let directory = fs::read_dir(path).expect("failed to read directory");

        let children = directory
            .map(|child| {
                let child_path = child.expect("failed to read child file").path();
                let child_path = child_path.as_path();

                if child_path.is_dir() {
                    FileNode::visit(&OsString::from(child_path))
                } else {
                    FileNode::File(OsString::from(child_path))
                }
            })
            .collect();

        FileNode::Directory(OsString::from(path), children)
    }

    fn basename(&self) -> Option<&OsStr> {
        return Path::new(self.to_path()).file_name();
    }

    fn is_hidden(&self) -> bool {
        match self.basename() {
            Some(s) => match s.to_os_string().into_string() {
                Ok(s) => s.starts_with('.'),
                Err(_) => false,
            },
            None => false,
        }
    }

    fn to_path(&self) -> &OsString {
        match self {
            FileNode::Directory(path, _) => path,
            FileNode::File(path) => path,
        }
    }

    pub fn display(&self, depth: usize) -> Option<String> {
        if self.is_hidden() {
            return None;
        }

        match self {
            FileNode::Directory(_, children) => {
                let head: Vec<String> = vec![format!("/{}", self.basename()?.to_str()?)];

                let tail: Vec<String> = children
                    .iter()
                    .filter_map(|child| {
                        let subtree = child.display(depth + 1);
                        match subtree {
                            Some(tree) => Some(format!("{}└ {}", "\t".repeat(depth), tree)),
                            None => None,
                        }
                    })
                    .collect();

                Some([head, tail].concat().join("\n"))
            }
            FileNode::File(_) => Some(String::from(self.basename()?.to_str()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn file_shows_its_name() {
        let readme = FileNode::File(OsString::from("README.md"));

        assert_eq!(readme.display(0), Some(String::from("README.md")));
    }

    #[test]
    fn hidden_file_is_skipped() {
        let gitignore = FileNode::File(OsString::from(".gitignore"));

        assert_eq!(gitignore.display(0), None);
    }

    #[test]
    fn empty_directory_only_shows_dirname() {
        let dir = FileNode::Directory(OsString::from("target"), vec![]);

        assert_eq!(dir.display(0), Some(String::from("/target")));
    }

    #[test]
    fn non_empty_directory_shows_subtree() {
        let dir = FileNode::Directory(
            OsString::from("/src"),
            vec![
                FileNode::File(OsString::from(".gitignore")),
                FileNode::File(OsString::from("README.md")),
                FileNode::File(OsString::from("fuga.rs")),
                FileNode::Directory(
                    OsString::from("/child"),
                    vec![FileNode::File(OsString::from("hoge.png"))],
                ),
            ],
        );

        assert_eq!(dir.display(0), Some(expected_subtree()));
    }

    #[rustfmt::skip]
    fn expected_subtree() -> String {
        String::from(
"/src
└ README.md
└ fuga.rs
└ /child
\t└ hoge.png")
    }
}
