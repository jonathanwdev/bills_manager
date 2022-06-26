use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug)]
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

    fn remove(&mut self, bill_name: &str) -> bool {
        self.inner.remove(bill_name).is_some()
    }

    fn update(&mut self, bill_name: &str, amount: f64) -> bool {
        match self.inner.get_mut(bill_name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_user_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again !");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Bill amount: ");
    loop {
        let input = match get_user_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parse_input: Result<f64, _> = input.parse();
        match parse_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("please, enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_user_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_user_input() {
            Some(input) => input,
            None => return,
        };
        println!("");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added ");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!(" ------------ ");
            println!("Bill name: {:?}", bill.name);
            println!("Bill amount: {:?}", bill.amount);
            println!(" ------------ ");
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove: ");
        let bill_name = match get_user_input() {
            Some(name) => {
                println!("{:?}", name);
                name
            }
            None => return,
        };

        if bills.remove(&bill_name) {
            println!("");
            println!("bill removed !");
        } else {
            println!("");
            println!("bill not found !");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to update: ");
        let name = match get_user_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            println!("Updated !");
        } else {
            println!("Bill not found !");
        }
    }
}

enum MainMenu {
    Addbill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::Addbill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter selection: ");
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_user_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::Addbill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => {
                println!("Invalid menu");
                break;
            }
        }
    }
    None
}

fn main() {
    run_program();
}
