// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//

// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(250.0, String::from("Jeremy")),
        Ticket::Standard(40.0),
        Ticket::Vip(100.0, String::from("Clay")),
    ];

    for ticket in tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage Ticket: ${:?}, Holder: {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Standard Ticket: ${:?}", price),
            Ticket::Vip(price, holder) => {
                println!("Vip Ticket: ${:?}, Holder: {:?}", price, holder)
            }
        }
    }
}
