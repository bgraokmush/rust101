fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(1);
    let c: MyEnum = MyEnum::C { x: 1, y: 2 };

    println!("{:?}", a); // prints "A"
    println!("{:?}", b); // prints "B(1)"
    println!("{:?}", c); // prints "C { x: 1, y: 2 }"

    if let MyEnum::B(val) = b {
        println!("val: {}", val); // prints "val: 1"
    }

    if let MyEnum::C { x, y } = c {
        println!("x: {}, y: {}", x, y); // prints "x: 1, y: 2"
    }
}

#[derive(Debug)] // derive the Debug trait
enum MyEnum {
    A,                    // unit-like
    B(i32),               // tuple-like
    C { x: i32, y: i32 }, // struct-like
}
