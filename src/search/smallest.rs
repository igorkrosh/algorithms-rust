/// Finds the smallest element in an array. Returns `None` if the array is empty.
/// 
/// # Examples
/// ```
/// use algorithms_rust::search::smallest;
/// let arr = [5, 3, 8, 1, 4];
/// assert_eq!(smallest(&arr), Some(&1));
/// ```

pub fn smallest<T: Ord>(arr: &[T]) -> Option<&T> {
    if arr.is_empty() {
        return None;
    }

    let mut min = &arr[0];

    for item in arr.iter() {
        if item < min {
            min = item;
        }
    }

    Some(min)
}

/// Finds the index of the smallest element in an array. Returns `None` if the array is empty.
/// 
/// # Examples
/// ```
/// use algorithms_rust::search::smallest_index;
/// let arr = [5, 3, 8, 1, 4];
/// assert_eq!(smallest_index(&arr), Some(3));
/// ```

pub fn smallest_index<T: Ord>(arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut min_index = 0;

    for (index, item) in arr.iter().enumerate() {
        if item < &arr[min_index] {
            min_index = index;
        }
    }

    Some(min_index)
}

#[cfg(test)]
mod tests {
    use super::smallest;

    #[test]
    fn test_smallest_integers() {
        let arr = [5, 3, 8, 1, 4];
        assert_eq!(smallest(&arr), Some(&1));
    }

    #[test]
    fn test_smallest_strings() {
        let arr = ["banana", "apple", "cherry"];
        assert_eq!(smallest(&arr), Some(&"apple"));
    }

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(smallest(&arr), None);
    }

    #[test]
    fn test_single_element() {
        let arr = [42];
        assert_eq!(smallest(&arr), Some(&42));
    }

    #[test]
    fn test_smallest_index_integers() {
        let arr = [5, 3, 8, 1, 4];
        assert_eq!(super::smallest_index(&arr), Some(3));
    }

    #[test]
    fn test_smallest_index_strings() {
        let arr = ["banana", "apple", "cherry"];
        assert_eq!(super::smallest_index(&arr), Some(1));
    }

    #[test]
    fn test_smallest_index_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(super::smallest_index(&arr), None);
    }

    #[test]
    fn test_smallest_index_single_element() {
        let arr = [42];
        assert_eq!(super::smallest_index(&arr), Some(0));
    }
}