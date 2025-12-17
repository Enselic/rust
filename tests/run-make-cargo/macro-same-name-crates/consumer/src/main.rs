fn main() {
    // Use the macro from version 1
    let v1 = mylib_v1::my_macro!();
    
    // Use the macro from version 2
    let v2 = mylib_v2::my_macro!();
    
    println!("v1: {}", v1);
    println!("v2: {}", v2);
    
    assert_eq!(v1, "version 1");
    assert_eq!(v2, "version 2");
}
