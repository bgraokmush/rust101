fn main() {
    //unsignet integer
    let a: u8 = 10;

    //signed integer
    let b: i8 = -10;

    //floating point
    let c: f32 = 10.0;

    println!("unsign: {}, sign: {}, float: {}", a, b, c);

    //char - can only hold a single character
    let d = 'a';
    let emoji = "\u{1F600}";

    println!("letter: {}, emoji: {}", d, emoji);

    let is_true = true;

    print!("is_true: {}", is_true);
}
