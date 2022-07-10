pub mod array_sortions {

    pub fn bubble_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        let mut i = 0;
        let mut j = 0;
        let mut temp;
        while i < arr.len() {
            while j < arr.len() - 1 {
                if arr[j] > arr[j + 1] {
                    temp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = temp;
                }
                j += 1;
            }
            i += 1;
            j = 0;
        }
    }

    pub fn quick_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        if arr.len() > 1 {
            let mut i = 1;
            let mut j = 1;
            let mut temp;
            let pivot = arr[0];
            while j < arr.len() {
                if arr[j] < pivot {
                    temp = arr[i];
                    arr[i] = arr[j];
                    arr[j] = temp;
                    i += 1;
                }
                j += 1;
            }
            temp = arr[i - 1];
            arr[i - 1] = pivot;
            arr[0] = temp;
            quick_sort(&mut arr[0..i - 1]);
            quick_sort(&mut arr[i..]);
        }
    }

    pub fn selection_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        let mut i = 0;
        let mut j;
        let mut temp;
        while i < arr.len() {
            j = i;
            while j < arr.len() {
                if arr[i] > arr[j] {
                    temp = arr[i];
                    arr[i] = arr[j];
                    arr[j] = temp;
                }
                j += 1;
            }
            i += 1;
        }
    }

    pub fn heap_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        macro_rules! parent {
            ($child:ident) => {
                $child >> 1
            };
        }

        macro_rules! left_child {
            ($child:ident) => {
                ($child << 1) + 1
            };
        }

        macro_rules! right_child {
            ($child:ident) => {
                ($child + 1) << 1
            };
        }

        fn move_down<T>(arr: &mut [T], mut parent: usize)
        where
            T: Ord + Copy,
        {
            let last = arr.len() - 1;
            loop {
                let left = left_child!(parent);
                let right = right_child!(parent);
                if left > last {
                    break;
                }
                let child = if right <= last && arr[left] < arr[right] {
                    right
                } else {
                    left
                };
                if arr[child] > arr[parent] {
                    arr.swap(child, parent);
                }
                parent = child;
            }
        }

        let last_parent = parent!(len);
        for i in (0..=last_parent).rev() {
            move_down(arr, i);
        }
        for end in (1..len).rev() {
            arr.swap(0, end);
            move_down(&mut arr[..end], 0);
        }
    }

    pub fn insertion_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        let mut i = 1;
        let mut j;
        let mut temp;
        while i < arr.len() {
            j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                temp = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = temp;
                j -= 1;
            }
            i += 1;
        }
    }

    pub fn shell_sort<T>(arr: &mut [T])
    where
        T: Ord + Copy,
    {
        fn in_sort<T>(arr: &mut [T], start: usize, gap: usize)
        where
            T: Ord + Copy,
        {
            let mut i = start + gap;
            while i < arr.len() {
                let mut pos = i;
                let curr = arr[pos];
                while pos >= gap && arr[pos - gap] > curr {
                    arr[pos] = arr[pos - gap];
                    pos -= gap;
                }
                arr[pos] = curr;
                i += gap;
            }
        }

        let mut gap = arr.len() / 2;
        while gap > 0 {
            for start in 0..gap {
                in_sort(arr, start, gap);
            }
            gap /= 2;
        }
    }

    pub fn merge_sort<T>(arr: &mut [T])
    where
        T: Ord + Default + Copy,
    {
        fn merge<T>(arr: &mut [T], mid: usize)
        where
            T: Ord + Default + Copy,
        {
            let mut i = 0;
            let mut j = mid;
            let mut k = 0;
            let mut temp = vec![T::default(); arr.len()];
            while i < mid && j < arr.len() {
                if arr[i] < arr[j] {
                    temp[k] = arr[i];
                    i += 1;
                } else {
                    temp[k] = arr[j];
                    j += 1;
                }
                k += 1;
            }
            while i < mid {
                temp[k] = arr[i];
                i += 1;
                k += 1;
            }
            while j < arr.len() {
                temp[k] = arr[j];
                j += 1;
                k += 1;
            }
            arr[..k].copy_from_slice(&temp[..k]);
        }

        let len = arr.len();
        if len <= 1 {
            return;
        }
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }

    pub fn counting_sort<T>(arr: &mut [T])
    where
        T: Ord + Into<usize> + From<usize> + Copy,
    {
        if arr.len() <= 1 {
            return;
        }
        let max_num: usize = (*arr.iter().max().unwrap()).into() + 1;
        let mut counter = vec![0; max_num];
        for &v in arr.iter() {
            counter[v.into() as usize] += 1;
        }
        let mut j = 0;
        for (i, item) in counter.iter_mut().enumerate().take(max_num) {
            while *item > 0 {
                arr[j] = T::from(i);
                *item -= 1;
                j += 1;
            }
        }
    }

    pub fn radix_sort<T>(arr: &mut [T])
    where
        T: Ord + Into<usize> + From<usize> + Copy,
    {
        if arr.len() <= 1 {
            return;
        }
        let max = match arr.iter().max() {
            Some(&v) => v.into(),
            None => return,
        };
        let radix = arr.len().next_power_of_two();
        let mut place = 1;
        while place <= max {
            let digit_of = |x: T| x.into() as usize / place % radix;
            let mut counter = vec![0; radix];
            for &v in arr.iter() {
                counter[digit_of(v)] += 1;
            }
            for i in 1..radix {
                counter[i] += counter[i - 1];
            }
            for &x in arr.to_owned().iter().rev() {
                let i = digit_of(x);
                counter[i] -= 1;
                arr[counter[i]] = x;
            }
            place *= radix;
        }
    }

    pub fn bucket_sort<T>(arr: &mut [T])
    where
        T: Ord + From<usize> + Into<usize> + Copy,
    {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        use std::cmp::max;
        let max = arr.iter().cloned().fold(T::from(0), max::<T>);
        let mut buckets = vec![Vec::new(); max.into() as usize + 1];
        for &v in arr.iter() {
            buckets[v.into() as usize].push(v);
        }
        let mut j = 0;
        for bucket in buckets.iter_mut() {
            bucket.sort_by(|a, b| a.cmp(b));
            for &v in bucket.iter() {
                arr[j] = v;
                j += 1;
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    use array_sortions::bubble_sort;
    let mut arr = [4, 5, 6, 7, 1, 2, 3, 8, 9, 10];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_quick_sort() {
    use array_sortions::quick_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10];
    quick_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_selection_sort() {
    use array_sortions::selection_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10];
    selection_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_heap_sort() {
    use array_sortions::heap_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10];
    heap_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_insertion_sort() {
    use array_sortions::insertion_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_shell_sort() {
    use array_sortions::shell_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10, 0];
    shell_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_merge_sort() {
    use array_sortions::merge_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10, 0];
    merge_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_counting_sort() {
    use array_sortions::counting_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10, 0];
    counting_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_radix_sort() {
    use array_sortions::radix_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10, 0];
    radix_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_bucket_sort() {
    use array_sortions::bucket_sort;
    let mut arr = [8, 9, 4, 5, 6, 7, 1, 2, 3, 10, 0];
    bucket_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
