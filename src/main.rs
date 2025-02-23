mod human_input {
    use std::{
        io::{self, Write},
        str::FromStr,
    };
    pub fn read_string(prompt: &str) -> io::Result<Option<String>> {
        print!("{}", prompt);
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
}

#[derive(Debug)]
struct Bill {
    name: String,
    price: f32,
    due: String,
    paid: bool,
}

#[derive(Debug)]
struct Month {
    month: String,
    bills: Vec<Bill>,
}

impl Month {
    fn new(bills: Vec<Bill>) -> Month {
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
            bills,
        }
    }

    fn total(&self) -> f32 {
        let mut total: f32 = 0.0;
        for bill in &self.bills[..] {
            total += bill.price
        }
        total
    }
    fn print_pretty(&self) {
        println!("Month: {}", self.month());
    }
}

impl Bill {
    fn new() -> Bill {
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
}

fn main() {
    let water = Bill::new();
    let electricity = Bill::new();
    let bills = vec![water, electricity];
    let month = Month::new(bills);
    println!("month {:?}, total: {}", month, month.total());
}
