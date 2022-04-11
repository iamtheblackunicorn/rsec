use rsec::*;

fn main(){
    println!("{:?}", is_num(String::from("2")));
    println!("{:?}", is_num(String::from("a")));
    println!("{:?}", char_pos(String::from("b")));
    println!("{:?}", conv_string_to_usize(String::from("2")));
    println!("{:?}",get_char_space(String::from("a"),String::from("b")));
    println!("{:?}",get_char_space(String::from("1"),String::from("6")));
    println!("{}",string_type(String::from("a")));
    println!("{}",string_type(String::from("@")));
    println!("{}",char_type(String::from("a")));
    println!("{}",char_type(String::from("@")));
    println!("{}",char_type(String::from("1")));
    println!("{:?}",password_strength(String::from("beluga1234__@")));
    println!("{:?}",is_secure(String::from("beluga1234__@")));
}
