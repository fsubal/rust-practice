use std::ffi::OsString;
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

    fn basename(&self) -> Option<OsString> {
        return Path::new(self.to_path())
            .file_name()
            .map(|s| s.to_os_string());
    }

    fn is_hidden(&self) -> bool {
        match self.basename() {
            Some(s) => match s.into_string() {
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

    fn on_visit_node(
        item: &FileNode,
        index: usize,
        has_nexts: &[bool],
        all: &[FileNode],
    ) -> Option<String> {
        let has_next = all.iter().len() != index + 1;

        let arm = has_nexts.iter().fold(String::from(""), |arm, has_next| {
            if *has_next {
                format!("{}│   ", arm)
            } else {
                format!("{}    ", arm)
            }
        });
        let hand = if has_next { "├" } else { "└" };

        let subtree = item.display([has_nexts, &[has_next]].concat());
        match subtree {
            Some(tree) => Some(format!("{}{} {}", arm, hand, tree)),
            None => None,
        }
    }

    pub fn display(&self, has_nexts: Vec<bool>) -> Option<String> {
        if self.is_hidden() {
            return None;
        }

        match self {
            FileNode::Directory(_, children) => {
                let head: Vec<String> = vec![format!("/{}", self.basename()?.to_str()?)];

                let tail: Vec<String> = children
                    .iter()
                    .enumerate()
                    .filter_map(|(index, child)| {
                        FileNode::on_visit_node(child, index, &has_nexts, children)
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

        assert_eq!(readme.display(vec![]), Some(String::from("README.md")));
    }

    #[test]
    fn hidden_file_is_skipped() {
        let gitignore = FileNode::File(OsString::from(".gitignore"));

        assert_eq!(gitignore.display(vec![]), None);
    }

    #[test]
    fn empty_directory_only_shows_dirname() {
        let dir = FileNode::Directory(OsString::from("target"), vec![]);

        assert_eq!(dir.display(vec![]), Some(String::from("/target")));
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

        assert_eq!(dir.display(vec![]), Some(expected_subtree()));
    }

    #[rustfmt::skip]
    fn expected_subtree() -> String {
        String::from(
"/src
├ README.md
├ fuga.rs
└ /child
    └ hoge.png")
    }
}
