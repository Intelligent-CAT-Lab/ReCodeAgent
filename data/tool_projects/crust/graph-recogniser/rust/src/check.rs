#[cfg(debug_assertions)]
macro_rules! check {
    ($x:expr) => {
        assert!($x);
    };
}
#[cfg(not(debug_assertions))]
macro_rules! check {
    ($x:expr) => {
        (); // No-op in release mode
    };
}
#[cfg(debug_assertions)]
macro_rules! debugin {
    ($stm:stmt) => {
        $stm
    };
}
#[cfg(not(debug_assertions))]
macro_rules! debugin {
    ($stm:stmt) => {
        (); // No-op in release mode
    };
}