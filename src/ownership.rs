//ownership,browsing and reference

//fn main(){
  //  let s1 = String::from("RUST");
    //let len = calculate_length(&s1);
    //println!("Length of '{}' is {}.",s1 , len);
//}
//fn calculate_length(s: &String) -> usize{
  //  s.len()
//}

//there can be only one owner at a time
fn main(){
    let s1 = String::from("RUST");
    let s2 = s1;

    println!("{}",s1);
}