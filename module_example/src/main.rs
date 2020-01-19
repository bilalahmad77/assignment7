mod parent_mod
{
pub mod child_mod
{
pub fn table(table_no:u32)
{
    println!("Your table of choice is given below:");
    for table_from_to in 1..=10
    {
        
        println!("{} * {} = {}", table_no,table_from_to, table_no * table_from_to);
    }
}
}
}
use std::io;

fn main() {
    loop
    {
    println!("Please enter table number");
    let mut table_no=String::new();
    io::stdin().read_line(&mut table_no)
    .expect("Failed to read line");
    let table_no:u32=match table_no.trim().parse()
    {
        Ok(table_no)=>table_no,
        Err(_)=>continue,
    };

    println!("your table of choice is :{}",table_no);

    parent_mod::child_mod::table(table_no); // relative path
    // OR
   // crate::parent_mod::child_mod::table(table_no); //absolute path

    break;

}
}