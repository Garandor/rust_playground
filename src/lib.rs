use std::ops::Add;

pub fn add_int<'a, T>(x: &'a T, y: &'a T) -> T
where
    &'a T: Add<&'a T, Output = T>,
{
    x + y
}
// A helper function `distance_test` will need.
pub fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        let a = 1u8;
        let b = 4u8;
        assert_eq!(a + b, add_int(&a, &b));
        let a = 1u64;
        let b = 4u64;
        assert_eq!(a + b, add_int(&a, &b));
        let a = 1f64;
        let b = 4f64;
        assert_eq!(a + b, add_int(&a, &b));
    }
}