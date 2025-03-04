use bills::Month;
use std::collections::HashMap;

fn main() {
    let mut year: HashMap<String, Month> = HashMap::new();
    bills::run(&mut year);
}
