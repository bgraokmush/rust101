// None, Some, Option
// None, to indicate that a value is absent
// Some, to wrap a value that is present

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None // should be use 'None' instead of 'panic!()'
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    //Unwrapping a 'Some' variant
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    println!("{:?} unwraps to {}", divide2, divide2.unwrap());
}
