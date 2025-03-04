mod human_input {
    use std::{
        io::{self, Write},
        str::FromStr,
    };
    pub fn read_string(prompt: &str) -> io::Result<Option<String>> {
        println!("{}", prompt);
        print!(">> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(if input.trim() == "" {
            None
        } else {
            Some(input.trim().to_string())
        })
    }

    pub fn read_string_checked(prompt: &str) -> io::Result<String> {
        loop {
            match read_string(prompt)? {
                Some(input) => return Ok(input.to_string()),
                None => println!("input cannot be empty"),
            }
        }
    }

    pub fn read_typed_checked<T: FromStr>(prompt: &str) -> io::Result<T> {
        loop {
            let untyped_input = read_string_checked(prompt)?;
            match untyped_input.parse::<T>() {
                Ok(input) => return Ok(input),
                Err(_) => println!("{} is not valid", untyped_input),
            }
        }
    }

    pub fn read_menu(prompt: &str, options: &[impl AsRef<str>]) -> io::Result<usize> {
        loop {
            println!("{}", prompt);
            let mut options_iter = options.iter().enumerate();
            if let Some((_, option)) = options_iter.next() {
                println!("\t1: {}", option.as_ref());
            }
            for (idx, option) in options_iter {
                println!("\t{}: {}", idx + 1, option.as_ref());
            }
            print!(">> ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();
            if choice.is_empty() {
                return Ok(1);
            }
            match choice.parse::<usize>() {
                Ok(a) => {
                    if a == 0 {
                        println!("0 is not a valid option");
                    } else {
                        let choice = a;
                        if (choice - 1) < options.len() {
                            return Ok(choice);
                        } else {
                            println!("{} is not a valid option (too big)", choice);
                        }
                    }
                }
                Err(_) => {
                    println!("{} is not a valid option", choice);
                }
            }
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Bill {
    pub name: String,
    pub price: f32,
    pub due: String,
    pub paid: bool,
}

#[derive(Debug)]
pub struct Month {
    pub month: String,
    pub bills: Vec<Bill>,
}

impl Month {
    pub fn new() -> Month {
        Month {
            month: {
                match human_input::read_string_checked("enter month: ") {
                    Ok(input) => input,
                    Err(error) => {
                        println!("error: {:?}", error);
                        String::from("Na")
                    }
                }
            },
            bills: Self::make_bills(),
        }
    }

    pub fn total(&self) -> f32 {
        let mut total: f32 = 0.0;
        for bill in &self.bills[..] {
            total += bill.price
        }
        total
    }
    //fn print_pretty(&self) {
    //    println!("Month: {}", self.month());
    //}
    fn make_bills() -> Vec<Bill> {
        let mut bills = Vec::new();
        match human_input::read_typed_checked::<i32>("enter bill count:") {
            Ok(count) => {
                for i in 1..=count {
                    bills.push(Bill::default())
                }
                bills
            }
            Err(error) => {
                println!("error initializing bills...");
                eprintln!("error: {:?}", error);
                vec![Bill::default()]
            }
        }
    }

    pub fn print_bills(&self) {
        for bill in &self.bills {
            bill.print_bill()
        }
    }
}

impl Bill {
    pub fn new() -> Bill {
        Bill {
            name: {
                match human_input::read_string_checked("input name: ") {
                    Ok(input) => input,
                    Err(error) => {
                        println!("error: {:?}", error);
                        String::from("input error")
                    }
                }
            },
            price: {
                match human_input::read_typed_checked("input price: ") {
                    Ok(input) => input,
                    Err(error) => {
                        println!("error: {:?}", error);
                        0.0
                    }
                }
            },
            due: {
                match human_input::read_string_checked("input due date: ") {
                    Ok(input) => input,
                    Err(error) => {
                        println!("error: {:?}", error);
                        String::from("input error")
                    }
                }
            },
            paid: false,
        }
    }

    pub fn print_bill(&self) {
        println!(
            "bill: {} price: {} due date: {} paid: {}",
            self.name, self.price, self.due, self.paid
        );
    }
}

impl Default for Bill {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Month {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_menu() -> usize {
    loop {
        match human_input::read_menu("enter choice:", &["make new month", "print", "exit"]) {
            Ok(choice) => return choice,
            Err(error) => {
                eprintln!("error: {:?}", error);
                continue;
            }
        }
    }
}

pub fn print_year(year: &HashMap<String, Month>) {}

pub fn run(year: &mut HashMap<String, Month>) {
    loop {
        match print_menu() {
            1 => match human_input::read_string_checked("enter month:") {
                Ok(month) => {
                    year.entry(month).or_default();
                }
                Err(error) => {
                    eprintln!("error: {:?}", error);
                }
            },
            2 => match human_input::read_string_checked("enter month to print:") {
                Ok(month) => match year.get(&month) {
                    Some(month) => month.print_bills(),
                    None => {
                        println!("month not found");
                        continue;
                    }
                },
                Err(error) => {
                    eprintln!("error: {:?}", error);
                }
            },
            //println!("entered 2"),
            3 => {
                println!("exiting...");
                break;
            }
            _ => {
                println!("no choice... exiting...");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
