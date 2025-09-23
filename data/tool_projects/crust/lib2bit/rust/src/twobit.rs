use std::fs::File;
#[derive(Debug)]
pub struct TwoBitMaskedIdx {
    pub size: Vec<u32>,
    pub n_block_count: Vec<u32>,
    pub n_block_start: Vec<Vec<u32>>,
    pub n_block_sizes: Vec<Vec<u32>>,
    pub mask_block_count: Vec<u32>,
    pub mask_block_start: Vec<Vec<u32>>,
    pub mask_block_sizes: Vec<Vec<u32>>,
    pub offset: Vec<u64>,
}
#[derive(Debug)]
pub struct TwoBitHeader {
    pub magic: u32,
    pub version: u32,
    pub n_chroms: u32,
}
#[derive(Debug)]
pub struct TwoBitCL {
    pub chrom: Vec<String>,
    pub offset: Vec<u32>,
}
#[derive(Debug)]
pub struct TwoBit {
    pub fp: File,
    pub sz: u64,
    pub offset: u64,
    pub data: Vec<u8>,
    pub hdr: TwoBitHeader,
    pub cl: TwoBitCL,
    pub idx: TwoBitMaskedIdx,
}
impl TwoBit {
    pub fn twobit_open(fname: &str, store_masked: bool) -> Self {
        unimplemented!()
    }
    pub fn twobit_close(&mut self) {
        unimplemented!()
    }
    pub fn twobit_chrom_len(&self, chrom: &str) -> u32 {
        unimplemented!()
    }
    pub fn twobit_sequence(&self, chrom: &str, start: u32, end: u32) -> String {
        unimplemented!()
    }
    pub fn twobit_bases(&self, chrom: &str, start: u32, end: u32, fraction: i32) -> Vec<u8> {
        unimplemented!()
    }
    pub fn twobitTell(&mut self) -> u64 {
        unimplemented!()
    }
    pub fn twobitRead(&mut self, data:&Vec<u8>, sz: usize, nmemb: usize) -> usize {
        unimplemented!()
    }
    pub fn twobitSeek(&mut self, offset: u64) {
        unimplemented!()
    }
    pub fn NMask(&mut self, seq: &mut [char], tid: u32, start: u32, end: u32) {
        unimplemented!()
    }
    pub fn softMask(&mut self, seq: &mut [char], tid: u32, start: u32, end: u32) {
        unimplemented!()
    }
    pub fn constructSequence(&mut self, tid: u32, start: u32, end: u32) -> Vec<char> {
        unimplemented!()
    }
    pub fn getMask(&mut self, tid: u32, start: u32, end: u32) -> (u32, u32, u32) {
        unimplemented!()
    }
    pub fn twoBitBasesWorker(&mut self, tid: u32, start: u32, end: u32, fraction: i32) {
        unimplemented!()
    }
    pub fn twoBitIndexRead(&mut self, storeMasked: i32) {
        unimplemented!()
    }
    pub fn twoBitIndexDestroy(&mut self) {
        unimplemented!()
    }
    pub fn twobitChromListRead(&mut self) {
        unimplemented!()
    }
    pub fn twobitChromListDestroy(&mut self) {
        unimplemented!()
    }
    pub fn twobitHdrRead(&mut self) {
        unimplemented!()
    }
    pub fn twobitHdrDestroy(&mut self) {
        unimplemented!()
    }
}
// Helper function
pub fn byte2base(byte: u8, offset: i32) -> char {
    unimplemented!()
}
pub fn bytes2bases(seq: &mut [char], bytes: &mut [u8], sz: u32, offset: i32) {
    unimplemented!()
}
pub fn getByteMaskFromOffset(offset: i32) {
    unimplemented!()
}