use lib2bit::twobit::{TwoBit, TwoBitHeader, TwoBitCL, TwoBitMaskedIdx};
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
#[test]
fn test() {
    let filename = "src/bin/foo.2bit";
    let mut tb = TwoBit::twobit_open(filename, true);
    let new_file = "src/bin/result.txt";
    let mut output = File::create(new_file).expect("Unable to create file");

    for i in 0..tb.hdr.n_chroms {
        writeln!( output,
        "{}\t{}\t{} offset 0x{:x}",
        i,
        tb.cl.chrom[i as usize],
        tb.idx.size[i as usize],
        tb.idx.offset[i as usize]
        );
    }
    let seq1 = tb.twobit_sequence("chr1", 0, 0);
    writeln!(output, "{}", seq1);
    let seq2 = tb.twobit_sequence("chr1", 24, 74);
    writeln!(output, "{}", seq2);
    let stats1 = tb.twobit_bases("chr1", 0, 0, 1);
    assert!(!stats1.is_empty());
    for (i, stat) in stats1.iter().enumerate() {
        writeln!(output, "{}\t{}", i, *stat as f64);
    }
    let stats2 = tb.twobit_bases("chr1", 24, 74, 1);
    assert!(!stats2.is_empty());
    for (i, stat) in stats2.iter().enumerate() {
        writeln!(output, "{}\t{}", i, *stat as f64);
    }
    tb.twobit_close();
    output.flush().expect("Unable to flush file");
    let expected_file = "src/bin/expected.txt";
    let expected_lines = read_lines(expected_file).expect("Unable to read lines");
    let output_lines = read_lines(new_file).expect("Unable to read lines");
    assert_eq!(expected_lines, output_lines);
}

fn read_lines<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
fn main(){}