// Topic: Ownership
//
//

// * Use a struct for the grocery item
struct Item {
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &Item) {
    println!("Quantity: {:?}", item.quantity);
}

// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &Item) {
    println!("ID: {:?}", item.id);
}

fn main() {
    let item1 = Item {
        quantity: 2,
        id: 1234,
    };

    // * Print out the quantity and id number of a grocery item
    display_quantity(&item1);
    display_id(&item1);
}
