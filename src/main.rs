fn main() {
    let mut v = vec![3, 6, 7, 5, 2, 1, 4, 8];
    quicksort(&mut v);
    println!("{:?}", v);
}

fn quicksort<T: Clone + PartialOrd>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        let pivot = v[len / 2].clone();
        let mut left = 0;
        let mut right = len - 1;
        while left <= right {
            while v[left] < pivot {
                left += 1;
            }
            while v[right] > pivot {
                right -= 1;
            }
            if left <= right {
                v.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        quicksort(&mut v[..=right]);
        quicksort(&mut v[left..]);
    }
}
