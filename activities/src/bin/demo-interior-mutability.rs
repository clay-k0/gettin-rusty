// Demo: Interior Mutability: Cell & RefCell
//
// Note that mut and &mut should be preferred.
// Cell & RefCell should be used when it's not possbile to express your intetions otherwise
//
// Interior mutability is NOT thread safe (cannot be used in multithreaded code).

use std::cell::{Cell, RefCell};

struct Book {
    signed: Cell<bool>,
    title: RefCell<String>,
}

impl Book {
    fn sign(&self) {
        self.signed.set(true)
    }

    fn signed(&self) -> bool {
        self.signed.get()
    }
}

fn main() {
    // Book demonstrating Cell
    let untitled_book = Book {
        signed: Cell::new(false),
        title: RefCell::new(".".to_owned()),
    };

    println!("---Untitled Book---");
    println!("Book Signed? {:?}", untitled_book.signed());
    untitled_book.sign();
    println!("Signed Book");
    println!("Book Signed? {:?}", untitled_book.signed());

    // Book demonstrating RefCell
    let titled_book = Book {
        signed: Cell::new(false),
        title: RefCell::new("The Alchemist".to_owned()),
    };

    {
        println!("\n---Titled Book---");
        // Borrow book title immutably
        let borrowed_title = titled_book.title.borrow();
        println!("Title: {:?}", *borrowed_title);
    }

    // If the following two pieces of code were ran in the same scope,
    // we'd get a runtime panic since we're double mutating

    // Borrow book title mutably and mutate it
    let mut mutable_title = titled_book.title.borrow_mut();
    *mutable_title = "The Myth of Sisyphus".to_owned();
    println!("Title: {:?}", *mutable_title);

    //     titled_book.title.replace("New Title 2".to_owned());
    //     println!("Title: {:?}", titled_book.title);

    // To avoid panicking, we could use try_borrow() and try_borrow_mut()
    // let title: Result<_, _> = titled_book.title.try_borrow();
    // let title: Result<_, _> = titled_book.title.try_borrow_mut();
}
