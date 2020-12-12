pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: PartialOrd,
{
    let mut size = arr.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        if arr[mid] <= *target {
            base = mid
        }
        size -= half;
    }
    if arr[base] == *target {
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}