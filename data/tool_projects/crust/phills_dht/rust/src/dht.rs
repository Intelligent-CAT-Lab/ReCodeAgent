pub mod dht {
    pub struct HASHTABLE<T> {
        pub hash_table: Vec<Option<T>>, 
        pub lower_bound: i32, 
        pub higher_bound: i32,
    }
    impl <T> HASHTABLE<T> {
        pub fn dht_init() -> HASHTABLE<T> {
            unimplemented!()
        }
        pub fn dht_is_initialised(&self) -> bool {
            unimplemented!()
        }
        pub fn dht_init_table(&mut self, new_lower_bound: u32, new_upper_bound: u32, migrate: bool) -> bool {
            unimplemented!()
        }
        pub fn dht_get_lower_bound(&self) -> u32 {
            unimplemented!()
        }
        pub fn dht_get_upper_bound(&self) -> u32 {
            unimplemented!()
        }
        pub fn dht_get_size(&self) -> u32 {
            unimplemented!()
        }
        pub fn dht_read(&self, location: u32) -> &Option<T> {
            unimplemented!()
        }
        pub fn dht_write(&mut self, location: u32, data: T) {
            unimplemented!()
        }
    }
}