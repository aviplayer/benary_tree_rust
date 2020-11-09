use List::*;
use std::fmt::Display;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    
    fn new() -> List<T> {
        Nil
    }
    
    fn push(self, elem: T) -> List<T> {
        Cons(elem, Box::new(self))
    }
    fn pop<'a>(&'a self) -> (&'a T, &List<T>) {
        match &self {
            Cons(res, ref tail) => {
                let list_ref = tail;
                (res, list_ref)
            }
            Nil => panic!("Empty list")
        }
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String
        where T: Display + Clone
    {
        match &self {
            Cons(head, ref tail) => {
                let heap_str = tail.stringify();
                format!("{}, {}", head, heap_str)
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    
    list = list.push(1);
    list = list.push(2);
    list = list.push(3);
    list = list.push(4);
    list = list.push(5);


    println!("Size of Linked List: {}", list.len());
    println!("{}", list.stringify());

    let(e, list) = list.pop();
    println!("El {}", e);
    println!("{}", list.stringify());

    let(e, list) = list.pop();
    println!("El {}", e);
    println!("{}", list.stringify());
    
    let mut list_str = List::new();
    list_str = list_str.push("one");
    list_str = list_str.push("two");
    list_str = list_str.push("tree");
    list_str = list_str.push("four");
    list_str = list_str.push("five");
    
    println!("{}", list_str.stringify());
    
    let(e, list_str) = list_str.pop();
    println!("El {}", e);
    println!("{}", list_str.stringify());

    let(e, list_str) = list_str.pop();
    println!("El {}", e);
    println!("{}", list_str.stringify());
}
