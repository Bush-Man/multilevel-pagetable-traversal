use std::fmt;



const ENTRIES_PER_TABLE: usize = 5;

#[derive(Clone)]
enum DataTypes{
    StringType(String),
    NumberType,
    }
// struct StringType<'a>{
//     value:&'a str
// }    
enum NumberType{
     FloatType(f64),
     IntegerType(usize),
}
// struct FloatType{
//     value:f64
// }
// struct IntegerType{
//     value:usize
// }
#[derive(Clone)]
struct PhysicalMemory{
   frames:Vec<Option<DataTypes>>
}

impl PhysicalMemory{
    fn new(size:usize)->Self{
        PhysicalMemory{
            frames:vec![None;size]
        }
    }

}
macro_rules! generate_table {
    ($name:ident, $entry_type:ty) => {
        #[derive(Clone,Debug)]
        struct $name {
            entries: [$entry_type; ENTRIES_PER_TABLE],
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    entries: std::array::from_fn(|_| None),
                }
            }
        }
    };
   
}
 generate_table!(Table4,Option<Box<Table3>>);
  generate_table!(Table3,Option<Box<Table2>>);
  generate_table!(Table2,Option<Box<Table1>>);
  generate_table!(Table1,Option<usize>);

  #[derive(Debug)]
struct PageTableHierarchy{
     table4:Table4,
}
impl PageTableHierarchy{
    fn new()->Self{
        PageTableHierarchy { table4:Table4::default() }
    }

   fn generate_virtual_address(&mut self) -> usize {
    // using the remainder as the index
    // remainder is always less than the divisor (ENTRIES_PER_PAGE) and greater than zero in modulus operator for positive numbers
    // generates numbers between 0-255
    let table4_idx = rand::random::<usize>() % ENTRIES_PER_TABLE;
    let table3_idx = rand::random::<usize>() % ENTRIES_PER_TABLE;
    let table2_idx = rand::random::<usize>() % ENTRIES_PER_TABLE;
    let table1_idx = rand::random::<usize>() % ENTRIES_PER_TABLE;
    let offset = rand::random::<usize>() % 1024;
  
 
    (table4_idx << 39) | (table3_idx << 30) | (table2_idx << 21) | (table1_idx << 12) | offset
}
    fn extract_indices_from_virtual_address(&self,virtual_address:usize)->(usize,usize,usize,usize,usize){
       let table4_idx = virtual_address >> 39 & 0xFF;
       let table3_idx = virtual_address >> 30 & 0xFF;
       let table2_idx = virtual_address >> 21 & 0xFF;
       let table1_idx = virtual_address >> 12 & 0xFF;
       let offset = virtual_address & 0xFFF;

       (table4_idx,table3_idx,table2_idx,table1_idx,offset)
       
    }

    fn create_mapping(&mut self,virtual_address:usize)->Result<String,String>{
   let (table4_idx,table3_idx,table2_idx,table1_idx,offset) = self.extract_indices_from_virtual_address(virtual_address);
     if self.table4.entries[table4_idx].is_none(){
        self.table4.entries[table4_idx] = Some(Box::new(Table3::default()));
            // return Ok( format!("ENTRY_MAPPED: Entry {:#?} mapped to Table 3 Successfully.  " ,table4_idx));
     }else{
      if self.table4.entries[table4_idx].is_some(){
         return Err( format!("ENTRY_USED: Failed to create Table 3 for entry:  {:#?}." ,table4_idx));
      }
     }
    
     
     let  mut table3_entry=  self.table4.entries[table3_idx].as_mut();
      println!("{:#?}",table3_entry);
     
     match table3_entry{

            Some(ref mut table_3_entry) => {

           if let Some(entry)=table_3_entry.entries.get_mut(table3_idx){
            if entry.is_none() {
                
                *entry = Some(Box::new(Table2::default()));
                
            //  return Ok(format!("ENTRY_MAPPED: Entry {} mapped to Table 2 successfully.", table3_idx));
           }else{
            return Err( format!("ENTRY_USED: Failed to create Table 2 for entry:  {:#?}." ,table3_idx));
          }
        }else{
            return Err(format!("INDEX_OUT_OF_BOUNDS: Index {} is out of bounds for Table 3 entries.", table3_idx));
        }
        }
    
         None => {
       return Err("TABLE 3 DOES NOT EXIST: Need to create and allocate memory for it.".to_string());
        }
    
         }


         let mut table2_entry = table3_entry.unwrap().entries[table2_idx].as_mut();

         match table2_entry{
            Some(ref mut table_2_entry)=>{
                if  let Some(entry) = table_2_entry.entries.get_mut(table2_idx){
                    if entry.is_none(){
                        *entry = Some(Box::new(Table1::default()));
                    //    return Ok(format!("ENTRY_MAPPED: Entry {} mapped to Table 1 successfully.", table2_idx));
                        
                    }else{
                    return Err( format!("ENTRY_USED: Failed to create Table 1 for entry:  {:#?}" ,table2_idx));

                    }
                }else{
            return Err(format!("INDEX_OUT_OF_BOUNDS: Index {} is out of bounds for Table 2 entries.", table2_idx));

                }

            }
            None =>{
                return Err("TABLE 2 DOES NOT EXIST: Need to create and allocate memory for it ".to_string());
            }
         }

         let table1_entry = table2_entry.unwrap().entries[table1_idx].as_mut();
         match table1_entry{
            Some(table)=>{
                 if let Some (entry) = table.entries.get_mut(table1_idx){
                    if entry.is_none(){
                    
                        *entry = Some(offset);
                       return Ok(format!("VIRTUAL_ADDRESS_MAPPED: Virtual address {:#?} mapped to Physical Frame successfully.",virtual_address));
                    }else{
                        return Err( format!("ENTRY_USED: Failed to Map virtual address to frame:  {:#?}" ,table1_idx));

                      }  
                    }else{
                        return Err(format!("INDEX_OUT_OF_BOUNDS: Index {} is out of bounds for Table 2 entries.", table2_idx));
                      }
                    
                 

            }
            None =>{
                return Err("TABLE 1 DOES NOT EXIST: Need to create and allocate memory for it ".to_string());
                
            }
         }


    
}

    

}
fn main(){
   let physicalMemory = PhysicalMemory::new(1024);
   let mut pageTableHierarchy = PageTableHierarchy::new();
//    let virtual_address= pageTableHierarchy.generate_virtual_address();
//    let result = pageTableHierarchy.create_mapping(virtual_address);

//    match  result {
//         Ok(res) => println!("{:#?}",res), 
//         Err(err) => println!("{:#?}",err)    
//    }

println!("{:#?}",pageTableHierarchy);

   


   

 
}