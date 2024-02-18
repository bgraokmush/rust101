use std::vec;
// dynamic array
fn main() {
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    vec.len();
    vec[0];

    // Add an element to the end of the vector
    vec.push(6);

    vec.remove(0);

    print!("{:?}", vec);
}
