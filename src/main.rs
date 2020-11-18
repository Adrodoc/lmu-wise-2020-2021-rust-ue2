// use rand::Rng;
use std::fmt::Debug;

fn main() {
    // let mut rng = rand::thread_rng();
    // let vec: Vec<u32> = (0..10).map(|_| rng.gen_range(0, 10)).collect();
    let vec = vec![3, 6, 7, 5, 2, 1, 4, 8];
    // let vec = [1, 3, 2];

    println!("quicksort:");
    let mut vec0 = vec.clone();
    quicksort(&mut vec0);
    println!("{:?}", vec0);

    println!("quicksort fix 1:");
    let mut vec1 = vec.clone();
    quicksort_fix_1(&mut vec1);
    println!("{:?}", vec1);

    println!("quicksort fix 2:");
    let mut vec2 = vec.clone();
    quicksort_fix_2(&mut vec2);
    println!("{:?}", vec2);
}

fn quicksort<T: Clone + PartialOrd + Debug>(slice: &mut [T]) {
    let len = slice.len();
    if len > 1 {
        let pivot = slice[len / 2].clone();
        println!("run quicksort({:?}) -> pivot={:?}", slice, pivot);
        let mut left = 0;
        let mut right = len - 1;
        while left <= right {
            while slice[left] < pivot {
                left += 1;
            }
            while slice[right] > pivot {
                right -= 1;
            }
            if left <= right {
                slice.swap(left, right);
                println!("swap({:?}, {:?}) -> {:?}", slice[right], slice[left], slice);
                left += 1;
                right -= 1;
            }
        }
        quicksort(&mut slice[..=right]);
        quicksort(&mut slice[left..]);
    }
}

fn quicksort_fix_1<T: Clone + PartialOrd + Debug>(slice: &mut [T]) {
    let len = slice.len();
    if len > 1 {
        let pivot = slice[0].clone();
        println!("run quicksort({:?}) -> pivot={:?}", slice, pivot);
        let mut left = 0;
        let mut right = len;
        while left < right {
            while slice[left] < pivot {
                left += 1;
            }
            while slice[right - 1] > pivot {
                right -= 1;
            }
            if left < right {
                slice.swap(left, right - 1);
                println!(
                    "swap({:?}, {:?}) -> {:?}",
                    slice[right - 1],
                    slice[left],
                    slice
                );
                left += 1;
                right -= 1;
            }
        }
        quicksort_fix_1(&mut slice[..right]);
        quicksort_fix_1(&mut slice[left..]);
    }
}

fn quicksort_fix_2<T: Clone + PartialOrd + Debug>(slice: &mut [T]) {
    let len = slice.len();
    if len > 1 {
        let pivot = slice[0].clone();
        println!("run quicksort({:?}) -> pivot={:?}", slice, pivot);
        let mut left = 0;
        let mut right = len - 1;
        while left <= right {
            while slice[left] < pivot {
                left += 1;
            }
            while slice[right] > pivot {
                right -= 1;
            }
            if left <= right {
                slice.swap(left, right);
                println!("swap({:?}, {:?}) -> {:?}", slice[right], slice[left], slice);
                left += 1;
                if right > 0 {
                    right -= 1;
                }
            }
        }
        quicksort_fix_2(&mut slice[..=right]);
        quicksort_fix_2(&mut slice[left..]);
    }
}
