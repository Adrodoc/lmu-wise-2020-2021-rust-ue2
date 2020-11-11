use std::fmt::Debug;

fn main() {
    let v = vec![3, 6, 7, 5, 2, 1, 4, 8];

    println!("quicksort:");
    let mut v1 = v.clone();
    quicksort(&mut v1);
    println!("{:?}", v1);

    println!("quicksort_hoare:");
    let mut v2 = v.clone();
    quicksort_hoare(&mut v2);
    println!("{:?}", v2);
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

fn quicksort_hoare<T: Clone + PartialOrd + Debug>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        let pivot = v[len / 2].clone();
        println!("run quicksort({:?}) -> pivot={:?}", v, pivot);
        let mut left = 0;
        let mut right = len - 1;
        loop {
            while v[left] < pivot {
                left += 1;
            }
            while v[right] > pivot {
                right -= 1;
            }
            if left < right {
                v.swap(left, right);
                println!("swap({:?}, {:?}) -> {:?}", v[right], v[left], v);
            } else {
                break;
            }
        }
        quicksort_hoare(&mut v[..right]);
        quicksort_hoare(&mut v[right..]);
    }
}
