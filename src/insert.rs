pub fn insertion_sort<T>(v: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut vec = Vec::from(v);

    let mut i = 0;
    while i < vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j, j - 1);
            j -= 1;
        }
        i += 1;
    }

    vec
}
