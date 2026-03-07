use algorithms_rust::search;

fn main() {
    let arr_numbers = [10, 23, 35, 47, 62, 81, 94, 108];
    let arr_strings = ["apple", "banana", "cherry", "date", "fig", "grape", "kiwi"];
    
    println!("Array of numbers: {:?}", arr_numbers);
    println!("Array of strings: {:?}", arr_strings);

    println!("\nBinary Search:");
    println!("Index of 35 in numbers: {:?}", search::binary(&arr_numbers, &35));
    println!("Index of 'cherry' in strings: {:?}", search::binary(&arr_strings, &"cherry"));

    println!("\nSmallest Element:");
    println!("Smallest number: {:?}", search::smallest(&arr_numbers));
    println!("Smallest string: {:?}", search::smallest(&arr_strings));

    println!("\nIndex of Smallest Element:");
    println!("Index of smallest number: {:?}", search::smallest_index(&arr_numbers));
    println!("Index of smallest string: {:?}", search::smallest_index(&arr_strings));
}