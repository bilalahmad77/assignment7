pub mod parent_module
{
    pub mod child_module
    {
    pub fn table(num:u32)
        {
            println!("Printing your desired table, Note: We are in library package and table function");
            for table_from_to in 1..=10
            {
                
                println!("{} * {} = {}", num,table_from_to, num * table_from_to );
            }
        }
    }
}