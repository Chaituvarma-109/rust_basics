#[cfg(test)]
mod test {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn test_box_smart_pointers() {

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let n1 = Box::new(Node {
            id: 1,
            next: Some(Box::new(Node { id: 2, next: None })),
        });

        dbg!(n1);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn test_reference_counters() {

        let x = Rc::new(RefCell::new(50));
        let y = Rc::clone(&x);

        *x.borrow_mut() = 70;

        // dbg!(x);
        // dbg!(y);

        dbg!(x.borrow());
        dbg!(y.borrow());

        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }

        let house_1 = Rc::new(House {
            address_number: 123,
            street: String::from("Main Street"),
            furniture: RefCell::new(vec![])
        });

        let table = Rc::new(Furniture {
            id: "table1".to_string(),
            description: String::from("Kitchen Table"),
            house: Rc::downgrade(&house_1),
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: String::from("Office Desk"),
            house: Rc::downgrade(&house_1),
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);
    }
    
}
