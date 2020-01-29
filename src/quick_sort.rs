fn partition(vector: &mut [i32]) -> usize {
    let pivot = vector[vector.len()-1];
    let mut i = 0;
    for j in 0..vector.len() - 1 {
        if vector[j] < pivot {
            let tmp = vector[j];
            vector[j] = vector[i];
            vector[i] = tmp;
            i += 1;
        }
    }
    *vector.last_mut().unwrap() = vector[i];
    vector[i] = pivot;

    return i;
}

pub fn quick_sort(mut vector: &mut [i32]){
    if vector.len() < 2 { return; }

    let q = partition(&mut vector);

    quick_sort(&mut vector[..q]);
    quick_sort(&mut vector[q+1..]);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empy() {
        let mut v = vec![];
        quick_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn small() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn sorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn odd_elements() {
        let mut v = vec![3, 1, 2];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}

