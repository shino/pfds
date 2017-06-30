use std::rc::Rc;
use std::fmt;

fn main() {
    let vec = vec![1,2,3];
    let list = List3::new_list(&vec);
    println!("list: {}", list);

    let suffixes = suffixes(list);
    println!("suffixes: {}", suffixes);
    println!("suffixes: {:?}", suffixes);
}

fn suffixes<T>(list: List3<T>) -> List3<List3<T>>{
    List3::<List3<T>>::cons(list, List3::nil())
}


#[derive(Debug)]
struct List3<T> ( Rc<List3Cons<T>> );

#[derive(Debug)]
enum List3Cons<T> {
    Cons(T, List3<T>),
    Nil,
}

impl<T> List3<T> {
    fn new_list(xs: &Vec<T>) -> List3<&T> {
        xs.iter().fold(List3::nil(), |list, r| {
            List3::cons(r, list)
        })
    }

    fn cons(x: T, list: List3<T>) -> List3<T> {
        List3(Rc::new(List3Cons::Cons(x, list)))
    }

    fn nil() -> List3<T> {
        List3(Rc::new(List3Cons::Nil))
    }
}


impl<T> fmt::Display for List3<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &List3(ref cons_rc) =>
                match **cons_rc {
                    List3Cons::Cons(ref hd, ref tl) => {
                        let _ = write!(f, "({hd}, ", hd = hd);
                        write!(f, "{tl})", tl = tl)
                    }
                    List3Cons::Nil => write!(f, "Nil")
                }
        }
    }
}
