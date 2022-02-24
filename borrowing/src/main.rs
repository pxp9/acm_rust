fn out_print(s: &String) {
    println!("\"{}\" its printed from out_print !", s);
}
fn main() {
    let mut z: String = String::from("hi");
    out_print(&z);
    z = String::from("hello");
    //*(&mut z) = String::from("hello");
    //let h: &mut String = &mut z;
    //let j: &mut String = &mut z;
    println!("\"{}\" its printed from main!", z);
    z.push_str("_there");
    println!("\"{}\" its printed from main!", z);
}
