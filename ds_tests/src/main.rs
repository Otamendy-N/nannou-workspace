use basic_data_structures::{hash_table::HashTable};

fn main() {
    let mut table = HashTable::new_with_custom_size(20);

    for i in 0..200 {
        table.insert(i.to_string());
    }

    table.print();
}
