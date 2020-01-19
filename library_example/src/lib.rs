
pub mod parent_module
{

 pub mod child_module
    {   
        pub fn table(num:u32)
        {
            println!("Here is your desired table, Note: We are in  lib.rs table function");
            for table_from_to in 1..=10
            {
                
                println!("{} * {} = {}", num,table_from_to, num * table_from_to);
            }
        }
    }
}