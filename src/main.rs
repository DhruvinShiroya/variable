fn main() {
    //variable project
    //
    //creating the boolean variable
    //
    println!("---------------------------------------------------------------------");
    let is_corrent : bool = true;
    println!("this is boolean variable isCorrent value : {}",is_corrent);

    println!("---------------------------------------------------------------------");
    // signed integer
    let eight_byte_integer : i8 = 127;
    let sixteen_byte_integer : i16 = -3000;
    let thirtytwo_byte_integer : i32 = -255;
    let sixty_four_byte_integer : i64 = -94389255;
    let one_twenty_eight_byte_integer : i128 = -4809243255;

    println!("this is 8 bit integer value = {}", eight_byte_integer);
    println!("this is 16 bit integer value = {}", sixteen_byte_integer);
    println!("this is 32 bit integer value = {}", thirtytwo_byte_integer);
    println!("this is 64 bit integer value = {}", sixty_four_byte_integer);
    println!("this is 128 bit integer value = {}", one_twenty_eight_byte_integer);

    println!("---------------------------------------------------------------------");
    // addition
    let sum = 5 + 10 ;
    println!("sum = {} ", sum);

    // subtraction
    let diff = 10-5;
    println!("diff = {}" , diff);

    //multipliciton
    let mul = 10 *5;
    println!("multiplication  = {}" , mul);

    //division
    let division = 10/5;
    println!("division = {}",division);

    //tremainder
    let remainder = 45 % 4;
    println!("remainder = {} " , remainder);


    println!("---------------------------------------------------------------------");
    // unsigned integer
    let eight_byte_unsigned_integer : u8 = 255;
    let sixteen_byte_unsigned_integer : u16 = 255;
    let thirtytwo_byte_unsigned_integer : u32 = 255;
    let sixty_four_byte_unsigned_integer : u64 = 255;
    let one_twenty_eight_unsigned_byte_integer : u128 = 255;

    println!("this is 8 bit unsigned integer value = {}", eight_byte_unsigned_integer);
    println!("this is 16 bit unsigned integer value = {}", sixteen_byte_unsigned_integer);
    println!("this is 32 bit unsigned integer value = {}", thirtytwo_byte_unsigned_integer);
    println!("this is 64 bit unsigned integer value = {}", sixty_four_byte_unsigned_integer);
    println!("this is 128 bit unsigned integer value = {}", one_twenty_eight_unsigned_byte_integer);


    println!("---------------------------------------------------------------------");
    // float value
    let thirty_two_byte_float : f32 = 123.8942;
    let sixty_four_byte_float : f64 = 12380.1418012;
    println!("this is thirty byte float {}",thirty_two_byte_float);
    println!("this is sityfour byte float {}",sixty_four_byte_float);

    // character in rust
    let char_variable : char = 'a';
    println!("char_variable : {}",char_variable);

    println!("-----------------------------------------------");
    let mut hello_variable = String::from("Team A");
    println!("this is hello_variable = {}",hello_variable);

    // these are string methods
    println!("string length of hello_varaible {}",hello_variable.len());
    hello_variable.push('B');
    println!("string push a character in hello varaible {} ", hello_variable);
    hello_variable.push_str(" Dhruvin");
    println!("string push String method for String in hello varaible {} ",hello_variable);


    println!("---------------------------------------------------------------------");
    // create a tuple in the rust
    let _cordinates = (12,12);

    let mut gps_position = (12.23453212,18.124561245);
    let (x,y) = &gps_position;
   // println!("tuple cordinates {} ",);
    println!("tuple mutable for gp_cordinates {},{}",x,y);
    gps_position.1 = gps_position.1+ 1.0;
    gps_position.0 = gps_position.0 + 1.0;

    println!("{:?} ",gps_position);

    println!("---------------------------------------------------------------------");
    //character datatype
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!(" characters in rust {} , {} ,{} " , c,z,heart_eyed_cat);

    println!("---------------------------------------------------------------------");
    //compound datatype  are two in rust
    //tuples and arrays
    // here compound means group of multiple values

    //tuple have fixed length so once it is created it can not be resized

    let tup : (i32,u32,f64,&str) = (500,12000,74.44,"Dammu");
    println!("tuple in rust: {:?}",tup);
    println!("---------------------------------------------------------------------");
    // arrays need to have fixed type of element or same type of element unlike tuples
    // and also arrays are in fixed length so we can not resize it.


    // so arrrays are stored in the stack unlike other datastructure that are stored in heap
    //
    let a: [i32;6] = [1,2,34,56,75,78];
    let string_array : [&str;4] = ["Dhruvin","jack", "Azio","fabulous"];

    println!("print array a = {:?}",a);
    println!("prin Strnig array = {:?}",string_array);

}
