fn main() {
    let str: &str = "Hello, world!";

    let mut string: String = String::from("Hello from the heap!");

    let slice = &string[..6];
    let _ = slice.len();

    string.push_str(" world!");

    string = string.replace("Hello", "Goodbye");

    println!("{}", string);
}
