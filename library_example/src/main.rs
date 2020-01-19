mod lib;
use std::io;

fn main() {
    loop
    {
    println!("Please enter table number");
    let mut table_number=String::new();
    io::stdin().read_line(&mut table_number)
    .expect("Failed to read line");
    let table_number:u32=match table_number.trim().parse()
    {
        Ok(num)=>num,
        Err(_)=>continue,
    };

    println!("you table choice is :{}",table_number);

    lib::parent_module::child_module::table(table_number);
    break;

}
}