use bills::Month;

fn main() {
    loop {
        match bills::print_menu() {
            1 => println!("entered 1"),
            2 => println!("entered 2"),
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
    //let water = Bill::new();
    //let electricity = Bill::new();
    //let bills = vec![water, electricity];
    //let month = Month::new(bills);
    //println!("month {:?}, total: {}", month, month.total());
}

//fn new_month() -> Month {}
