pub fn quicksort(arr: &mut [i32]){
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
}

fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize){
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot - 1);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

