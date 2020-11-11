use std::fmt::Debug;

fn main() {
    let mut v = vec![3, 6, 7, 5, 2, 1, 4, 8];
    quicksort(&mut v);
    println!("{:?}", v);
}

fn quicksort<T: Clone + PartialOrd + Debug>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        let pivot = v[len / 2].clone();
        println!("run quicksort({:?}) -> pivot={:?}", v, pivot);
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
                println!("swap({:?}, {:?}) -> {:?}", v[right], v[left], v);
                left += 1;
                right -= 1;
            }
        }
        quicksort(&mut v[..=right]);
        quicksort(&mut v[left..]);
    }
}
