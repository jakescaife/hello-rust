use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("Count after creating A = {}", Rc::strong_count(&a));

    let _ = Cons(3, Rc::clone(&a));

    println!("Count after creating B = {}", Rc::strong_count(&a));

    {
        let _ = Cons(4, Rc::clone(&a));

        println!("Count after creating C = {}", Rc::strong_count(&a));
    }

    println!("Count after C goes out of scope = {}", Rc::strong_count(&a));
}
