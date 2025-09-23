pub struct HashTableEntry<'a, T> {
    pub key: u64,
    pub val: &'a mut [T],
}
pub struct HashTable<'a, T> {
    pub size: u64,
    pub max_size: u64,
    pub growth_factor: f32,
    pub capacity: u64,
    pub data: &'a mut [HashTableEntry<'a, T>],
    pub last_found_idx: u64,
}
impl <T> HashTable<'_, T> {
pub fn new(log_init_capacity: i32) -> Self {
    unimplemented!()
}
pub fn realloc_table(&mut self) -> bool {
    unimplemented!()
}
pub fn hash_table_find(&self, key: u64) -> Option<&Box<dyn std::any::Any>> {
    unimplemented!()
}
pub fn hash_table_delete(&mut self, key: u64) -> bool {
    unimplemented!()
}
pub fn compute_idx(&self, key: u64) -> u64 {
    unimplemented!()
}
pub fn hash_table_insert(&mut self, key: u64, val: Box<dyn std::any::Any>) -> bool {
    unimplemented!()
}
pub fn compute_hash(key: u64) -> u64 {
    unimplemented!()
}
pub fn cell_empty(entry: &HashTableEntry<T>) -> bool {
    unimplemented!()
}
pub fn hash_table_free(&mut self) {
    unimplemented!()
}
pub fn handle_gap(&mut self, idx_of_gap: u64) -> bool {
    unimplemented!()
}
pub fn find_entry(&self, key: u64, idx: &mut u64) -> bool {
    unimplemented!()
}
pub fn hash_table_delete_last_found(&mut self) -> bool {
    unimplemented!()
}
}