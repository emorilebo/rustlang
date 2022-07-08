
#[derive(Debug)]
enum MyError{
    Error1
}

//Err, an enum that contains an error code
//Ok(value), a wrapper that contains a value
fn divide(dividend: i32, divisor: i32)-> Result<i32, MyError>{
    if dividend % divisor != 0{
        Err(MyError::Error1)
    }else{
        Ok(dividend/divisor)
    }
}
fn main() {
    let divide = divide(4, 3);
    let res = divide.expect("we crashed");

    // match divide{
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v)
    // }
    // if divide.is_ok(){
    //     println!("{}", divide.unwrap());
    // }
    //println!("{}", divide.unwrap());
    //println!("{}", divide.unwrap_or(100));
    println!("{}", res)
}
