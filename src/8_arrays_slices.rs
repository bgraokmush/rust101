fn main() {
    let arr = [0, 1, 2, 3, 4];

    let slice = &arr[1..=3]; //include 3
    println!("{:?}", slice);

    barrowing_slice(arr, slice)
}

fn barrowing_slice(arr: [u8; 5], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);

    println!("length: {}", slice.len());
    println!("first: {}", slice[0]);
}
