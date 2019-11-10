use std::io;
pub fn print_table()
{
    let mut table_value = String::new();
    println!("\n*Enter Number which table you want to print*" );

    io::stdin().read_line(&mut table_value).expect("Failed to read the line");
    let table_value:i32 = table_value.trim().parse().expect("Only numbers are allowed");

    println!("You requested to print ==> {} table", table_value);
    
    println!("*Enter range how many times you want print table*" );


    let mut range = String::new();
    io::stdin().read_line(&mut range).expect("Failed to read the line");
    let range:i32 = range.trim().parse().expect("Only numbers are allowed");

    let mut i = 1;

    while i <= range

    {
        println!(" {} X {} = {}", table_value,i,(table_value * i) );
        i += 1;
    }
}