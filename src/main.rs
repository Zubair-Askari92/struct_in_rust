fn main()
{
    let mut s = String::from("Muhammad");
    concatenate(&mut s);
}

fn concatenate(x: &mut String)
{
    x.push_str(" Zubair Yousuf");
    println!(" {}", x );
}

fn main()
{
    let answer = square(44);
    println!(" The answer is: {}", answer );
}

fn square(num:i32) -> i32
{
    num * num
}

fn main()
{
    let answer = my_favorite_color();
    println!(" My favorite color is: {}", answer );
}

fn my_favorite_color() -> String
{
    String::from("Black & Red")
}

fn main()
{
    let mut name = String::from("Zubair");
    let s1 = &mut name;
    println!(" name: {} \n s1: {}", name,s1);
}

fn main()
{
    let book_details = build(String::from("Who is who & What is what"), String::from("Yashwant Kartnikar"));
    println!(" {:#?}", book_details );
}

#[derive(Debug)]
struct Book 
{
    name: String,
    author: String,
    price: i32,
    availability: bool,
}

fn build(name:String, author:String) -> Book
{
    Book
    {
        name: name,
        author: author,
        price: 4500,
        availability: true,
    }
}












