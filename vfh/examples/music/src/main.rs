extern crate vfh;

use vfh::music::*;

fn main() {
    let config: Config = Config::new().unwrap();
    let mut tree: Node = Node::new();
    let _ = populate(&config.base, &mut tree);
    dbg!(tree);
}
