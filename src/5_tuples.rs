fn main() {
    let tuple: (u8, bool, f32) = (8, true, 6.9);
    let tuple2 = (3, 5);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    println!("a: {}, b: {}, c: {}", a, b, c);
}
