use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Logical representation of a file hierarchy as a tree-like structure.
pub type Node = BTreeMap<String, Entry>;

/// Logical representation of a file or directory as JSON.
///
/// # Fields
/// `src`: `PathBuf` -> The file path for the current file object.
/// `art`: `PathBuf` -> The file path for the image to be displayed as cover art.
/// `parent`: `String` -> A key to be used in reverse traversal of the node tree.
/// `child`: `Node` -> The associated files underlying the current file object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub src: PathBuf,
    pub art: PathBuf,
    pub parent: String,
    pub child: Node,
}

impl Entry {
    pub fn default() -> Entry {
        Entry {
            src: PathBuf::from(""),
            art: PathBuf::from(""),
            parent: String::from(""),
            child: Node::new(),
        }
    }
}
