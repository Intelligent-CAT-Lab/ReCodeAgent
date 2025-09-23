use Phills_DHT::dht::dht::*;
mod test {
    use super::*;
    #[test]
    fn uninitialise_test() {
        let dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let b = dht.dht_is_initialised();
        assert!(!b);
    }
    #[test]
    fn initialised_test() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        dht.dht_init_table(0, 10, false);
        let b = dht.dht_is_initialised();
        assert!(b);
    }
    #[test]
    fn get_size_test_1() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 10, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let size = dht.dht_get_size();
        assert_eq!(size, 10);
    }
    #[test]
    fn get_size_test_2() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 254, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let size = dht.dht_get_size();
        assert_eq!(size, 254);
    }
    #[test]
    fn get_size_test_3() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(1, 5, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let size = dht.dht_get_size();
        assert_eq!(size, 4);
    }
    #[test]
    fn get_size_test_4() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(6, 11, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let size = dht.dht_get_size();
        assert_eq!(size, 5);
    }
    #[test]
    fn get_size_test_5() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(1065, 1109, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let size = dht.dht_get_size();
        assert_eq!(size, 44);
    }
    #[test]
    fn check_bound_test_1() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(1065, 1109, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        assert_eq!(dht.dht_get_lower_bound(), 1065);
    }
    #[test]
    fn check_bound_test_2() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(62, 1109, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        assert_eq!(dht.dht_get_lower_bound(), 62);
    }
    #[test]
    fn check_bound_test_3() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 1109, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        assert_eq!(dht.dht_get_lower_bound(), 0);
    }
    #[test]
    fn check_bound_test_4() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(1065, 1109, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        assert_eq!(dht.dht_get_upper_bound(), 1109);
    }
    #[test]
    fn check_bound_test_5() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 2, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        assert_eq!(dht.dht_get_upper_bound(), 2);
    }
    #[test]
    fn read_write_1() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(14);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_2() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(0);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_3() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(1);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_4() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(2);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_5() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(1, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(3);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_6() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(7);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_7() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(16);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_8() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(17);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_9() {
        let mut dht: HASHTABLE<i32> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(18);
        assert!(j.is_none());
    }
    #[test]
    fn read_write_10() {
        let i = 65;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(19, Box::new(i));
        let j = dht.dht_read(19);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn read_write_11() {
        let i = 72;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(19, Box::new(i));
        let j = dht.dht_read(19);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn read_write_12() {
        let i = 65;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(19, Box::new(i));
        let j = dht.dht_read(19);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn write_remap_read_1() {
        let i = 65;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(13, Box::new(i));
        let ok = dht.dht_init_table(9, 14, true);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(13);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn write_remap_read_2() {
        let i = 101;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();;
        let ok = dht.dht_init_table(1, 6, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(5, Box::new(i));
        let ok = dht.dht_init_table(2, 14, true);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(5);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn write_remap_read_3() {
        let i = 1;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();;
        let ok = dht.dht_init_table(101, 1600, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(171, Box::new(i));
        let ok = dht.dht_init_table(100, 172, true);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(171);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn write_remap_read_4() {
        let i = 65;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();;
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(0, Box::new(i));
        let ok = dht.dht_init_table(0, 4, true);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(0);
        assert_eq!(*(j.clone().unwrap()), i);
    }
    #[test]
    fn write_remap_read_5() {
        let i = 65;
        let mut dht: HASHTABLE<Box<i32>> = HASHTABLE::dht_init();;
        let ok = dht.dht_init_table(0, 20, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        dht.dht_write(0, Box::new(i));
        let ok = dht.dht_init_table(0, 4, false);
        assert!(ok, "> malloc fail causes fail not logic fail.");
        let j = dht.dht_read(0);
        assert!(j.is_none());
    }
}
fn main() {}