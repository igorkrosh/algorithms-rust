/// Implementation of the quick sort algorithm in Rust.
/// 
/// # Example
/// ```
/// use algorithms_rust::sort::quick_sort;
/// let mut arr = [64, 25, 12, 22, 11];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [11, 12, 22, 25, 64]);
/// ```

pub fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    
    let len = array.len();
    let pivot_index = len / 2;

    array.swap(pivot_index, len - 1);
    
    let mut store_index = 0;
    for i in 0..len-1 {
        if array[i] <= array[len - 1] {
            array.swap(i, store_index);
            store_index += 1;
        }
    }

    array.swap(store_index, len - 1);

    quick_sort(&mut array[0..store_index]);
    quick_sort(&mut array[store_index + 1..]);

    return;
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test_quick_sort_integers() {
        let mut arr = [64, 25, 12, 22, 11];
        quick_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_quick_sort_strings() {
        let mut arr = ["banana", "apple", "cherry", "date", "grape"];
        quick_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cherry", "date", "grape"]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = [42];
        quick_sort(&mut arr);
        assert_eq!(arr, [42]);
    }
}