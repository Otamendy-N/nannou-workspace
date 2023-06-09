use std::f32::consts::PI;

use crate::{linked_list::LinkedList, queue::Queue};

pub struct HashTable {
    size: usize,
    rows_count: usize,
    rows: LinkedList<LinkedList<String>>
}

impl HashTable {
    pub fn new() -> Self {
        let mut rows = LinkedList::new();
        let rows_count = 10;
        for _ in 0..rows_count {
            rows.append(LinkedList::new());
        }
        HashTable { size: 0, rows: rows, rows_count }
    }
    pub fn new_with_custom_size(rows_count: u32) -> Self {
        let mut rows = LinkedList::new();
        for _ in 0..rows_count {
            rows.append(LinkedList::new());
        }
        HashTable { size: 0, rows: rows, rows_count: rows_count as usize }
    }

    fn hash(&self, elem: String) -> usize {
        let sum: u32 = elem.bytes().into_iter().map(|i|{ i as u32}).sum();
        (sum % self.rows_count as u32) as usize
    }

    fn hash_with_string_folding(&self, elem: String) -> usize {
        let sum: u32 = elem
            .bytes()
            .into_iter()
            .map(|i|{ 
                let mult = if i % 4 == 0 { 1 } else { 255 };
                i as u32 * mult
            })
            .sum();
        (sum % self.rows_count as u32) as usize
    }

    pub fn insert(&mut self, elem: String) {
        let hash = if self.rows_count > 30 {
            self.hash_with_string_folding(elem.clone())
        } else {
            self.hash(elem.clone())
        };
        self.rows.for_each_mut(|list, i: usize| {
            if i != hash { return; }
            list.append(elem.clone());
        });
        self.size += 1;
    }

    pub fn print(& self) {
        println!("\nHashTable:");
        self.rows.for_each(|list, i: usize| {
            print!("{}-| ", i);
            list.print();
        });
    }
}