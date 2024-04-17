pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    let mut merged = Vec::with_capacity(arr.len());
    merge_sort(left);
    merge_sort(right);
    merge(left, right, &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], merged: &mut Vec<T>) {
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            merged.push(left[left_index].clone());
            left_index += 1;
        } else {
            merged.push(right[right_index].clone());
            right_index += 1;
        }
    }
    // Copy remaining elements from left and right if any
    merged.extend_from_slice(&left[left_index..]);
    merged.extend_from_slice(&right[right_index..]);
}
