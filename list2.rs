// List as cons seq, second trial
//
// Based on list1.rs, cons tail is changed to reference.

use std::fmt;

fn main() {
    let nil = List2::Nil;
    let one = List2::Cons(1, &nil);
    let two = List2::Cons(2, &one);
    println!("one: {:?}", one);
    println!("two: {:?}", two);
    println!("two: {}", two);

}

#[derive(Debug)]
enum List2<'a, T: 'a> {
    Cons(T, &'a List2<'a, T>),
    Nil,
}

impl<'a, T> fmt::Display for List2<'a, T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List2::Cons(ref hd, ref tl) => {
                let _ = write!(f, "{hd} :: ", hd = hd);
                write!(f, "{tl} ", tl = tl)
            }
            List2::Nil => write!(f, "Nil")
        }
    }
}
