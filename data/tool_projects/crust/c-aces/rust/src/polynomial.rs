use crate::error::Result;
pub type Coeff = i64;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub coeffs: Vec<Coeff>,
}
impl Polynomial {
    pub fn new(coeffs: Vec<Coeff>) -> Self {
        unimplemented!()
    }
    pub fn degree(&self) -> usize {
        unimplemented!()
    }
    pub fn set_zero(&mut self) {
        unimplemented!()
    }
    pub fn coef_sum(&self) -> Coeff {
        unimplemented!()
    }
    pub fn fit(&mut self, modulus: u64) -> Result<()> {
        unimplemented!()
    }
    pub fn add(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
    pub fn sub(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
    pub fn mul(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
    pub fn lshift(&self, _other: &Polynomial, _modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
    pub fn poly_mod(&mut self, _divisor: &Polynomial, _modulus: u64) -> Result<()> {
        unimplemented!()
    }
    pub fn sub_scaler(&self, scaler: u64, modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
    pub fn add_scaler(&self, scaler: u64, modulus: u64) -> Result<Polynomial> {
        unimplemented!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyArray {
    pub polies: Vec<Polynomial>,
}
impl PolyArray {
    pub fn new(polies: Vec<Polynomial>) -> Self {
        unimplemented!()
    }
}