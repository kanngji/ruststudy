use std::mem::size_of;
// 함수!
fn give_age() -> i32 {
    42
}

fn main() {
    //let my_number= u8 = 9;
    let my_number2= 9_u8;
    println!("{}",my_number2);

    let num1 = 9.4;
    let num2 = 9;
    println!("{}", num1 as i32+num2);


    // macro = function that writes code 
    // 실행 cargo run

    let my_name ="kang";
    println!("MY name is {},{}",my_name,give_age());
}
