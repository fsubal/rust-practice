mod file_node;
mod file_tree;

use file_tree::FileTree;
use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("usage: cargo run [filename]");
        std::process::exit(1);
    }

    let _command = args.next();
    let target = args.next().unwrap();
    let cwd = env::current_dir().unwrap();

    println!("{}", FileTree::from(target, cwd));
}
