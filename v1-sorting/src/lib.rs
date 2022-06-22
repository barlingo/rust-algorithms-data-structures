use std::fmt::Debug;
// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        if p % 2 == 0 {
            for i in (0..v.len() - 1 - p).rev() {
                println!("{:?}", v);
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    sorted = false;
                }
            }
        } else {
            for i in 0..v.len() - 1 - p {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
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
}
