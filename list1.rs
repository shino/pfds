// List as cons seq, first trial
//
// Based on the example in std::boxed
//   https://doc.rust-lang.org/std/boxed/
// Weak points:
// - One can not share intermediate lists (sublists).
//   See error when printing `one`.

use std::fmt;

fn main() {
    let nil = List1::Nil;
    let one = List1::Cons(1, Box::new(nil));
    let two = List1::Cons(2, Box::new(one));

    // `one` cannnot be used here,
    // error message: error[E0382]: use of moved value: `one`
    // println!("one: {:?}", one);

    println!("two (Debug)  : {:?}", two);
    println!("two (Display): {}", two);
}

#[derive(Debug)]
enum List1<T> {
    Cons(T, Box<List1<T>>),
    Nil,
}

impl<T> fmt::Display for List1<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List1::Cons(ref hd, ref tl) => {
                let _ = write!(f, "{hd} :: ", hd = hd);
                write!(f, "{tl} ", tl = tl)
            }
            List1::Nil => write!(f, "Nil")
        }
    }
}
