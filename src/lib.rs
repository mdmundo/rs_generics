use std::{iter, ops::Add};

fn largest<T: Ord>(a: T, b: T) -> T {
    a.max(b)
}

fn once<T: Add>(a: T, b: T) -> impl Iterator<Item = <T as Add>::Output> {
    iter::once(a + b)
}

#[cfg(test)]
mod tests {
    use crate::{largest, once};

    #[test]
    fn char_largest() {
        let res = largest('a', 'b');
        assert_eq!(res, 'b');
    }
    #[test]
    fn u32_largest() {
        let res = largest(150u32, 200u32);
        assert_eq!(res, 200);
    }
    #[test]
    fn u32_once() {
        let res = once(1, 3).next().unwrap();
        assert_eq!(res, 4);
    }
    #[test]
    fn i128_once() {
        let res = once(5, 8).next().unwrap();
        assert_eq!(res, 13);
    }
}
