/// Sort using the Merge Sort algorithm.
///
/// # Examples
/// ```
/// let to_sort = vec![5, 3, 2, 4, 1];
/// assert_eq!(vec![1, 2, 3, 4, 5], sorting::merge_sort(&to_sort));
/// ```
pub fn merge_sort<T>(v: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if v.len() == 1 {
        return Vec::from(&v[..]);
    }

    let mid = v.len() / 2;
    let left = merge_sort(&v[0..mid]);
    let right = merge_sort(&v[mid..v.len()]);
    merge(&left[..], &right[..])
}

/// Merge two lists of elements.
fn merge<T: PartialOrd + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    let mut res = Vec::with_capacity(a.len() + b.len());

    let mut left = 0;
    let mut right = 0;

    loop {
        if left < a.len() && right < b.len() {
            if a[left] < b[right] {
                res.push(a[left]);
                left += 1;
            } else {
                res.push(b[right]);
                right += 1;
            }
        } else if left < a.len() {
            res.push(a[left]);
            left += 1;
        } else if right < b.len() {
            res.push(b[right]);
            right += 1;
        } else {
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_1() {
        let a = vec![1, 3, 5];
        let b = vec![2, 4];
        assert_eq!(vec![1, 2, 3, 4, 5], merge(&a[..], &b[..]));
    }
    #[test]
    fn merge_2() {
        let a = vec![1, 2, 3];
        let b = vec![];
        assert_eq!(vec![1, 2, 3], merge(&a[..], &b[..]));
    }
    #[test]
    fn merge_3() {
        let a = vec![];
        let b = vec![1, 2, 3];
        assert_eq!(vec![1, 2, 3], merge(&a[..], &b[..]));
    }
    #[test]
    fn merge_4() {
        let a = vec![1, 3, 5, 7, 9];
        let b = vec![1, 2, 3, 4, 5];
        assert_eq!(vec![1, 1, 2, 3, 3, 4, 5, 5, 7, 9], merge(&a[..], &b[..]));
    }
}
