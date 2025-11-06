fn myprint<T: std::fmt::Display>(val: T) {
    println!("{}", val);
}

fn main() {
    // let x = "hello".to_string();
    // myprint(x);
    // myprint(x); // エラー: xはすでにムーブされている
    
    let x = "hello".to_string();
    let xx = x.clone();
    myprint(x);
    myprint(xx); // エラー: xはすでにムーブされている
}
