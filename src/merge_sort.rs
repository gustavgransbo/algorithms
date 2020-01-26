fn merge(left_vector: &[i32], right_vector: &[i32], result: &mut [i32]){

    let mut left_id = 0;
    let mut right_id = 0;
    let mut insert_id = 0;

    while left_id < left_vector.len() && right_id < right_vector.len() {
        if left_vector[left_id] < right_vector[right_id] {
            result[insert_id] = left_vector[left_id];
            left_id += 1;
        } else {
            result[insert_id] = right_vector[right_id];
            right_id += 1;
        }
        insert_id += 1;
    }
    if left_id < left_vector.len() {
        result[insert_id..].copy_from_slice(&left_vector[left_id..]);
    } else {
        result[insert_id..].copy_from_slice(&right_vector[right_id..]);
    }
}

pub fn merge_sort(vector: &mut [i32]){
    let mid = vector.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut vector[..mid]);
    merge_sort(&mut vector[mid..]);

    let mut intermediary_vector = vector.to_vec();

    merge(&vector[..mid], &vector[mid..], &mut intermediary_vector);

    vector.copy_from_slice(&intermediary_vector);
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empy() {
        let mut v = vec![];
        merge_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn small() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn sorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn odd_elements() {
        let mut v = vec![3, 1, 2];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}

