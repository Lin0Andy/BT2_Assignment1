pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    merge_sort(left);
    merge_sort(right);
    merge(left, right, arr);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], arr: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut arr_index = 0;
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            arr[arr_index] = left[left_index].clone();
            left_index += 1;
        } else {
            arr[arr_index] = right[right_index].clone();
            right_index += 1;
        }
        arr_index += 1;
    }
    while left_index < left.len() {
        arr[arr_index] = left[left_index].clone();
        left_index += 1;
        arr_index += 1;
    }
    while right_index < right.len() {
        arr[arr_index] = right[right_index].clone();
        right_index += 1;
        arr_index += 1;
    }
}
