fn myclear(val: &mut String) {
    val.clear();
}

fn main() {
    let mut x = "hello".to_string();
    println!("before myclear: {}", x);

    let s_ref = &mut x;
    //let s_ref2 = &mut x; // これはできない
    myclear(s_ref);
    println!("after myclear: {}", x);
}
