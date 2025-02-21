const ENTRIES_PER_TABLE: usize = 256;

struct Table1 {
    entries: [Option<usize>; ENTRIES_PER_TABLE], // Physical page numbers
}

impl Default for Table1 {
    fn default() -> Self {
        Table1 {
            entries: [None; ENTRIES_PER_TABLE],
        }
    }
}

struct Table2 {
    entries: [Option<Box<Table1>>; ENTRIES_PER_TABLE],
}

impl Default for Table2 {
    fn default() -> Self {
        Table2 {
            entries: std::array::from_fn(|_| None),
        }
    }
}

struct Table3 {
    entries: [Option<Box<Table2>>; ENTRIES_PER_TABLE],
}

impl Default for Table3 {
    fn default() -> Self {
        Table3 {
            entries: std::array::from_fn(|_| None),
        }
    }
}

struct Table4 {
    entries: [Option<Box<Table3>>; ENTRIES_PER_TABLE],
}

impl Default for Table4 {
    fn default() -> Self {
        Table4 {
            entries: std::array::from_fn(|_| None),
        }
    }
}

struct PageTableHierarchy {
    table4: Table4,
}

impl PageTableHierarchy {
    fn new() -> Self {
        PageTableHierarchy {
            table4: Table4::default(),
        }
    }
    
    fn map(&mut self, i3: usize, i2: usize, i1: usize, i0: usize, phys_page: usize) {
        assert!(i3 < ENTRIES_PER_TABLE, "i3 out of bounds");
        assert!(i2 < ENTRIES_PER_TABLE, "i2 out of bounds");
        assert!(i1 < ENTRIES_PER_TABLE, "i1 out of bounds");
        assert!(i0 < ENTRIES_PER_TABLE, "i0 out of bounds");

        if self.table4.entries[i3].is_none() {
            self.table4.entries[i3] = Some(Box::new(Table3::default()));
        }
        let table3 = self.table4.entries[i3].as_mut().unwrap();

        if table3.entries[i2].is_none() {
            table3.entries[i2] = Some(Box::new(Table2::default()));
        }
        let table2 = table3.entries[i2].as_mut().unwrap();

        if table2.entries[i1].is_none() {
            table2.entries[i1] = Some(Box::new(Table1::default()));
        }
        let table1 = table2.entries[i1].as_mut().unwrap();

        table1.entries[i0] = Some(phys_page);
    }
    
    fn get(&self, i3: usize, i2: usize, i1: usize, i0: usize) -> Option<usize> {
        if let Some(table3) = &self.table4.entries[i3] {
            if let Some(table2) = &table3.entries[i2] {
                if let Some(table1) = &table2.entries[i1] {
                    return table1.entries[i0];
                }
            }
        }
        None
    }
    
    fn print_path(&self, i3: usize, i2: usize, i1: usize, i0: usize) {
        println!("Accessing virtual address ({}, {}, {}, {}):", i3, i2, i1, i0);
        println!(
            "  Table4[{}] = {}",
            i3,
            if self.table4.entries[i3].is_some() {
                "Some(Table3)"
            } else {
                "None"
            }
        );

        if let Some(table3) = &self.table4.entries[i3] {
            println!(
                "    Table3[{}] = {}",
                i2,
                if table3.entries[i2].is_some() {
                    "Some(Table2)"
                } else {
                    "None"
                }
            );

            if let Some(table2) = &table3.entries[i2] {
                println!(
                    "      Table2 [{}] = {}",
                    i1,
                    if table2.entries[i1].is_some() {
                        "Some(Table1)"
                    } else {
                        "None"
                    }
                );

                if let Some(table1) = &table2.entries[i1] {
                    if let Some(phys_page) = table1.entries[i0] {
                        println!("        Table1[{}] = Some(physical page {})", i0, phys_page);
                    } else {
                        println!("        Table1[{}] = None", i0);
                    }
                }
            }
        }
    }
     /// Checks if Table1 has any physical page mappings.
    fn has_mappings_table1(&self, table1: &Table1) -> bool {
        table1.entries.iter().any(|e| e.is_some())
    }

    /// Checks if Table2 has any mappings through its Table1 entries.
    fn has_mappings_table2(&self, table2: &Table2) -> bool {
        table2.entries.iter().any(|e| e.as_ref().map_or(false, |t1| self.has_mappings_table1(t1)))
    }

    /// Checks if Table3 has any mappings through its Table2 entries.
    fn has_mappings_table3(&self, table3: &Table3) -> bool {
        table3.entries.iter().any(|e| e.as_ref().map_or(false, |t2| self.has_mappings_table2(t2)))
    }

    /// Checks if Table4 has any mappings through its Table3 entries.
    fn has_mappings_table4(&self, table4: &Table4) -> bool {
        table4.entries.iter().any(|e| e.as_ref().map_or(false, |t3| self.has_mappings_table3(t3)))
    
}
pub fn print_hierarchy(&self) {
    if !self.has_mappings_table4(&self.table4) {
        println!("No mappings present.");
    } else {
        self.print_table4(&self.table4, "");
    }
}
fn print_table4(&self, table4: &Table4, prefix: &str) {
    // Collect indices where entries are Some and have mappings
    let some_indices: Vec<_> = (0..ENTRIES_PER_TABLE)
        .filter(|&i3| table4.entries[i3].as_ref().map_or(false, |t3| self.has_mappings_table3(t3)))
        .collect();

    // Iterate over mapped indices
    for (pos, &i3) in some_indices.iter().enumerate() {
        let is_last = pos == some_indices.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}Table4[{}] -> Some(Table3)", prefix, connector, i3);
        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
        self.print_table3(table4.entries[i3].as_ref().unwrap(), &new_prefix);
    }
}
fn print_table3(&self, table3: &Table3, prefix: &str) {
    let some_indices: Vec<_> = (0..ENTRIES_PER_TABLE)
        .filter(|&i2| table3.entries[i2].as_ref().map_or(false, |t2| self.has_mappings_table2(t2)))
        .collect();

    for (pos, &i2) in some_indices.iter().enumerate() {
        let is_last = pos == some_indices.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}Table3[{}] -> Some(Table2)", prefix, connector, i2);
        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
        self.print_table2(table3.entries[i2].as_ref().unwrap(), &new_prefix);
    }
}
fn print_table2(&self, table2: &Table2, prefix: &str) {
    let some_indices: Vec<_> = (0..ENTRIES_PER_TABLE)
        .filter(|&i1| table2.entries[i1].as_ref().map_or(false, |t1| self.has_mappings_table1(t1)))
        .collect();

    for (pos, &i1) in some_indices.iter().enumerate() {
        let is_last = pos == some_indices.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}Table2[{}] -> Some(Table1)", prefix, connector, i1);
        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
        self.print_table1(table2.entries[i1].as_ref().unwrap(), &new_prefix);
    }
}
fn print_table1(&self, table1: &Table1, prefix: &str) {
    let some_indices: Vec<_> = (0..ENTRIES_PER_TABLE)
        .filter(|&i0| table1.entries[i0].is_some())
        .collect();

    for (pos, &i0) in some_indices.iter().enumerate() {
        let is_last = pos == some_indices.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        println!(
            "{}{}Table1[{}] -> Physical Page {:?}", 
            prefix, connector, i0, table1.entries[i0].unwrap()
        );
    }
}
  
}

fn main() {
    let mut pt = PageTableHierarchy::new();

     pt.map(1, 2, 3, 4, 42); // Map virtual address (1,2,3,4) to physical page 42
    pt.print_hierarchy();
    
    
}

/*

  fn print_hierarchy(&self) {
    println!("Page Table Hierarchy:");
    self.print_table4(&self.table4, 0);
}

fn print_table4(&self, table4: &Table4, indent: usize) {
    let indent_str = "  ".repeat(indent);
    for i3 in 0..ENTRIES_PER_TABLE {
        match &table4.entries[i3] {
            Some(table3) => {
                println!("{}Table4[{}] -> Some(Table3)", indent_str, i3);
                self.print_table3(table3, indent + 1);
            }
            None => {
                println!("{}Table4[{}] -> None", indent_str, i3);
            }
        }
    }
}

fn print_table3(&self, table3: &Table3, indent: usize) {
    let indent_str = "  ".repeat(indent);
    for i2 in 0..ENTRIES_PER_TABLE {
        match &table3.entries[i2] {
            Some(table2) => {
                println!("{}Table3[{}] -> Some(Table2)", indent_str, i2);
                self.print_table2(table2, indent + 1);
            }
            None => {
                println!("{}Table3[{}] -> None", indent_str, i2);
            }
        }
    }
}

fn print_table2(&self, table2: &Table2, indent: usize) {
    let indent_str = "  ".repeat(indent);
    for i1 in 0..ENTRIES_PER_TABLE {
        match &table2.entries[i1] {
            Some(table1) => {
                println!("{}Table2[{}] -> Some(Table1)", indent_str, i1);
                self.print_table1(table1, indent + 1);
            }
            None => {
                println!("{}Table2[{}] -> None", indent_str, i1);
            }
        }
    }
}

fn print_table1(&self, table1: &Table1, indent: usize) {
    let indent_str = "  ".repeat(indent);
    for i0 in 0..ENTRIES_PER_TABLE {
        match table1.entries[i0] {
            Some(phys_page) => {
                println!("{}Table1[{}] -> Some(physical page {})", indent_str, i0, phys_page);
            }
            None => {
                println!("{}Table1[{}] -> None", indent_str, i0);
            }
        }
    }
}

*/