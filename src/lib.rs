mod merge;

pub use merge::merge_sort;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn sort_i32() {
        let init = vec![4, 9, 6, 2, 3, 1, 5, 7, 8];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(expected, merge_sort(&init));
    }
    #[test]
    fn sort_f64() {
        let init = vec![0.25, 1.2, 0.1, 0.69, 0.2];
        let expected = vec![0.1, 0.2, 0.25, 0.69, 1.2];
        assert_eq!(expected, merge_sort(&init));
    }

    #[test]
    fn sort_random() {
        let mut rng = rand::thread_rng();
        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
        ];
        let mut init = expected.clone();
        init.shuffle(&mut rng);

        assert_eq!(expected, merge_sort(&init));
    }
}
