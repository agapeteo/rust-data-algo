use std::fmt::{Display};

enum ConsList<T> {
    Empty,
    Elem(T, Box<ConsList<T>>),
}

impl<T: PartialEq + Display> ConsList<T> {
    fn new() -> Self {
        ConsList::Empty
    }

    fn print_elements(&self) {
        let mut cur_list = self;
        while let ConsList::Elem(elem, boxed_list) = cur_list {
            println!("elem: {}", elem);
            cur_list = boxed_list;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let list_1: ConsList<i32> = ConsList::Elem(0,
                                                   Box::new(ConsList::Elem(1,
                                                                           Box::new(ConsList::Elem(2,
                                                                                                   Box::new(ConsList::Empty))))));
        list_1.print_elements();
    }
}

