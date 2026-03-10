/// Sum of an array using recursion.
/// This function takes a slice of integers and returns the sum of all elements in the array.
/// 
/// # Examples
/// ```
/// use algorithms_rust::recursion::sum;
/// let arr = [1, 2, 3, 4, 5];
/// assert_eq!(sum(&arr), 15);
/// ```

pub fn sum(array: &[i32]) -> i32 {
    if array.is_empty() {
        return 0;
    }

    array[0] + sum(&array[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[10, -5, 3]), 8);
    }
}