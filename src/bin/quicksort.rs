fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        quicksort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date"];
        quicksort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }
}

fn main() {
    println!("Hello, world!");
    let mut numbers = vec![5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    quicksort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
