// fn main()
// {
//     let mut s = String::from("Muhammad");
//     concatenate(&mut s);
// }

// fn concatenate(x: &mut String)
// {
//     x.push_str(" Zubair Yousuf");
//     println!(" {}", x );
// }

// fn main()
// {
//     let answer = square(44);
//     println!(" The answer is: {}", answer );
// }

// fn square(num:i32) -> i32
// {
//     num * num
// }

// fn main()
// {
//     let answer = my_favorite_color();
//     println!(" My favorite color is: {}", answer );
// }

// fn my_favorite_color() -> String
// {
//     String::from("Black & Red")
// }

// fn main()
// {
//     let mut name = String::from("Zubair");
//     let s1 = &mut name;
//     println!(" name: {} \n s1: {}", name,s1);
// }

// fn main()
// {
//     let book_details = function_for_book(String::from("Let us C"), String::from("Yashwant Kartnikar"));
//     println!(" {:#?}", book_details );
// }

// #[derive(Debug)]
// struct Book
// {
//     title: String,
//     author: String,
//     price: i32,
//     availability: bool,
// }

// fn function_for_book(title:String, author:String) -> Book
// {
//     Book
//     {
//         title,
//         author,
//         price: 7899,
//         availability: false,
//     }
// }

// fn main()
// {
//     let result = my_favorite_color();
//     println!(" My Favorite Color is: {}", result );
// }

// fn my_favorite_color() -> String
// {
//     String::from("Black & Red")
// }

// struct Square
// {
//     number: i32,
// }

// impl Square
// {
//     fn calculate_square(&self) -> i32
//     {
//         self.number * self.number
//     }
// }

// fn main()
// {
//     let squar = Square{number: 5 };
//     let result = squar.calculate_square();
//     println!(" The Self Sqaure: {}", result );
// }

// use std::io;
// fn main()
// {
//     let mut input_1 = String::new();
//     println!(" Hey mate! Please Enter first number.");
//     io::stdin().read_line(&mut input_1).expect("Failed to read the line");

//     let mut input_2 = String::new();
//     println!(" Hey mate! Please Enter second number");
//     io::stdin().read_line(&mut input_2).expect("Failed to read the line");

//     let aint:i32 = input_1.trim().parse().expect(" Only numbers are allowed");
//     let bint: i32 = input_2.trim().parse().expect(" Only numbers are allowed");

//     println!(" Sum of your given numbers is: {}", aint + bint );
//     println!(" Difference of your given numbers is: {}", aint - bint );
//     println!(" Multiplication of your given numbers is: {}", aint * bint );
//     println!(" Division of your given numbers is: {}", aint / bint );

// }

// use std::io;
// fn main()
// {
//     let mut input_1 = String::new();
//     println!(" Please enter first number" );
//     io::stdin().read_line(&mut input_1).expect(" Failed to read the line");

//     println!(" Please enter second number" );
//     let mut input_2 = String::new();
//     io::stdin().read_line(&mut input_2).expect(" Failed to read the line");

//     let conv_input_1:i32 = input_1.trim().parse().expect(" Only numbers are allowed");
//     let conv_input_2:i32 = input_2.trim().parse().expect(" Only numbers are allowed");

//     let result = conv_input_1 * conv_input_2;
//     println!(" Multiplication result of your given two digits is: {}", result);
// }

// fn main()
// {
//     let home_ip = IPAdress_kind::V4(String::from("192.168.0.1"));
//     let office_ip = IPAdress_kind::V6(String::from("202.89.01.9"));

//     println!(" {:#?}", home_ip );
//     println!(" {:#?}", office_ip );

// }

// #[derive(Debug)]
// enum IPAdress_kind
// {
//     V4 (String),
//     V6 (String),
// }

// fn main()
// {
//     let msg = Message::Write(String::from("Hey! Mate say something"));
//     let msg1 = Message::ChangeColor(10,20,39);
//     let msg2 = Message::Quit;

//     msg.call();
//     msg1.call();
//     msg2.call();

// }

// #[derive(Debug)]
// enum Message
// {
//     Quit,
//     Write (String),
//     Move {x: i32, y: i32},
//     ChangeColor (u8,u8,u8),
// }

// impl Message
// {
//     fn call(&self)
//     {
//         println!(" {:?}", self );
//     }
// }

// fn main()
// {
//     let some_number = Option::Some(2050510);
//     println!(" {:?}", some_number );

//     let some_text = Option::Some(String::from(" Hey! Mate say something...!"));
//     println!(" {:?}", some_text );
// }

// #[derive(Debug)]
// enum Option <T>
// {
//     Some (T),
//     None,
// }

// fn main()
// {
//     let some_integer: Option<i32> = Some(555511);
//     let some_string: Option<String> = Some(String::from("Hey! Mate say something"));

//     println!(" {:?}", some_integer );
//     println!(" {:?}", some_string );

//     let some_none_value: Option<i32> = None;
    
//     println!(" {:?}", some_none_value );
// }

// fn main()
// {
//     let row = vec![SpeardSheetCell::Int(45), SpeardSheetCell::Float(12.55), SpeardSheetCell::text(String::from("Hey! Mate say something"))];

//     for i in &row
//     {
//         println!(" {:?}", i)
//     }
// }

// #[derive(Debug)]
// enum SpeardSheetCell
// {
//     Int (i32),
//     Float (f64),
//     text (String),
// }

// fn main()
// {
//     let data = "Hey! mate say someting".to_string();
//     println!(" {}", data );
// }

// fn main()
// {
//     let mut s = String::from("Muhammad");
//     let s1 = String::from(" Zubair");

//     s.push_str(&s1);
//     println!(" {}", s);
// }

// fn main()
// {
//     let s1 = String::from("Tic");
//     let s2 = String::from("Tac");
//     let s3 = String::from("Toe");

//     let concatenate = s1 "-" + &s2 + &s3;

//     println!(" {}", concatenate )
// }

// fn main()
// {
//     let s1 = String::from("Muhammad");
//     let s2 = String::from("Zubair");
//     let s3 = String::from("Yousuf");

//     let concate = format!(" {} {} {}", s1, s2, s3 );
//     println!(" {}", concate );
// }

// fn main()
// {
//     let index = "Zubair";
//     let ext = &index[0..4];
//     println!(" {}", ext );
// }

// fn main()
// {
//     for i in "Muhammad Zubair Yousuf".bytes()
//     {
//         println!(" {}", i );
//     }
// }

// use std::collections::HashMap;
// fn main()
// {
//     let mut map = HashMap::new();
    
//     map.insert(String::from("Kapray"), 021);
//     map.insert(String::from("Zellbury"), 042);

//     println!(" {:#?}", map );
// }

// use std::io;
// fn main()
// {
//     let mut number = String::new();

//     println!(" Please Enter the number" );

//     io::stdin().read_line(&mut number).expect("Failed to read the line");
//     let conv_num: i32 = number.trim().parse().expect("Only number are allowed");

//     if conv_num > 0
//     {
//         println!(" The number is positive" );
//     } else if conv_num < 0
//     {
//         println!(" The number is negative" );
//     }
//     else {
//         println!(" 0 is neither negative nor positive" );
//     }
// }


mod table;
fn main()
{
    table::print_table();
}





























