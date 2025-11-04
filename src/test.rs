fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test1(){
    assert_eq!(add_i32(2, 3), 5);
}

#[test]
fn test2(){
    assert_eq!(add_i32(-2, 3), -1);
}