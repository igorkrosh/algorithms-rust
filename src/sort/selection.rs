use crate::search;

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let Some(min_index) = search::smallest_index(&arr[i..]) else {
            continue;
        };
        arr.swap(i, i + min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_selection_sort_integers() {
        let mut arr = [64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_selection_sort_strings() {
        let mut arr = ["banana", "apple", "cherry", "date"];
        selection_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: [i32; 0] = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_selection_sort_single_element() {
        let mut arr = [42];
        selection_sort(&mut arr);
        assert_eq!(arr, [42]);
    }
}