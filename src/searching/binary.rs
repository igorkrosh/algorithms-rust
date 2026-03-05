/// Binary search algorithm implementation in Rust.
/// This function takes a sorted slice of elements and a target element, and returns the index of the target if found, or `None` if not found.
/// 
/// # Examples
/// ```
/// use algorithms_rust::searching::binary;
/// 
/// let arr = [1, 2, 3, 4, 5];
/// assert_eq!(binary(&arr, &3), Some(2));
/// assert_eq!(binary(&arr, &6), None);
/// ```

pub fn binary<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    let mut mid;

    while low < high {
        mid = low + (high - low) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::binary;

    #[test]
    fn found_in_middle() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary(&arr, &3), Some(2));
    }

    #[test]
    fn found_at_start() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary(&arr, &1), Some(0));
    }

    #[test]
    fn found_at_end() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary(&arr, &5), Some(4));
    }

    #[test]
    fn not_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary(&arr, &6), None);
    }

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary(&arr, &1), None);
    }

    #[test]
    fn test_single_element_found() {
        let arr = [1];
        assert_eq!(binary(&arr, &1), Some(0));
    }

    #[test]
    fn test_single_element_not_found() {
        let arr = [1];
        assert_eq!(binary(&arr, &2), None);
    }

    #[test]
    fn test_strings() {
        let arr = ["apple", "banana", "cherry", "date", "fig"];
        assert_eq!(binary(&arr, &"cherry"), Some(2));
        assert_eq!(binary(&arr, &"grape"), None);
    }
}