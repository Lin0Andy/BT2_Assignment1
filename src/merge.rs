pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);
    merge_sort(left);
    merge_sort(right);
    merge(left, right);
}

fn merge<T: Ord + Clone>(left: &mut [T], right: &mut [T]) {
    let mut merged = Vec::with_capacity(left.len() + right.len());
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
    merged.extend_from_slice(&left[left_index..]);
    merged.extend_from_slice(&right[right_index..]);

    for (i, elem) in merged.into_iter().enumerate() {
        left[i] = elem;
    }
}
