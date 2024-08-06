#[cfg(test)]
mod tests {

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_box_smart_pointer() {
        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let nodes = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node {
                id: 1,
                next: Some(Box::new(Node { id: 2, next: None })),
            })),
        });

        dbg!(nodes);
    }

    #[test]
    fn test_reference_counter() {
        let x = Rc::new(RefCell::new(50));
        let y = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x);
        dbg!(y);
    }
}
