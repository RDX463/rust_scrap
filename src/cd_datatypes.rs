//compound data types
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers in array are {:?}",numbers);
    let fruits : [&str; 4] = ["Mango","Apple","Banana","Kiwi"];
    println!("Fruits array {:?}",fruits);
    println!("First Fruit in Array {}",fruits[0]);
    println!("Second Fruit in Array {}",fruits[1]);
    println!("Third Fruit in Array {}",fruits[2]);
    println!("Fourth Fruit in Array {}",fruits[3]);
//================================================================
//Tupple Datatype
    let human : (String , i32 , bool) = ("Rohan".to_string(),22,true); //added function to convert
                                                                       //into String 
    println!("My info {:?}", human);
    let mix_tuple = ("Bats",5,false,[1,2,4,5,8]);
    println!("Mix tuple is {:?}",mix_tuple);                //prints all datatype including array  
//=================================================================
//Slices
    let number_string:&[i32] = &[1,2,3,4,5];
    println!("Number Slices : {:?}",number_string);
    
    let animal_slices:&[&str] = &["Lion","Tiger","Leopard","Cheeta"];
    println!("Animal Slices: {:?}",animal_slices);
    
    let book_slices:&[&String] = &[&"It".to_string(),&"Harry Potter".to_string(),&"ZEN".to_string()];
    println!("Book Slices: {:?}",book_slices);
    
    //Strings
    let mut stone_cold : String = String::from("hell,");
    stone_cold.push_str("yeah");
    println!("Stone cold says {}",stone_cold);
    
    //&str
    let string : String = String::from("hello ,world!");
    let slice : &str = &string[0..8];
    println!("Slice value: {}",slice);
}
