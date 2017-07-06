use std::rc::Rc;
use std::fmt;

/// PFDS 演習問題2.1
/// > suffixes [1,2,3,4] = [[1,2,3,4], [2,3,4], [3,4], [4], []]
///
/// Check
/// - [ ] correctness
/// - [ ] O(n)

fn main() {
    let vec = vec![1,2,3, 4];
    let list = List3::new_list(&vec);
    println!("list: {}", list);

    let suffixes = suffixes(&list, List3::nil());
    println!("suffixes: {}", suffixes);
    println!("suffixes: {:?}", suffixes);

    let list2 = List3::new_list(&vec);
    let sum = List3::foldl(0, |sum, x| {*sum + x}, &list2);
    let sum_rec = List3::foldl_rec(0, |sum, x| *sum + x, &list2);
    println!("sum: {}", sum);
    println!("sum_rec: {}", sum_rec);

    let strs = vec!["abc".to_string(), "def".to_string(), "xyz".to_string()];
    println!("concat: {}",
             List3::foldl("".to_string(), |res, str| format!("{}{}", res, str),
                          &List3::new_list(&strs)));
    println!("concat_rec: {}",
             List3::foldl_rec("".to_string(), |res, str| format!("{}{}", res, str),
                              &List3::new_list(&strs)));

}

fn suffixes<T>(list: &List3<T>, acc: List3<List3<T>>) -> List3<List3<T>>{
    match list {
        &List3(ref rc_cons) =>
            match **rc_cons {
                List3Segment::Nil =>
                    acc,
                List3Segment::Cons(ref hd, ref tl) =>
                    suffixes(tl, *List3::cons(tl, acc))
            }
    }
    // List3::<List3<T>>::cons(list, List3::nil())
}


#[derive(Debug)]
struct List3<T> ( Rc<List3Segment<T>> );

#[derive(Debug)]
enum List3Segment<T> {
    Cons(T, List3<T>),
    Nil,
}

impl<T : fmt::Display> List3<T> {
    fn new_list(xs: &Vec<T>) -> List3<&T> {
        xs.iter().fold(List3::nil(), |list, r| {
            List3::cons(r, list)
        })
    }

    fn foldl<U, F>(acc0: U, f: F, list: &List3<T>) -> U
        where F: Fn(&T, U) -> U {
        let mut acc = acc0;
        let mut sublist = list;
        loop {
            match sublist {
                &List3(ref rc_cons) =>
                    // rc_cons : &std::rc::Rc<List3Segment<T>>
                    match **rc_cons {
                        List3Segment::Nil =>
                            return acc,
                        List3Segment::Cons(ref hd, ref tl) => {
                            acc = f(hd, acc);
                            sublist = tl;
                        }
                    }
            }
        }
    }

    fn foldl_rec<U, F>(acc: U, f: F, list: &List3<T>) -> U
        where F: Fn(&T, U) -> U {
        match list {
            &List3(ref rc_cons) => {
                match **rc_cons {
                    List3Segment::Cons(ref hd, ref tl) =>
                        List3::foldl_rec(f(hd, acc), f, tl),
                    List3Segment::Nil =>
                        acc
                }
            }
        }
    }

    fn cons(x: T, list: List3<T>) -> List3<T> {
        List3(Rc::new(List3Segment::Cons(x, list)))
    }

    fn nil() -> List3<T> {
        List3(Rc::new(List3Segment::Nil))
    }

}


impl<T> fmt::Display for List3<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Want to use foldl and closure, but does not work.
        // ```
        // error[E0281]: type mismatch: the type `[closure@ex2_1.rs:104:29: 104:71 f:_]`
        // implements the trait         `std::ops::Fn<(_,     _ )>`,
        // but the trait        `for<'r> std::ops::Fn<(&'r T, ())>` is required
        // (expected concrete lifetime, found bound lifetime parameter )
        //
        // let _ = write!(f, "[");
        // {
        //     let write_one = |x, acc| { let _ = write!(f, "{}, ", x); };
        //     <List3<T>>::foldl((), write_one, self);
        // }
        // write!(f, "]")

        let _ = write!(f, "[");
        let mut sublist = self;
        loop {
            match sublist {
                &List3(ref rc_cons) =>
                    // rc_cons : &std::rc::Rc<List3Segment<T>>
                    match **rc_cons {
                        List3Segment::Nil =>
                            break,
                        List3Segment::Cons(ref hd, ref tl) => {
                            let _ = write!(f, "{}, ", hd);
                            sublist = tl;
                        }
                    }
            }
        }
        write!(f, "]")
    }


}
