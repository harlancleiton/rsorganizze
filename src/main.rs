use std::collections::HashMap;
use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error reading input, please try again...");
    }

    let input = buffer.trim().to_owned();
    match input.is_empty() {
        true => None,
        false => Some(input),
    }
}

fn get_amount_input() -> Option<f64> {
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };

        if input.is_empty() {
            return None;
        }

        match input.parse::<f64>() {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Invalid amount, please try again.\n"),
        }
    }
}

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

    fn update(&mut self, name: &str, amount: f64) -> Option<Bill> {
        match self.inner.get_mut(name) {
            None => None,
            Some(bill) => {
                bill.amount = amount;
                Some(bill.clone())
            }
        }
    }
}

mod menu {
    use crate::{get_amount_input, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Please enter the bill name:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        println!("Please enter the bill amount:");
        let amount = match get_amount_input() {
            Some(amount) => amount,
            None => return,
        };

        bills.add(Bill { name, amount });
        println!("Bill added successfully!");
    }

    pub fn view_bills(bills: &Bills) {
        match bills.get_all().is_empty() {
            true => println!("No bills found!"),
            false => {
                for bill in bills.get_all() {
                    println!("{} - {}", bill.name, bill.amount)
                }
            }
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!("Please enter the bill name:");
        view_bills(bills);

        if bills.get_all().is_empty() {
            return;
        }

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        let removed_bill = bills.inner.remove(&name);
        match removed_bill {
            Some(_) => println!("Bill removed successfully!"),
            None => println!("Bill not found!"),
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        println!("Please enter the bill name:");
        view_bills(bills);

        if bills.get_all().is_empty() {
            return;
        }

        println!("Please enter the bill name:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        println!("Please enter the bill amount:");
        let amount = match get_amount_input() {
            Some(amount) => amount,
            None => return,
        };

        let removed_bill = bills.update(&name, amount);
        match removed_bill {
            Some(_) => println!("Bill updated successfully!"),
            None => println!("Bill not found!"),
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBills,
    EditBill,
    Exit,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBills),
            "4" => Some(Self::EditBill),
            "5" => Some(Self::Exit),
            _ => None,
        }
    }

    fn show() {
        println!();
        println!("Welcome to Bill Tracker!");
        println!();
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Edit Bill");
        println!("5. Exit");
        println!();
        println!("Please select an option:");
        println!();
    }
}

fn main() {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input().expect("Failed to read input");

        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBills) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBills) => menu::remove_bill(&mut bills),
            Some(MainMenu::EditBill) => menu::update_bill(&mut bills),
            Some(MainMenu::Exit) => break,
            None => println!("Invalid option, please try again...\n"),
        }
    }
}
