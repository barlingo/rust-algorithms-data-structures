use std::fmt::Debug;
// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        if p % 2 == 0 {
            println!("-->");
            for i in 0..v.len() - 1 {
                println!("{:?}", v);
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    sorted = false;
                }
            }
        } else {
            println!("<--");
            for i in (0..v.len() - 1).rev() {
                println!("{:?}", v);
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    sorted = false;
                }
            }
        }
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half
    // sort the right half O (n + n + n + n + )
    // bring the sorted halfs together O(n)
    // O(n * log(n))
    println!("MS->{:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);
    merge(a, b)
}

pub fn merge<T: PartialOrd + Debug>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut res = Vec::with_capacity(a.len() + b.len());
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next(); // take first element
    let mut b_peek = b_it.next(); // take first element

    loop {
        match a_peek {
            // take as a refernce so we do not consume the a_val
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        // take() takes an option and removes its value.
                        // b_peek is Some
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    println!("M->{:?}", res);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                println!("M->{:?}", res);
                return res;
            }
        }
    }
}
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = 0usize;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            println!("P->{:?}", v);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
    println!("V->{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort_even() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 3];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 6, 8, 11, 13]);
    }
    #[test]
    fn test_bubble_sort_odd() {
        let mut v = vec![4, 1, 8, 11, 13];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 4, 8, 11, 13]);
    }
    #[test]
    fn test_merge_sort_even() {
        let v = vec![4, 6, 1, 8, 11, 13, 2, 3, 0, 20, 19, 10, 100, 7];
        let v = merge_sort(v);
        assert_eq!(v, [0, 1, 2, 3, 4, 6, 7, 8, 10, 11, 13, 19, 20, 100]);
    }
    #[test]
    fn test_merge_sort_odd() {
        let v = vec![4, 1, 8, 11, 13];
        let v = merge_sort(v);
        assert_eq!(v, [1, 4, 8, 11, 13]);
    }
    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }
    #[test]
    fn test_quick_sort_even() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 2, 3];
        quick_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 6, 8, 11, 13]);
    }
    #[test]
    fn test_quick_sort_odd() {
        let mut v = vec![4, 1, 8, 11, 13];
        quick_sort(&mut v);
        assert_eq!(v, [1, 4, 8, 11, 13]);
    }
}
