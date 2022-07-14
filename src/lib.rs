fn largest<T: Ord>(a: T, b: T) -> T {
    a.max(b)
}

#[cfg(test)]
mod tests {
    use crate::largest;

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
}
