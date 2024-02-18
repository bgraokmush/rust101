fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let other_arr: [i32; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], arr.len());

    println!("{:?}", other_arr)
}
