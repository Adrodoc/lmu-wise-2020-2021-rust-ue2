// use rand::Rng;
use std::fmt::Debug;

fn main() {
    // let mut rng = rand::thread_rng();
    // let v: Vec<u32> = (0..10).map(|_| rng.gen_range(0, 10)).collect();
    let v = vec![3, 6, 7, 5, 2, 1, 4, 8];

    println!("quicksort:");
    let mut v0 = v.clone();
    quicksort(&mut v0);
    println!("{:?}", v0);

    println!("quicksort fix 1:");
    let mut v1 = v.clone();
    quicksort_fix_1(&mut v1);
    println!("{:?}", v1);

    println!("quicksort fix 2:");
    let mut v2 = v.clone();
    quicksort_fix_2(&mut v2);
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

fn quicksort_fix_1<T: Clone + PartialOrd + Debug>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        let pivot = v[0].clone();
        println!("run quicksort({:?}) -> pivot={:?}", v, pivot);
        let mut left = 0;
        let mut right = len;
        while left < right {
            while v[left] < pivot {
                left += 1;
            }
            while v[right - 1] > pivot {
                right -= 1;
            }
            if left < right {
                v.swap(left, right - 1);
                println!("swap({:?}, {:?}) -> {:?}", v[right - 1], v[left], v);
                left += 1;
                right -= 1;
            }
        }
        quicksort_fix_1(&mut v[..right]);
        quicksort_fix_1(&mut v[left..]);
    }
}

fn quicksort_fix_2<T: Clone + PartialOrd + Debug>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        let pivot = v[0].clone();
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
                if right > 0 {
                    right -= 1;
                }
            }
        }
        quicksort_fix_2(&mut v[..=right]);
        quicksort_fix_2(&mut v[left..]);
    }
}
