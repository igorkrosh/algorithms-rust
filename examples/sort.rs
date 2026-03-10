use algorithms_rust::sort;

fn main() {
    let arr_numbers = [64, 25, 12, 22, 11];
    let arr_strings = ["banana", "apple", "grape", "cherry", "date"];

    println!("Array of numbers: {:?}", arr_numbers);
    println!("Array of strings: {:?}", arr_strings);

    println!("\nSelection Sort:");

    let mut selection_sorted_numbers = arr_numbers.clone();
    let mut selection_sorted_strings = arr_strings.clone();

    sort::selection_sort(&mut selection_sorted_numbers);
    sort::selection_sort(&mut selection_sorted_strings);

    println!("Sorted numbers: {:?}", selection_sorted_numbers);
    println!("Sorted strings: {:?}", selection_sorted_strings);

    println!("\nQuick Sort:");

    let mut quick_sorted_numbers = arr_numbers.clone();
    let mut quick_sorted_strings = arr_strings.clone();

    sort::quick_sort(&mut quick_sorted_numbers);
    sort::quick_sort(&mut quick_sorted_strings);

    println!("Sorted numbers: {:?}", quick_sorted_numbers);
    println!("Sorted strings: {:?}", quick_sorted_strings);

}