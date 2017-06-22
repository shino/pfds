// List as cons seq, third trial
//
// Based on list2.rs, cons tail reference is changed to Rc.

use std::rc::Rc;
use std::fmt;

fn main() {
    let nil = List3::nil();
    let one = List3::cons(1, nil.clone());
    let two = List3::cons(2, one.clone());
    println!("one: {:?}", one);
    println!("two: {:?}", two);
    println!("two: {}", two);

    let vec = vec![1,2,3];
    let three = List3::new_list(&vec);
    println!("three: {}", three);
}

#[derive(Debug)]
enum List3<T> {
    Cons(T, Rc<List3<T>>),
    Nil,
}

impl<T> List3<T> {
    fn new_list(xs: &Vec<T>) -> Rc<List3<&T>> {
        xs.iter().fold(List3::nil(), |list, r| {
            List3::cons(r, list)
        })
    }

    fn cons(x: T, list: Rc<List3<T>>) -> Rc<List3<T>> {
        Rc::new(List3::Cons(x, list))
    }

    fn nil() -> Rc<List3<T>> {
        Rc::new(List3::Nil)
    }
}


impl<T> fmt::Display for List3<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List3::Cons(ref hd, ref tl) => {
                let _ = write!(f, "{hd} :: ", hd = hd);
                write!(f, "{tl} ", tl = tl)
            }
            List3::Nil => write!(f, "Nil")
        }
    }
}

