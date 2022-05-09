use sorting::{merge_sort, quick_sort};

#[test]
fn sort_i32() {
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let init = vec![4, 9, 6, 2, 3, 1, 5, 7, 8];
    assert_eq!(expected, merge_sort(&init));
    assert_eq!(expected, quick_sort(&init));
}
#[test]
fn sort_f64() {
    let expected = vec![0.1, 0.2, 0.25, 0.69, 1.2];
    let init = vec![0.25, 1.2, 0.1, 0.69, 0.2];
    assert_eq!(expected, merge_sort(&init));
    assert_eq!(expected, quick_sort(&init));
}
