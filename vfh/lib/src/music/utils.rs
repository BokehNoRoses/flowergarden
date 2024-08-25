use super::node::{Entry, Node};
use std::{
    ffi::{OsStr, OsString},
    fs::{read_dir, DirEntry},
    io::Result,
    path::{Path, PathBuf},
};

/// Helper function that returns a `PathBuf` for the `Entry.art` field.
///
/// Image naming convention: `${KEY}.${EXT}`, where
///     &#09;`KEY`: `OsString` -> The name of file or directory as it appears in the file browser or terminal.
///                        &#09;&#09;&#09;&#09;&#09;&emsp;If it is a file, the key is the basename with file extension stripped.
///     &#09;`EXT`: `&str` -> The chosen file extension to be displayed in the Tauri web app.
///                           &#09;&#09;&#09;&#09;&emsp;In order of precedence:
///                            &#09;&#09;&#09;&#09;&emsp;- `.webp`
///                            &#09;&#09;&#09;&#09;&emsp;- `.apng`
///                            &#09;&#09;&#09;&#09;&emsp;- `.gif`
///                            &#09;&#09;&#09;&#09;&emsp;- `.svg`
///                            &#09;&#09;&#09;&#09;&emsp;- `.png`
///                            &#09;&#09;&#09;&#09;&emsp;- `.jpeg`
///                            &#09;&#09;&#09;&#09;&emsp;- `.jpg`
///
/// The image file type search is based on guidance given in the [`Mozilla MDN web docs`](https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Image_types).
///
/// Current Nodes will be evaluated first to see if they have a corresponding image file first. If
/// the current Node does not have a corresponding image file, the parent Node's art will be used.
/// This allows for extensive customization for individual Artist, Album, and Song file objects as
/// well as ease of setting cover art for all descendants of a common ancestor in the Node tree.
///
/// # Examples
/// ```
/// let config: Config = Config::new().unwrap(); // get_base_directory() is the entry point for
///                                             // the file hiearchy being built
/// let mut tree: Node = Node::new(); // Node -> BTreeMap<OsString, Entry>
/// let _ = populate(&config.base, &mut tree); // populate() calls update_art()
/// ```
pub fn update_art(path: &Path) -> PathBuf {
    let mut pb: PathBuf = path.to_path_buf();
    let extensions: [&str; 8] = [
        ".webp", ".apng", ".gif", ".avif", ".svg", ".png", ".jpeg", ".jpg",
    ];

    // Check if child node has a corresponding art file first
    for ext in extensions {
        let mut p: PathBuf = pb.clone();
        let mut name: OsString = p.file_stem().unwrap_or(OsStr::new("")).to_os_string();

        if name.is_empty() {
            break;
        }

        match p.is_dir() {
            // If dir, we want to search inside path for art
            true => {
                name.push(ext);
                p.push(name);
            }
            // If file, we want to search alongside path for art
            false => {
                p.pop();
                name.push(ext);
                p.push(name);
            }
        }

        if p.exists() {
            return p;
        }
    }

    // Check if parent node has a corresponding art file to use instead
    // Parent node should always be a directory file object, so we do not need to strip path
    pb.pop();
    for ext in extensions {
        let mut p: PathBuf = pb.clone();
        let mut name: OsString = p.file_stem().unwrap_or(OsStr::new("")).to_os_string();

        if name.is_empty() {
            break;
        }

        name.push(ext);
        p.push(name);

        if p.exists() {
            return p;
        }
    }

    return PathBuf::new(); // default to an empty Path if nothing is found
}

/// Helper function that populates file paths and associated metadata in a Node tree.
///
/// # Examples
/// ```
/// let config: Config = Config::new().unwrap(); // get_base_directory() is the entry point for
///                                             // the file hiearchy being built
/// let mut tree: Node = Node::new(); // Node -> BTreeMap<OsString, Entry>
/// let _ = populate(&config.base, &mut tree); // populate() mutates Node tree
/// ```
pub fn populate(dir: &Path, node: &mut Node) -> Result<()> {
    if dir.is_dir() {
        for entry in read_dir(&dir)? {
            let entry: DirEntry = entry?;
            let path: PathBuf = entry.path();

            if path.is_dir() {
                let name: String = path
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .to_string();
                let parent: String = dir
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .to_string();
                let next_entry: Entry = Entry {
                    src: path.clone(),
                    art: update_art(&path),
                    parent,
                    child: Node::new(),
                };

                node.insert(name.clone(), next_entry);

                // Recurse inserted nodes until a file object is reached
                populate(
                    &path,
                    &mut node.get_mut(&name).unwrap_or(&mut Entry::default()).child,
                )?;
            } else {
                // Disallow any non-audio formats in HTML from being inserted into Node tree
                if ["mp3", "ogg", "opus", "wav", "aac", "flac"].contains(
                    &path
                        .extension()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or(""),
                ) {
                    let name: String = path
                        .file_stem()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string();
                    let parent: String = dir
                        .file_stem()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string();
                    let next_entry = Entry {
                        src: path.clone(),
                        art: update_art(&path),
                        parent,
                        child: Node::new(),
                    };

                    node.insert(name, next_entry);
                }
            }
        }
    }
    Ok(()) // return () if updates ran successfully
}
