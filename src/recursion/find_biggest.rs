/// Finds the biggest element in an array using recursion.
/// This function takes a slice of elements that implement the `Ord` trait and returns an `Option` containing a reference to the biggest element, or `None` if the array is empty.
/// 
/// # Examples
/// ```
/// use algorithms_rust::recursion::find_biggest;
/// let arr = [3, 1, 4, 1, 5, 9];
/// assert_eq!(find_biggest(&arr), Some(&9));
/// ```

pub fn find_biggest<T: Ord>(array: &[T]) -> Option<&T> {
    if array.is_empty() {
        return None;
    }

    let first = &array[0];
    let recursive_biggest = find_biggest(&array[1..]);

    match recursive_biggest {
        None => Some(first),
        Some(n) => {
            if first > n {
                Some(first)
            } else {
                Some(n)
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_find_biggest() {
        let arr = [3, 1, 4, 1, 5, 9];
        assert_eq!(find_biggest(&arr), Some(&9));
    }

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(find_biggest(&arr), None);
    }

    #[test]
    fn test_single_element() {
        let arr = [42];
        assert_eq!(find_biggest(&arr), Some(&42));
    }
}