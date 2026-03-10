use algorithms_rust::recursion;

fn main() {
    let arr_numbers = [10, 23, 35, 47, 62, 81, 94, 108];

    println!("Array of numbers: {:?}", arr_numbers);
    println!("\nSum of the array: {}", recursion::sum(&arr_numbers));
}