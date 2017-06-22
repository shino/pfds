// List as cons seq, second trial
//
// Based on list1.rs, cons tail is changed to reference.
// Weak point:
// - Explicit lifetyme parameter is needed, complicated
//   and too much burden for me. I don't know it is possible
//   or not at the moment.

use std::fmt;

fn main() {
    let nil = List2::Nil;
    let one = List2::Cons(1, &nil);
    let two = List2::Cons(2, &one);
    println!("one: {:?}", one);
    println!("two: {:?}", two);
    println!("two: {}", two);

    let three = List2::new_list(vec![1,2,3]);
    println!("three: {}", three);

}

#[derive(Debug)]
enum List2<'a, T: 'a> {
    Cons(T, &'a List2<'a, T>),
    Nil,
}

// TODO: can not implement convert fucntion from vector.
//   Is it possible or not??
// impl<'a, T> List2<'a, T> {
//     fn new_list(xs: Vec<T>) -> &'a List2<'a, T>
//         where T: std::fmt::Display {
//         let nil : List2<'a, T> = List2::Nil;
//         let mapped = xs.iter().fold(nil, |list, &x| {
//             List2::Cons(x, &list)
//         });
//         &mapped
//     }
// }


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

