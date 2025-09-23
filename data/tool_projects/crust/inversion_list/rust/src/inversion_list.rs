use std::fmt;
#[derive(Debug)]
pub enum InversionListError {
    ValueOutOfRange(u32, u32),
    Generic(String),
}
#[derive(Clone, Debug)]
pub struct InversionList {
    capacity: u32,
    support: u32,
    pub intervals: Vec<(u32, u32)>,
}
impl InversionList {
    pub fn new(capacity: u32, values: &[u32]) -> Result<Self, InversionListError> {
        unimplemented!()
    }
    pub fn capacity(&self) -> u32 {
        unimplemented!()
    }
    pub fn support(&self) -> u32 {
        unimplemented!()
    }
    pub fn contains(&self, value: u32) -> bool {
        unimplemented!()
    }
    pub fn clone_list(&self) -> Self {
        unimplemented!()
    }
    pub fn complement(&self) -> Self {
        unimplemented!()
    }
    pub fn to_str(&self) -> String {
        unimplemented!()
    }
    pub fn equal(&self, other: &Self) -> bool {
        unimplemented!()
    }
    pub fn is_strict_subset_of(&self, other: &Self) -> bool {
        unimplemented!()
    }
    pub fn is_subset_of(&self, other: &Self) -> bool {
        unimplemented!()
    }
    pub fn is_disjoint(&self, other: &Self) -> bool {
        unimplemented!()
    }
    pub fn union(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn intersection(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn difference(&self, other: &Self) -> Self {
        unimplemented!()
    }
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        unimplemented!()
    }
}
impl PartialEq for InversionList {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}
impl Eq for InversionList {}
impl fmt::Display for InversionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
pub struct InversionListIterator<'a> {
    list: &'a InversionList,
    interval_index: usize,
    current_value: u32,
}
impl<'a> InversionListIterator<'a> {
    pub fn new(list: &'a InversionList) -> Self {
        unimplemented!()
    }
}
impl<'a> Iterator for InversionListIterator<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
pub struct InversionListCoupleIterator<'a> {
    list: &'a InversionList,
    couple_index: usize,
}
impl<'a> InversionListCoupleIterator<'a> {
    pub fn new(list: &'a InversionList) -> Self {
        unimplemented!()
    }
}
impl<'a> Iterator for InversionListCoupleIterator<'a> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}