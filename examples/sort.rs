use algorithms_rust::sort;

fn main() {
    let mut arr_numbers = [64, 25, 12, 22, 11];
    let mut arr_strings = ["banana", "apple", "grape", "cherry", "date"];

    println!("Array of numbers: {:?}", arr_numbers);
    println!("Array of strings: {:?}", arr_strings);

    sort::selection_sort(&mut arr_numbers);
    sort::selection_sort(&mut arr_strings);

    println!("\nSelection Sort:");
    println!("Sorted numbers: {:?}", arr_numbers);
    println!("Sorted strings: {:?}", arr_strings);
}