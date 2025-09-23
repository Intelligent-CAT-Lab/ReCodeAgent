use fslib::symt::SymTable;
#[test]
fn test_create() {
    let st: SymTable = SymTable::new();
    assert!(st.sym.len() == 0);
    for i in 0..st.n_max{
        assert!(st.get(i as i32) == None);
    }
}
#[test]
fn test_add(){
    let mut st: SymTable = SymTable::new();
    st.add(1, "Mary");
    st.add(2, "had");
    st.add(3, "a");
    st.add(4, "little");
    st.add(5, "lamb");
    st.print();
    assert!(st.get(1) == Some("Mary"));
    assert!(st.get(2) == Some("had"));
    assert!(st.get(3) == Some("a"));
    assert!(st.get(4) == Some("little"));
    assert!(st.get(5) == Some("lamb"));
    assert!(st.n_items == 5);
    assert!(st.n_max >= 5);
    st.remove();
}
#[test]
fn test_add_sparse(){
    let mut token: &str = "token";
    let mut st: SymTable = SymTable::new();
    st.add(100, token);
    assert!(st.sym.get(0) == None);
    assert!(st.get(100) == Some("token"));
}
#[test]
fn test_reverse(){
    let mut st: SymTable = SymTable::new();
    let k0 = "000";
    let k1 = "111";
    st.add(0, k0);
    st.add(1, k1);
    st.print();
    let rev: &std::collections::HashMap<String, usize> = st.reverse();
    assert!(rev.get(k0) != None);
    assert!(rev.get(k1) != None);
    assert!(rev.get(k0) == Some(&0));
    assert!(rev.get(k1) == Some(&1));
    st.remove();
}
#[test]
fn test_getr(){
    let mut st: SymTable = SymTable::new();
    let k0 = "000";
    let k1 = "111";
    let k2 = "222";
    st.add(0, k0);
    st.add(1, k1);
    st.print();
    st.reverse();
    assert!(st.getr(k0).unwrap() == 0);
    assert!(st.getr(k1).unwrap() == 1);
    assert!(st.getr(k2).unwrap() == -1);
    st.remove();
}
// #[test]
// fn test_read(){
//     let mut st: SymTable = SymTable::new();
//     st.read("tests/test_symt.txt");
//     assert!(st.n_items == 2);
//     asssert!(st.get(1)==Some("one"));
//     assert!(st.get(2)==Some("two"));
//     st.remove();
// }
fn main(){}