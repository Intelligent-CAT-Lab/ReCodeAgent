use std::cmp::Ordering;
use std::fmt;
#[derive(Clone, Debug)]
pub struct BigInt {
    negative: bool,
    digits: Vec<u8>,
}
impl BigInt {
    pub fn zero() -> Self {
        unimplemented!()
    }
    pub fn is_64_bit(&self) -> bool {
        unimplemented!()
    }
    pub fn remove_leading_zeros(&mut self) {
        unimplemented!()
    }
    pub fn from_int(n: i64) -> Self {
        unimplemented!()
    }
    pub fn to_int(&self) -> i64 {
        unimplemented!()
    }
    pub fn from_str(s: &str) -> Self {
        unimplemented!()
    }
    pub fn copy(&self) -> BigInt {
        unimplemented!()
    }
    pub fn print(&self) {
        unimplemented!()
    }
    pub fn is_zero(&self) -> bool {
        unimplemented!()
    }
    pub fn lt_zero(&self) -> bool {
        unimplemented!()
    }
    pub fn gt_zero(&self) -> bool {
        unimplemented!()
    }
    pub fn lezero(&self) -> bool {
        unimplemented!()
    }
    pub fn gezero(&self) -> bool {
        unimplemented!()
    }
    pub fn abs(&self) -> Self {
        unimplemented!()
    }
    pub fn add(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn sub(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn inc(&mut self) {
        unimplemented!()
    }
    pub fn dec(&mut self) {
        unimplemented!()
    }
    pub fn mul(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn divmod(&self, other: &Self) -> (Self, Self) {
        unimplemented!()
    }
    pub fn div(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn r#mod(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn pow(&self, exponent: &Self) -> Self {
        unimplemented!()
    }
    pub fn fast_pow(&self, exponent: &Self, modulo: &Self) -> Self {
        unimplemented!()
    }
    pub fn modinv(&self, modulo: &Self) -> Self {
        unimplemented!()
    }
    pub fn sqrt(&self) -> Self {
        unimplemented!()
    }
    pub fn is_even(&self) -> bool {
        unimplemented!()
    }
    pub fn is_odd(&self) -> bool {
        unimplemented!()
    }
    pub fn is_prime(&self) -> bool {
        unimplemented!()
    }
}
pub fn gt(a: &BigInt, b: &BigInt) -> bool {
    unimplemented!()
}
pub fn lt(a: &BigInt, b: &BigInt) -> bool {
    unimplemented!()
}
pub fn ge(a: &BigInt, b: &BigInt) -> bool {
    unimplemented!()
}
pub fn le(a: &BigInt, b: &BigInt) -> bool {
    unimplemented!()
}
pub fn delete(a: &mut BigInt) {
    unimplemented!()
}
impl std::ops::Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> BigInt {
        unimplemented!()
    }
}
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}
impl Eq for BigInt {}
impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}