#[derive(Debug)]
enum MyError<'a> {
    Error1(&'a str),
}

//Err, an anum that contains a value of type MyError
//Ok, an enum that contains a value of type i32
fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError<'static>> {
    if dividend % divisor != 0 {
        Err(MyError::Error1("dividend is not divisible by divisor"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let divide1 = divide(4, 5);

    match divide1 {
        Ok(result) => println!("The result is {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    let divide2 = divide(4, 2);
    println!("{}", divide2.unwrap_or(0));
}
