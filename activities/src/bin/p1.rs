// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter the data again.")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number."),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill Added.")
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter Bill Name To Remove: ");

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill Removed.");
        } else {
            println!("Bill Not Found");
        }
    }
    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }

        println!("Enter Bill To Update: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            println!("Bill Updated.");
        } else {
            println!("Bill Not Found");
        }
    }
}

enum MainMenu {
    AddBill,
    ViewVill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewVill),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("Bill Manager");
        println!("--------");
        println!("(1) Add Bill");
        println!("(2) View Bills");
        println!("(3) Remove Bill");
        println!("(4) Update Bill");
        println!("");
        println!("Enter Selection: ");
    }
}

fn run_program() -> Option<()> {
    // Create bill structure
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;

        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewVill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
