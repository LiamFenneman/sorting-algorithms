use sorting::{insertion_sort, merge_sort, quick_sort};

fn test_i32<F>(f: F)
where
    F: Fn(&[i32]) -> Vec<i32>,
{
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let init = vec![4, 9, 6, 2, 3, 1, 5, 7, 8];
    assert_eq!(expected, f(&init));
}

fn test_f64<F>(f: F)
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    let expected = vec![0.1, 0.2, 0.25, 0.69, 1.2];
    let init = vec![0.25, 1.2, 0.1, 0.69, 0.2];
    assert_eq!(expected, f(&init));
}

#[test]
fn merge() {
    test_i32(|i| merge_sort(i));
    test_f64(|i| merge_sort(i));
}

#[test]
fn quick() {
    test_i32(|i| quick_sort(i));
    test_f64(|i| quick_sort(i));
}

#[test]
fn insert() {
    test_i32(|i| insertion_sort(i));
    test_f64(|i| insertion_sort(i));
}
