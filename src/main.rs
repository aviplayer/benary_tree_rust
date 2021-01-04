use rand::Rng;

use crate::binary::Tree;

mod binary;

fn main() {
    let mut rng = rand::thread_rng();
    let mut tree = Tree::new();
    let mut to_remove: i32 = 0;
    for _ in 1..20 {
        let i: i32 = rng.gen_range(1..20);
        println!("Adding new el {}", i);
        tree.add(i);
        if i == 10 {
            to_remove = i.clone();
        }
    }
    tree.print();
    //println!("Result: \n {:?}", tree);
    tree.remove(to_remove);
    println!("Result after remove: ");
    tree.print();
}
