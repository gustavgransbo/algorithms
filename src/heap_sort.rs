struct InPlaceHeap<'a, T: PartialOrd + Copy> {
    elements: &'a mut [T],
    heap_size: usize,
}

impl<T: PartialOrd + Copy> InPlaceHeap<'_, T> {
    pub fn heapify(&mut self, i: usize) {
        let l = InPlaceHeap::<T>::left(i);
        let r = InPlaceHeap::<T>::right(i);
        let mut largets_element = i;
        if l < self.heap_size && self.elements[l] > self.elements[i] {
            largets_element = l;
        }
        if r < self.heap_size && self.elements[r] > self.elements[largets_element] {
            largets_element = r;
        }
        if largets_element != i {
            self.elements.swap(i, largets_element);
            self.heapify(largets_element);
        }
    }

    pub fn heap_sort(&mut self) {
        while self.heap_size > 0 {
            self.heap_size -= 1;
            self.elements.swap(0, self.heap_size);
            self.heapify(0);
        }
    }

    pub fn build_heap(vector: &mut [T]) -> InPlaceHeap<'_, T> {
        let size = vector.len();
        let mut heap = InPlaceHeap {
            elements: vector,
            heap_size: size,
        };
        for i in (0..size / 2).rev() {
            heap.heapify(i);
        }
        heap
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * (i + 1)
    }
}

pub fn heap_sort<T: PartialOrd + Copy>(vector: &mut [T]) {
    let mut heap = InPlaceHeap::build_heap(vector);
    heap.heap_sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut v: Vec<i32> = vec![];
        heap_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn small() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn sorted() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn odd_elements() {
        let mut v = vec![3, 1, 2];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn sort_chars() {
        let mut v = vec!['a', 'b', 'b', 'a'];
        heap_sort(&mut v);
        assert_eq!(v, vec!['a', 'a', 'b', 'b']);
    }
}
