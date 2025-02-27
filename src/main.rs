use std::sync::Mutex;

use lazy_static::lazy_static;

const ENTRIES_PER_TABLE: usize = 256;

lazy_static! {
    static ref PHYSICAL_MEMORY: Mutex<PhysicalMemory> = {
        let pm = Mutex::new(PhysicalMemory::new(1024));
        pm
    };
}
#[derive(Clone, Debug)]
enum DataTypes {
    StringType(String),
    FloatType(f64),
    IntegerType(usize),
}

#[derive(Clone)]
struct PhysicalMemory {
    frames: Vec<Option<DataTypes>>,
}

impl PhysicalMemory {
    fn new(size: usize) -> Self {
        PhysicalMemory {
            frames: vec![None; size],
        }
    }
}
macro_rules! generate_table {
    ($name:ident, $entry_type:ty) => {
        #[derive(Clone, Debug)]
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
generate_table!(Table4, Option<Box<Table3>>);
generate_table!(Table3, Option<Box<Table2>>);
generate_table!(Table2, Option<Box<Table1>>);
generate_table!(Table1, Option<usize>);

#[derive(Debug)]
struct PageTableHierarchy {
    table4: Table4,
}
impl PageTableHierarchy {
    fn new() -> Self {
        PageTableHierarchy {
            table4: Table4::default(),
        }
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
    fn extract_indices_from_virtual_address(
        &self,
        virtual_address: usize,
    ) -> (usize, usize, usize, usize, usize) {
        let table4_idx = virtual_address >> 39 & 0xFF;
        let table3_idx = virtual_address >> 30 & 0xFF;
        let table2_idx = virtual_address >> 21 & 0xFF;
        let table1_idx = virtual_address >> 12 & 0xFF;
        let offset = virtual_address & 0xFFF;

        (table4_idx, table3_idx, table2_idx, table1_idx, offset)
    }

    fn create_mapping(&mut self, virtual_address: usize) -> Result<String, String> {
        let (table4_idx, table3_idx, table2_idx, table1_idx, offset) =
            self.extract_indices_from_virtual_address(virtual_address);
        if self.table4.entries[table4_idx].is_none() {
            self.table4.entries[table4_idx] = Some(Box::new(Table3::default()));
        }

        let ref mut table3 = self.table4.entries[table4_idx];

        match table3 {
            Some(ref mut table3) => {
                // return Err( format!("ENTRY_USED: Failed to create Table 3 at entry:  {:#?}." ,table3_idx));
                let entry = &mut table3.as_mut().entries[table3_idx];
                if entry.is_none() {
                    *entry = Some(Box::new(Table2::default()));
                }
            }
            None => *table3 = Some(Box::new(Table3::default())),
        }

        /*
        Table 3 was created above so its safe to call unwrap()
        */
        let ref mut table2 = table3.as_mut().unwrap().entries[table3_idx];

        match table2 {
            Some(table2) => {
                let entry = &mut table2.as_mut().entries[table2_idx];
                if entry.is_none() {
                    *entry = Some(Box::new(Table1::default()));
                }
            }
            None => *table2 = Some(Box::new(Table2::default())),
        }

        let ref mut table1 = table2.as_mut().unwrap().entries[table2_idx];
        match table1 {
            Some(table1) => {
                let entry = &mut table1.as_mut().entries[table1_idx];
                if entry.is_none() {
                    *entry = Some(offset);

                    return Ok(format!("VIRTUAL_ADDRESS_MAPPED: Virtual address {:#X} mapped to Physical Frame successfully.",virtual_address));
                }
                if entry.is_some() {
                    return Err( format!("ENTRY_USED: Failed to Map Virtual Address: {:#X} to Physical Frame {:#X} through Table 1 at index:  {:#?}" ,virtual_address,offset,table1_idx));
                } else {
                    return Err( format!("PHYSICAL_MEMORY NOT INITALIZED: Failed to Map virtual address to physical memory"));
                }
            }

            None => {
                *table1 = Some(Box::new(Table1::default()));
                table1.as_mut().unwrap().entries[table1_idx] = Some(offset);

                return Ok(format!("VIRTUAL_ADDRESS_MAPPED: Virtual address {:#?} mapped to Physical Frame successfully.",virtual_address));
            }
        }
    }
    fn traverse_hierarchy(&mut self, virtual_address: usize) -> usize {
        let (table4_idx, table3_idx, table2_idx, table1_idx, _) =
            self.extract_indices_from_virtual_address(virtual_address);
        let table3 = self.table4.entries[table4_idx].as_mut().unwrap();
        let table2 = table3.entries[table3_idx].as_mut().unwrap();
        let table1 = table2.entries[table2_idx].as_mut().unwrap();
        let frame = table1.entries[table1_idx].unwrap();

        frame
    }

    fn write_to_memory(
        &mut self,
        virtual_address: usize,
        value: DataTypes,
    ) -> Result<String, String> {
        let ref mut physical_mem = PHYSICAL_MEMORY.lock().unwrap().frames;
        let frame = self.traverse_hierarchy(virtual_address);
        physical_mem[frame] = Some(value);
        return Ok("Successfully written to physical memory".to_string());
    }

  fn read_from_memory(&mut self, virtual_address: usize) -> Result<DataTypes, String> {
    let physical_mem = PHYSICAL_MEMORY.lock().map_err(|_| "Failed to lock physical memory")?;
    let frame = self.traverse_hierarchy(virtual_address);
    
    match physical_mem.frames[frame].as_ref() {
        Some(data) => Ok(data.clone()),
        None => Err("No Data Stored In Memory.".to_string()),
        // None => Err("Invalid Memory Address".to_string()),
    }
}
fn dump_memory(self)->Result<Vec<Option<DataTypes>>,String>{
    let physical_mem = PHYSICAL_MEMORY.lock().map_err(|_|"Failed to lock physical memory ")?;

     let mem = &physical_mem.frames;
     return Ok(mem.to_vec());
}
}
fn main() {
    let mut page_table_hierarchy = PageTableHierarchy::new();

    println!("PML4 PAGE TABLE:{:#?}",page_table_hierarchy);

    let virtual_address = page_table_hierarchy.generate_virtual_address();
    let map_result = page_table_hierarchy.create_mapping(virtual_address);
    match map_result {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{:#?}", err),
    }

    let write_res = page_table_hierarchy
        .write_to_memory(virtual_address, DataTypes::StringType("hello".to_owned()));
    match write_res {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{:#?}", err),
    }
    let read_res = page_table_hierarchy.read_from_memory(virtual_address);
    match read_res{
        Ok(data)=> println!("{:#?}",data),
        Err(err)=>println!("{:#?}",err)
    }
    let memory = page_table_hierarchy.dump_memory();
    match memory{
        Ok(data)=>{
            for (index,item) in data.iter().enumerate(){
                //VISUALIZE FULL MEMORY
                //  println!("Memory data at address: {:#X} : {:#?}",index,item);
            }

        },
        Err(err)=>println!("{:#?}",err)
    }
    }
