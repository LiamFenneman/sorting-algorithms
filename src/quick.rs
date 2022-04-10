/// Sort using the Quick Sort algorithm.
///
/// # Examples
/// ```
/// let to_sort = vec![5, 3, 2, 4, 1];
/// assert_eq!(vec![1, 2, 3, 4, 5], sorting::quick_sort(&to_sort));
/// ```
pub fn quick_sort<T>(v: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut v = Vec::from(v);

    quick_sort_mut(&mut v);

    v
}

fn quick_sort_mut<T>(v: &mut [T])
where
    T: PartialOrd + Copy,
{
    if !v.is_empty() {
        let pi = partition(v);
        let len = v.len();

        quick_sort_mut(&mut v[0..pi]);
        quick_sort_mut(&mut v[pi + 1..len]);
    }
}

fn partition<T>(v: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let len = v.len();
    let pivot = v[len - 1];
    let mut a = 0;
    let mut b = 0;

    while b < len - 1 {
        if v[b] <= pivot {
            v.swap(a, b);
            a += 1;
        }
        b += 1;
    }

    v.swap(a, len - 1);

    a
}
