use std::{ops::Add};

fn add_int<'a,'b,T>(x: &'a T,y: &'a T) -> T
where
    &'a T: Add<&'a T, Output=T>
{
    x + y
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_add(){
    let a= 1u8;
    let b= 4u8;
    assert_eq!(a+b,add_int(&a,&b));
    let a= 1u64;
    let b= 4u64;
    assert_eq!(a+b,add_int(&a,&b));
    let a= 1f64;
    let b= 4f64;
    assert_eq!(a+b,add_int(&a,&b));
}