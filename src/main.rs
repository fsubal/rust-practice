use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::fs;
use std::path::Path;

enum FileNode {
    Directory(OsString, Vec<FileNode>),
    File(OsString),
}

impl FileNode {
    fn visit(os_string: &OsString) -> FileNode {
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

    fn display(&self, depth: usize) -> Option<String> {
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

struct FileTree {
    root: FileNode,
}

impl FileTree {
    fn new(path: String) -> Self {
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

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("usage: cargo run [filename]");
        std::process::exit(1);
    }

    let _command = args.next();
    let target = args.next().unwrap();

    println!("{}", FileTree::new(target));
}
