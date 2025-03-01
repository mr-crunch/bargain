use bills::{Bill, Month};
use std::collections::HashMap;

fn main() {
    let mut bills: Vec<Bill> = Vec::new();
    let mut year: HashMap<String, Month> = HashMap::new();
    bills::run(&mut bills, &mut year);
}
