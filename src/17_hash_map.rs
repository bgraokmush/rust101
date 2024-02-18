use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i16, &str> = HashMap::new();

    map.insert(1, "one");
    map.insert(2, "two");

    match map.get(&1) {
        Some(str) => println!("{}", str),
        _ => println!("Not one"),
    }

    match map.get(&3) {
        Some(str) => println!("{}", str),
        _ => println!("Not three"),
    }
    println!("Map: {:?}", map);
    map.remove(&1);
    println!("Map: {:?}", map);
}
