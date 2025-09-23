
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::{max, min};
use std::iter::FromIterator;




#[derive(PartialEq, Clone, Debug, Copy)]
pub struct RanState {
    pub m_w: u32,
    pub m_z: u32,
}



#[derive(PartialEq, Clone, Debug)]
pub struct ZopfliHash {
    pub head: Vec<i32>,
    pub prev: Vec<u16>,
    pub hashval: Vec<i32>,
    pub val: i32,
    pub head2: Vec<i32>,
    pub prev2: Vec<u16>,
    pub hashval2: Vec<i32>,
    pub val2: i32,
    pub same: Vec<u16>,
}



#[derive(PartialEq, Clone, Debug)]
pub struct ZopfliLongestMatchCache {
    pub length: Vec<u16>,
    pub dist: Vec<u16>,
    pub sublen: Vec<u8>,
}



#[derive(PartialEq, Clone, Debug)]
pub struct ZopfliOptions {
    pub verbose: bool,
    pub verbose_more: bool,
    pub numiterations: i32,
    pub blocksplitting: i32,
    pub blocksplittinglast: i32,
    pub blocksplittingmax: i32,
}



#[derive(PartialEq, Clone, Debug)]
pub struct ZopfliBlockState {
    pub options: ZopfliOptions,
    pub lmc: Option<ZopfliLongestMatchCache>,
    pub blockstart: usize,
    pub blockend: usize,
}


#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub weight: usize,
    pub tail: Option<Rc<RefCell<Node>>>,
    pub count: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct NodePair {
    pub node0: Option<Rc<RefCell<Node>>>,
    pub node1: Option<Rc<RefCell<Node>>>,
}



#[derive(PartialEq, Clone, Debug)]
pub struct NodePool {
    pub next: Vec<Rc<RefCell<Node>>>,
    pub index: usize,
}

impl NodePool {
    pub fn new(size: usize) -> Self {
        let mut next = Vec::with_capacity(size);
        for _ in 0..size {
            next.push(Rc::new(RefCell::new(Node {
                weight: 0,
                tail: None,
                count: 0,
            })));
        }
        NodePool { next, index: 0 }
    }

    pub fn expand(&mut self, size: usize) {
        for _ in 0..size {
            self.next.push(Rc::new(RefCell::new(Node {
                weight: 0,
                tail: None,
                count: 0,
            })));
        }
    }
}




#[derive(PartialEq, Clone, Debug)]
pub struct ZopfliLZ77Store {
    pub litlens: Vec<u16>,
    pub dists: Vec<u16>,
    pub size: usize,
    pub data: Vec<u8>,
    pub pos: Vec<usize>,
    pub ll_symbol: Vec<u16>,
    pub d_symbol: Vec<u16>,
    pub ll_counts: Vec<usize>,
    pub d_counts: Vec<usize>,
}



#[derive(PartialEq, Clone, Debug)]
pub struct SplitCostContext {
    pub lz77: ZopfliLZ77Store,
    pub start: usize,
    pub end: usize,
}



// Define the constant ZOPFLI_NUM_D with a value of 32
pub const ZOPFLI_NUM_D: usize = 32;


/// Number of distinct literal/length and distance symbols in DEFLATE
pub const ZOPFLI_NUM_LL: usize = 288;


#[derive(PartialEq, Clone, Debug, Copy)]
pub struct SymbolStats {
    pub litlens: [usize; ZOPFLI_NUM_LL],
    pub dists: [usize; ZOPFLI_NUM_D],
    pub ll_symbols: [f64; ZOPFLI_NUM_LL],
    pub d_symbols: [f64; ZOPFLI_NUM_D],
}



/// Gets the symbol for the given `dist`, according to the DEFLATE specification.
pub fn zopfli_get_dist_symbol(dist: i32) -> i32 {
    if dist < 5 {
        // If `dist` is less than 5, return `dist - 1`
        dist - 1
    } else {
        // Calculate `log2(dist - 1)` using leading zeros
        let x = (dist - 1) as u32;
        let l = 31 - x.leading_zeros() as i32; // Equivalent to floor(log2(dist - 1))
        
        // Calculate the remainder bit
        let r = ((x >> (l - 1)) & 1) as i32;
        
        // Return the final symbol
        l * 2 + r
    }
}


/// Gets the symbol for the given length, cfr. the DEFLATE spec.
/// Returns the symbol in the range [257-285] (inclusive)
pub fn ZopfliGetLengthSymbol(l: i32) -> i32 {
    // Static table mapping lengths to symbols as per DEFLATE specification
    static TABLE: [i32; 259] = [
        0, 0, 0, 257, 258, 259, 260, 261, 262, 263, 264,
        265, 265, 266, 266, 267, 267, 268, 268,
        269, 269, 269, 269, 270, 270, 270, 270,
        271, 271, 271, 271, 272, 272, 272, 272,
        273, 273, 273, 273, 273, 273, 273, 273,
        274, 274, 274, 274, 274, 274, 274, 274,
        275, 275, 275, 275, 275, 275, 275, 275,
        276, 276, 276, 276, 276, 276, 276, 276,
        277, 277, 277, 277, 277, 277, 277, 277,
        277, 277, 277, 277, 277, 277, 277, 277,
        278, 278, 278, 278, 278, 278, 278, 278,
        278, 278, 278, 278, 278, 278, 278, 278,
        279, 279, 279, 279, 279, 279, 279, 279,
        279, 279, 279, 279, 279, 279, 279, 279,
        280, 280, 280, 280, 280, 280, 280, 280,
        280, 280, 280, 280, 280, 280, 280, 280,
        281, 281, 281, 281, 281, 281, 281, 281,
        281, 281, 281, 281, 281, 281, 281, 281,
        281, 281, 281, 281, 281, 281, 281, 281,
        281, 281, 281, 281, 281, 281, 281, 281,
        282, 282, 282, 282, 282, 282, 282, 282,
        282, 282, 282, 282, 282, 282, 282, 282,
        282, 282, 282, 282, 282, 282, 282, 282,
        282, 282, 282, 282, 282, 282, 282, 282,
        283, 283, 283, 283, 283, 283, 283, 283,
        283, 283, 283, 283, 283, 283, 283, 283,
        283, 283, 283, 283, 283, 283, 283, 283,
        283, 283, 283, 283, 283, 283, 283, 283,
        284, 284, 284, 284, 284, 284, 284, 284,
        284, 284, 284, 284, 284, 284, 284, 284,
        284, 284, 284, 284, 284, 284, 284, 284,
        284, 284, 284, 284, 284, 284, 284, 285
    ];

    // Returns the symbol from the table based on the input length
    TABLE[l as usize]
}


/// Gets the amount of extra bits for the given `dist`, according to the DEFLATE specification.
/// 
/// # Arguments
/// 
/// * `dist` - The distance value for which to calculate the extra bits.
/// 
/// # Returns
/// 
/// Returns the number of extra bits as an `i32`.
pub fn zopfli_get_dist_extra_bits(dist: i32) -> i32 {
    // If the distance is less than 5, no extra bits are needed.
    if dist < 5 {
        0
    } else {
        // Calculate log2(dist - 1) - 1 using leading_zeros.
        // `leading_zeros` returns the number of leading zeros in the binary representation of (dist - 1).
        // For a 32-bit integer, subtracting leading zeros from 31 gives floor(log2(dist - 1)).
        // Finally, subtract 1 to match the C implementation.
        (31 ^ ((dist - 1) as u32).leading_zeros()) as i32 - 1
    }
}


/// Gets the amount of extra bits for the given length, cfr. the DEFLATE spec.
pub fn zopfli_get_length_extra_bits(l: i32) -> i32 {
    // Define the table as per the DEFLATE specification
    const TABLE: [i32; 259] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0-10
        1, 1, 1, 1, 1, 1, 1, 1, // 11-18
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 19-34
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 35-50
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 51-66
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 67-82
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 83-98
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 99-114
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 115-130
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 131-146
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 147-162
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 163-178
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 179-194
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 195-210
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 211-226
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 227-242
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0  // 243-258
    ];

    // Return the corresponding extra bits from the table
    TABLE[l as usize]
}


/// Cost model which should exactly match fixed tree.
///
/// This function calculates the cost based on the given literal/length (`litlen`)
/// and distance (`dist`) values according to the DEFLATE specification's fixed tree.
///
/// # Arguments
///
/// * `litlen` - The literal or length value.
/// * `dist` - The distance value.
/// * `unused` - An unused parameter, provided for compatibility.
///
/// # Returns
///
/// Returns the calculated cost as an `f64`.
pub fn GetCostFixed(litlen: u32, dist: u32, _unused: *mut std::ffi::c_void) -> f64 {
    // The `unused` parameter is not utilized in this function.
    // In Rust, prefixing the variable name with an underscore (`_`) suppresses unused variable warnings.

    if dist == 0 {
        // If the distance is zero, the cost depends solely on the `litlen` value.
        if litlen <= 143 {
            // For `litlen` values up to 143, the cost is 8.0.
            8.0
        } else {
            // For `litlen` values above 143, the cost is 9.0.
            9.0
        }
    } else {
        // For non-zero distances, additional calculations are required.

        // Calculate the number of extra bits for the distance.
        // Use `as` to convert `u32` to `i32`, similar to C's implicit conversion.
        let dbits = zopfli_get_dist_extra_bits(dist as i32);

        // Calculate the number of extra bits for the length.
        let lbits = zopfli_get_length_extra_bits(litlen as i32);

        // Retrieve the length symbol based on `litlen`.
        let lsym = ZopfliGetLengthSymbol(litlen as i32);

        // Initialize the cost accumulator.
        let mut cost: i32 = 0;

        // Determine the base cost based on the length symbol.
        if lsym <= 279 {
            // If the length symbol is 279 or below, add 7 to the cost.
            cost += 7;
        } else {
            // If the length symbol is above 279, add 8 to the cost.
            cost += 8;
        }

        // Add the fixed cost for the distance symbol.
        // Every distance symbol has a fixed length cost of 5.
        cost += 5;

        // The total cost is the sum of the base cost, distance extra bits, and length extra bits.
        // Cast the integer sum to `f64` for the return type.
        (cost + dbits + lbits) as f64
    }
}


/// Cost model based on symbol statistics.
/// Equivalent to the `GetCostStat` function in C.
///
/// # Arguments
///
/// * `litlen` - The literal or length symbol.
/// * `dist` - The distance symbol.
/// * `context` - A reference to `SymbolStats` containing symbol statistics.
///
/// # Returns
///
/// The calculated cost as an `f64`.
pub fn zopfli_get_cost_stat(litlen: i32, dist: i32, context: &SymbolStats) -> f64 {
    if dist == 0 {
        // If distance is 0, return the cost of the literal/length symbol directly.
        context.ll_symbols[litlen as usize]
    } else {
        // Get the length symbol based on the literal/length value.
        let lsym = ZopfliGetLengthSymbol(litlen);
        // Get the extra bits required for the length symbol.
        let lbits = zopfli_get_length_extra_bits(litlen);
        // Get the distance symbol based on the distance value.
        let dsym = zopfli_get_dist_symbol(dist);
        // Get the extra bits required for the distance symbol.
        let dbits = zopfli_get_dist_extra_bits(dist);
        // Calculate and return the total cost.
        (lbits as f64) + (dbits as f64) + context.ll_symbols[lsym as usize] + context.d_symbols[dsym as usize]
    }
}


use std::any::Any;

/// A type alias for a cost model function.
///
/// This function calculates a cost based on the given literal/length (`litlen`)
/// and distance (`dist`) values, using the provided `context`.
///
/// - `litlen`: The literal symbol if `dist` is 0, otherwise the length.
/// - `dist`: The distance value.
/// - `context`: A reference to any context-specific data required by the cost model.
///
/// # Returns
///
/// The calculated cost as an `f64`.
///
/// # Instances
///
/// - `GetCostFixed`: Uses `NULL` (represented as an unused parameter) as context.
/// - `GetCostStat`: Uses a reference to `SymbolStats` as context.
pub type CostModelFun = fn(litlen: u32, dist: u32, context: &dyn Any) -> f64;


/// Appends value to a dynamically allocated vector, doubling its allocation size whenever needed.
/// 
/// # Arguments
/// 
/// * `value` - The value to append, of generic type `T`.
/// * `data` - A mutable reference to the dynamic vector to append to, `Vec<T>`.
/// * `size` - A mutable reference to the size of the vector, `usize`.
///
/// The function ensures that the vector's capacity is a power of two greater than or equal to `*size`.
pub fn zopfli_append_data<T: Default + Clone>(value: T, data: &mut Vec<T>, size: &mut usize) {
    // Check if current size is a power of two, which indicates we need to double the allocation.
    if (*size==0) || (*size & (*size - 1) == 0) {
        // Double the allocation size if needed. If current size is zero, allocate initial memory.
        if *size == 0 {
            data.reserve(1);
        } else {
            data.reserve(*size);
        }
        
        // Extend the vector with default values to fill in the new capacity up to the new size.
        data.extend((0..*size).map(|_| T::default()));
    }
    
    // Insert the value at the current size position in the vector.
    if *size < data.len() {
        data[*size] = value;
    } else {
        data.push(value);
    }
    
    // Increment the size.
    *size += 1;
}


/// Gets the histogram of literal/length and distance symbols in the given range, using the
/// cumulative histograms, so faster than adding one by one for large range. Does
/// not add the one end symbol of value 256.
pub fn zopfli_lz77_get_histogram_at(
    lz77: &ZopfliLZ77Store,
    lpos: usize,
    ll_counts: &mut [usize],
    d_counts: &mut [usize],
) {
    // The real histogram is created by using the histogram for this chunk,
    // but all superfluous values of this chunk are subtracted.
    let llpos = ZOPFLI_NUM_LL * (lpos / ZOPFLI_NUM_LL);
    let dpos = ZOPFLI_NUM_D * (lpos / ZOPFLI_NUM_D);

    // Copy the cumulative literal/length counts for the current chunk.
    for i in 0..ZOPFLI_NUM_LL {
        ll_counts[i] = lz77.ll_counts[llpos + i];
    }

    // Subtract the counts for symbols beyond the current position in the chunk.
    let ll_end = std::cmp::min(llpos + ZOPFLI_NUM_LL, lz77.size);
    for i in (lpos + 1)..ll_end {
        let symbol = lz77.ll_symbol[i] as usize;
        if symbol < ll_counts.len() {
            ll_counts[symbol] -= 1;
        }
    }

    // Copy the cumulative distance counts for the current chunk.
    for i in 0..ZOPFLI_NUM_D {
        d_counts[i] = lz77.d_counts[dpos + i];
    }

    // Subtract the counts for distance symbols beyond the current position in the chunk.
    let d_end = std::cmp::min(dpos + ZOPFLI_NUM_D, lz77.size);
    for i in (lpos + 1)..d_end {
        if lz77.dists[i] != 0 {
            let d_symbol = lz77.d_symbol[i] as usize;
            if d_symbol < d_counts.len() {
                d_counts[d_symbol] -= 1;
            }
        }
    }
}


/// Gets the histogram of literal/length and distance symbols in the given range,
/// using the cumulative histograms, so faster than adding one by one for large range.
/// Does not add the one end symbol of value 256.
pub fn zopfli_lz77_get_histogram(
    lz77: &ZopfliLZ77Store,
    lstart: usize,
    lend: usize,
    ll_counts: &mut [usize],
    d_counts: &mut [usize],
) {
    if lstart + ZOPFLI_NUM_LL * 3 > lend {
        // When the range is small, compute the histogram directly.

        // Initialize ll_counts and d_counts to zero.
        for i in 0..ZOPFLI_NUM_LL {
            ll_counts[i] = 0;
        }
        for i in 0..ZOPFLI_NUM_D {
            d_counts[i] = 0;
        }
        // Accumulate counts in the given range by iterating over each symbol.
        for i in lstart..lend {
            // Increment count for the literal/length symbol.
            let ll_symbol_index = lz77.ll_symbol[i] as usize;
            ll_counts[ll_symbol_index] += 1;

            // If there is a distance symbol, increment its count.
            if lz77.dists[i] != 0 {
                let d_symbol_index = lz77.d_symbol[i] as usize;
                d_counts[d_symbol_index] += 1;
            }
        }
    } else {
        // For large ranges, use cumulative histograms for efficiency.

        // Get the cumulative histogram up to position lend - 1.
        zopfli_lz77_get_histogram_at(lz77, lend - 1, ll_counts, d_counts);

        if lstart > 0 {
            // Subtract the cumulative histogram up to position lstart - 1 to get the histogram for the range.

            // Temporary arrays to store cumulative histograms up to lstart - 1.
            let mut ll_counts2 = [0usize; ZOPFLI_NUM_LL];
            let mut d_counts2 = [0usize; ZOPFLI_NUM_D];

            // Get the cumulative histograms up to lstart - 1.
            zopfli_lz77_get_histogram_at(lz77, lstart - 1, &mut ll_counts2, &mut d_counts2);

            // Subtract the cumulative counts up to lstart - 1 from counts up to lend - 1.
            for i in 0..ZOPFLI_NUM_LL {
                ll_counts[i] = ll_counts[i].wrapping_sub(ll_counts2[i]);
            }
            for i in 0..ZOPFLI_NUM_D {
                d_counts[i] = d_counts[i].wrapping_sub(d_counts2[i]);
            }
        }
    }
}


/// Gets the amount of extra bits for the given length symbol.
pub fn zopfli_get_length_symbol_extra_bits(s: i32) -> i32 {
    // Define the static table with 29 entries.
    const TABLE: [i32; 29] = [
        0, 0, 0, 0, 0, 0, 0, 0, 
        1, 1, 1, 1, 
        2, 2, 2, 2,
        3, 3, 3, 3, 
        4, 4, 4, 4, 
        5, 5, 5, 5, 
        0,
    ];
    
    // Calculate the index by subtracting 257 from the symbol.
    let index = (s - 257) as usize;
    
    // Return the corresponding extra bits from the table.
    // If the index is out of bounds, return 0 as a default.
    TABLE.get(index).copied().unwrap_or(0)
}


/// Gets the amount of extra bits for the given distance symbol.
///
/// # Arguments
///
/// * `s` - The distance symbol as an integer.
///
/// # Returns
///
/// * The number of extra bits corresponding to the distance symbol.
///
/// # Panics
///
/// * If `s` is out of bounds (not in 0..30).
pub fn zopfli_get_dist_symbol_extra_bits(s: i32) -> i32 {
    // Define the table of extra bits for distance symbols
    let table: [i32; 30] = [
        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4,
        5, 5, 6, 6, 7, 7, 8, 8, 9, 9,
        10, 10, 11, 11, 12, 12, 13, 13,
    ];

    // Return the extra bits from the table based on the given symbol
    table[s as usize]
}


/// Same as `CalculateBlockSymbolSize`, but for block size smaller than histogram size.
pub fn calculate_block_symbol_size_small(
    ll_lengths: &[usize], // Lengths of literal/length symbols.
    d_lengths: &[usize],  // Lengths of distance symbols.
    lz77: &ZopfliLZ77Store, // The LZ77 store containing literals, lengths, and distances.
    lstart: usize,        // The starting index for the block.
    lend: usize,          // The ending index for the block.
) -> usize {
    // Initialize the result to accumulate the total symbol size.
    let mut result: usize = 0;

    // Iterate over the range from `lstart` to `lend` (exclusive).
    for i in lstart..lend {
        // Ensure that the current index `i` is within the bounds of the LZ77 store.
        assert!(
            i < lz77.size,
            "Index i ({}) out of bounds for LZ77 store size ({})",
            i,
            lz77.size
        );

        // Ensure that the literal or length at index `i` is less than 259.
        assert!(
            lz77.litlens[i] < 259,
            "Literal/Length value ({}) at index {} is out of allowed range (< 259)",
            lz77.litlens[i],
            i
        );

        // Check if the distance at index `i` is 0, indicating a literal.
        if lz77.dists[i] == 0 {
            // Add the corresponding length from `ll_lengths` to the result.
            // `lz77.litlens[i]` is a `u16`, so cast it to `usize` for indexing.
            result += ll_lengths[lz77.litlens[i] as usize];
        } else {
            // If distance is greater than 0, it's a length-distance pair.

            // Get the length symbol using the DEFLATE specification.
            let ll_symbol = ZopfliGetLengthSymbol(lz77.litlens[i] as i32);

            // Get the distance symbol using the DEFLATE specification.
            let d_symbol = zopfli_get_dist_symbol(lz77.dists[i] as i32);

            // Add the length corresponding to `ll_symbol` from `ll_lengths`.
            result += ll_lengths[ll_symbol as usize];

            // Add the distance corresponding to `d_symbol` from `d_lengths`.
            result += d_lengths[d_symbol as usize];

            // Add the extra bits required for the length symbol.
            result += zopfli_get_length_symbol_extra_bits(ll_symbol) as usize;

            // Add the extra bits required for the distance symbol.
            result += zopfli_get_dist_symbol_extra_bits(d_symbol) as usize;
        }
    }

    // After processing all symbols in the block, add the end symbol length.
    result += ll_lengths[256]; // End symbol.

    // Return the total calculated symbol size.
    result
}


/// Same as `calculate_block_symbol_size`, but with the histogram provided by the caller.
///
/// Calculates the total size of the block symbols based on the provided literal/length
/// and distance counts, as well as their corresponding lengths. This function uses the
/// histograms (`ll_counts` and `d_counts`) provided by the caller to compute the size.
/// If the block size is smaller than a certain threshold, it delegates the computation
/// to `calculate_block_symbol_size_small`.
///
/// # Arguments
///
/// * `ll_counts` - A slice containing the counts of each literal/length symbol.
/// * `d_counts` - A slice containing the counts of each distance symbol.
/// * `ll_lengths` - A slice containing the bit lengths for each literal/length symbol.
/// * `d_lengths` - A slice containing the bit lengths for each distance symbol.
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block.
///
/// # Returns
///
/// * The total size of the block symbols as a `usize`.
///
/// # Panics
///
/// * Panics if any of the indices used for accessing the slices are out of bounds.
///
/// # Examples
///
/// ```
/// let ll_counts = vec![0; 288];
/// let d_counts = vec![0; 30];
/// let ll_lengths = vec![0; 287];
/// let d_lengths = vec![0; 30];
/// let lz77 = ZopfliLZ77Store {
///     litlens: vec![],
///     dists: vec![],
///     size: 0,
///     data: vec![],
///     pos: vec![],
///     ll_symbol: vec![],
///     d_symbol: vec![],
///     ll_counts: vec![],
///     d_counts: vec![],
/// };
/// let size = calculate_block_symbol_size_given_counts(
///     &ll_counts,
///     &d_counts,
///     &ll_lengths,
///     &d_lengths,
///     &lz77,
///     0,
///     100,
/// );
/// ```
pub fn calculate_block_symbol_size_given_counts(
    ll_counts: &[usize],      // Counts of each literal/length symbol.
    d_counts: &[usize],       // Counts of each distance symbol.
    ll_lengths: &[usize],     // Bit lengths for each literal/length symbol.
    d_lengths: &[usize],      // Bit lengths for each distance symbol.
    lz77: &ZopfliLZ77Store,    // The LZ77 store containing literals, lengths, and distances.
    lstart: usize,            // The starting index for the block.
    lend: usize,              // The ending index for the block.
) -> usize {
    // Initialize the result to accumulate the total symbol size.
    let mut result: usize = 0;

    // Check if the block size is smaller than the threshold.
    if lstart + ZOPFLI_NUM_LL * 3 > lend {
        // If the block is small, delegate the computation to `calculate_block_symbol_size_small`.
        return calculate_block_symbol_size_small(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        // Iterate over the range 0..256 to accumulate sizes for literal symbols.
        for i in 0..256 {
            // Multiply the bit length of the literal symbol by its count and add to the result.
            result += ll_lengths[i] * ll_counts[i];
        }

        // Iterate over the range 257..286 to accumulate sizes for length symbols.
        for i in 257..286 {
            // Multiply the bit length of the length symbol by its count and add to the result.
            result += ll_lengths[i] * ll_counts[i];
            
            // Get the extra bits for the current length symbol.
            let extra_bits = zopfli_get_length_symbol_extra_bits(i as i32) as usize;
            
            // Multiply the extra bits by the count of the length symbol and add to the result.
            result += extra_bits * ll_counts[i];
        }

        // Iterate over the range 0..30 to accumulate sizes for distance symbols.
        for i in 0..30 {
            // Multiply the bit length of the distance symbol by its count and add to the result.
            result += d_lengths[i] * d_counts[i];
            
            // Get the extra bits for the current distance symbol.
            let extra_bits = zopfli_get_dist_symbol_extra_bits(i as i32) as usize;
            
            // Multiply the extra bits by the count of the distance symbol and add to the result.
            result += extra_bits * d_counts[i];
        }

        // Add the bit length of the end symbol (symbol 256) to the result.
        result += ll_lengths[256]; // End symbol.

        // Return the total calculated symbol size.
        return result;
    }
}


/// Calculates the size of the part after the header and tree of an LZ77 block, in bits.
///
/// This function determines whether to calculate the block symbol size using a small
/// block approach or by utilizing histograms for larger blocks. It mirrors the logic
/// of the corresponding C function `CalculateBlockSymbolSize`.
///
/// # Arguments
///
/// * `ll_lengths` - A slice containing the bit lengths for each literal/length symbol.
/// * `d_lengths` - A slice containing the bit lengths for each distance symbol.
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block.
///
/// # Returns
///
/// * The total size of the block symbols as a `usize`.
pub fn calculate_block_symbol_size(
    ll_lengths: &[usize],       // Bit lengths for literal/length symbols
    d_lengths: &[usize],        // Bit lengths for distance symbols
    lz77: &ZopfliLZ77Store,      // Reference to the LZ77 store
    lstart: usize,               // Start index of the block
    lend: usize,                 // End index of the block
) -> usize {
    // Check if the block size is smaller than the threshold for using histograms
    if lstart + ZOPFLI_NUM_LL * 3 > lend {
        // If the block is small, calculate the symbol size using the small block method
        return calculate_block_symbol_size_small(ll_lengths, d_lengths, lz77, lstart, lend);
    } else {
        // For larger blocks, use histograms to efficiently calculate the symbol size

        // Initialize vectors to store the counts of literal/length and distance symbols
        let mut ll_counts = vec![0usize; ZOPFLI_NUM_LL];
        let mut d_counts = vec![0usize; ZOPFLI_NUM_D];

        // Populate the histogram counts for the given range [lstart, lend)
        zopfli_lz77_get_histogram(lz77, lstart, lend, &mut ll_counts, &mut d_counts);

        // Calculate the total symbol size using the provided histogram counts
        return calculate_block_symbol_size_given_counts(
            &ll_counts,            // Counts of each literal/length symbol
            &d_counts,             // Counts of each distance symbol
            ll_lengths,            // Bit lengths for each literal/length symbol
            d_lengths,             // Bit lengths for each distance symbol
            lz77,                  // Reference to the LZ77 store
            lstart,                // Start index of the block
            lend,                  // End index of the block
        );
    }
}


/// Gets the amount of raw bytes that the specified range of LZ77 symbols spans.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index of the LZ77 symbol range.
/// * `lend` - The ending index of the LZ77 symbol range.
///
/// # Returns
///
/// The total number of raw bytes spanned by the LZ77 symbols from `lstart` to `lend`.
pub fn zopfli_lz77_get_byte_range(lz77: &ZopfliLZ77Store, lstart: usize, lend: usize) -> usize {
    // Calculate the index of the last symbol in the range.
    let l = lend.wrapping_sub(1);
    
    // If the start and end indices are the same, the range spans 0 bytes.
    if lstart == lend {
        return 0;
    }
    
    // Calculate the byte range:
    // - lz77.pos[l]: The position in the original data where the last LZ77 command begins.
    // - If lz77.dists[l] is 0, it's a literal, so it spans 1 byte.
    //   Otherwise, it's a length, and spans lz77.litlens[l] bytes.
    // - Subtract lz77.pos[lstart] to get the total byte range.
    lz77.pos[l]
        + if lz77.dists[l] == 0 {
            1
        } else {
            lz77.litlens[l] as usize
        }
        - lz77.pos[lstart]
}


/// Initializes the fixed tree by setting specific lengths.
///
/// This function sets the `ll_lengths` array with specific values:
/// - Indices 0..144 are set to 8
/// - Indices 144..256 are set to 9
/// - Indices 256..280 are set to 7
/// - Indices 280..288 are set to 8
///
/// The `d_lengths` array is set with all values as 5.
///
/// # Arguments
///
/// * `ll_lengths` - A mutable reference to a vector of unsigned lengths.
/// * `d_lengths` - A mutable reference to a vector of unsigned lengths.
pub fn get_fixed_tree(ll_lengths: &mut Vec<u32>, d_lengths: &mut Vec<u32>) {
    // Set ll_lengths[0..144] to 8
    for i in 0..144 {
        ll_lengths[i] = 8;
    }

    // Set ll_lengths[144..256] to 9
    for i in 144..256 {
        ll_lengths[i] = 9;
    }

    // Set ll_lengths[256..280] to 7
    for i in 256..280 {
        ll_lengths[i] = 7;
    }

    // Set ll_lengths[280..288] to 8
    for i in 280..288 {
        ll_lengths[i] = 8;
    }

    // Set d_lengths[0..32] to 5
    for i in 0..32 {
        d_lengths[i] = 5;
    }
}


/// Ensures there are at least 2 distance codes to support buggy decoders.
/// Zlib 1.2.1 and below have a bug where it fails if there isn't at least 1
/// distance code (with length > 0), even though it's valid according to the
/// deflate spec to have 0 distance codes. On top of that, some mobile phones
/// require at least two distance codes. To support these decoders too (but
/// potentially at the cost of a few bytes), add dummy code lengths of 1.
/// References to this bug can be found in the changelog of
/// Zlib 1.2.2 and here: http://www.jonof.id.au/forum/index.php?topic=515.0.
///
/// # Arguments
///
/// * `d_lengths` - The 32 lengths of the distance codes.
pub fn patch_distance_codes_for_buggy_decoders(d_lengths: &mut [u32; 32]) {
    let mut num_dist_codes = 0; // Amount of non-zero distance codes

    // Iterate through the first 30 distance codes, ignoring the last two unused codes
    for i in 0..30 {
        if d_lengths[i] != 0 {
            num_dist_codes += 1;
        }

        // If we have found at least two non-zero distance codes, no need to patch
        if num_dist_codes >= 2 {
            return; // Two or more codes is fine.
        }
    }

    // If there are no non-zero distance codes, add two dummy codes with length 1
    if num_dist_codes == 0 {
        d_lengths[0] = 1;
        d_lengths[1] = 1;
    }
    // If there is only one non-zero distance code, add one dummy code with length 1
    else if num_dist_codes == 1 {
        // Determine which index to patch based on whether the first distance code is already used
        let index = if d_lengths[0] != 0 { 1 } else { 0 };
        d_lengths[index] = 1;
    }
}


/// Converts a series of Huffman tree bit lengths to the bit values of the symbols.
///
/// # Arguments
///
/// * `lengths` - A slice of unsigned integers representing the bit lengths of the Huffman tree.
/// * `n` - The number of elements in the `lengths` slice.
/// * `maxbits` - The maximum number of bits for any symbol.
/// * `symbols` - A mutable slice where the resulting symbol bit values will be stored.
///
/// # Panics
///
/// This function will panic if any of the bit lengths in `lengths` exceed `maxbits` or if
/// the lengths of `lengths` and `symbols` do not match the specified `n`.
pub fn zopfli_lengths_to_symbols(
    lengths: &[u32],
    n: usize,
    maxbits: u32,
    symbols: &mut [u32],
) {
    // Ensure that the lengths and symbols slices have at least `n` elements
    assert!(lengths.len() >= n, "Lengths slice is shorter than n");
    assert!(symbols.len() >= n, "Symbols slice is shorter than n");

    // Allocate and initialize bl_count and next_code vectors with size (maxbits + 1)
    let mut bl_count: Vec<usize> = vec![0; (maxbits as usize) + 1];
    let mut next_code: Vec<usize> = vec![0; (maxbits as usize) + 1];
    
    // Initialize all symbols to 0
    for i in 0..n {
        symbols[i] = 0;
    }

    /* 
    1) Count the number of codes for each code length. 
    Let bl_count[N] be the number of codes of length N, N >= 1.
    */
    for bits in 0..=maxbits as usize {
        bl_count[bits] = 0;
    }

    for i in 0..n {
        let length = lengths[i];
        assert!(
            length <= maxbits,
            "Bit length at index {} exceeds maxbits",
            i
        );
        bl_count[length as usize] += 1;
    }

    /* 
    2) Find the numerical value of the smallest code for each code length.
    */
    let mut code: usize = 0;
    bl_count[0] = 0; // No codes of length 0

    for bits in 1..=maxbits as usize {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    /* 
    3) Assign numerical values to all codes, using consecutive values for all
    codes of the same length with the base values determined at step 2.
    */
    for i in 0..n {
        let len = lengths[i];
        if len != 0 {
            let len_usize = len as usize;
            symbols[i] = next_code[len_usize] as u32;
            next_code[len_usize] += 1;
        }
    }

    // No need to manually free memory as Rust's ownership system handles it
}


/// Appends `length` bits from `symbol` to the output bit stream.
///
/// # Arguments
///
/// * `symbol` - The bits to add.
/// * `length` - Number of bits to add.
/// * `bp` - Bit position (0..7) in the last byte of the output.
/// * `out` - Mutable reference to the output data vector.
/// * `outsize` - Mutable reference to the size of the output data.
///
/// # Notes
///
/// This function appends bits one by one. TODO: make more efficient (add more bits at once).
pub fn add_bits(
    symbol: u32,
    length: u32,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Iterate over each bit position from 0 to `length - 1`
    for i in 0..length {
        // Extract the i-th bit from `symbol`
        let bit = ((symbol >> i) & 1) as u8;

        // If the current bit position is 0, append a new byte initialized to 0
        if *bp == 0 {
            zopfli_append_data(0u8, out, outsize);
        }

        // Safely access the last byte in the `out` vector and set the appropriate bit
        if let Some(last_byte) = out.get_mut(*outsize - 1) {
            *last_byte |= bit << *bp;
        }

        // Increment the bit position and wrap around using modulo 8
        *bp = (*bp + 1) & 7;
    }
}


/// Adds bits to the output buffer in inverted order, as specified by the deflate standard.
/// 
/// This function appends bits from the `symbol` to the `out` buffer, starting from the most significant bit.
/// It manages the bit position within the current byte (`bp`) and ensures that the `out` buffer grows
/// dynamically as needed by doubling its allocation size.
/// 
/// # Arguments
/// 
/// * `symbol` - The bits to add, represented as a `u32`.
/// * `length` - The number of bits to add from the `symbol`.
/// * `bp` - A mutable reference to the current bit position within the last byte of `out` (0-7).
/// * `out` - A mutable reference to the output buffer, represented as a `Vec<u8>`.
/// * `outsize` - A mutable reference to the current size of the `out` buffer.
pub fn add_huffman_bits(
    symbol: u32, 
    length: u32,
    bp: &mut u8, 
    out: &mut Vec<u8>, 
    outsize: &mut usize
) {
    // TODO: make more efficient (add more bits at once).
    
    // Iterate over each bit to be added, starting from the most significant bit.
    for i in 0..length {
        // Extract the bit at the current position.
        let bit = (symbol >> (length - i - 1)) & 1;
        
        // If the bit position is at the start of a new byte, append a new byte to `out`.
        if *bp == 0 {
            // Append a 0 byte to the `out` buffer and update `outsize`.
            zopfli_append_data(0u8, out, outsize);
        }
        
        // Set the extracted bit in the last byte of `out` at the current bit position.
        out[*outsize - 1] |= (bit as u8) << *bp;
        
        // Move to the next bit position, wrapping around after 7.
        *bp = (*bp + 1) & 7;
    }
}


/// Initializes a chain node with the given values and marks it as in use.
///
/// # Arguments
///
/// * `weight` - Total weight (symbol count) of this chain.
/// * `count` - Leaf symbol index, or number of leaves before this chain.
/// * `tail` - Previous node(s) of this chain, or `None` if none.
/// * `node` - The node to initialize.
///
/// This function sets the `weight`, `count`, and `tail` fields of the provided `node`.
/// The `tail` parameter is handled as an `Option` to accommodate its nullability in C.
///
/// # Example
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let node = Rc::new(RefCell::new(Node {
///     weight: 0,
///     count: 0,
///     tail: None,
/// }));
/// init_node(10, 5, None, Rc::clone(&node));
/// assert_eq!(node.borrow().weight, 10);
/// assert_eq!(node.borrow().count, 5);
/// assert!(node.borrow().tail.is_none());
/// ```
pub fn init_node(
    weight: usize,
    count: i32,
    tail: Option<Rc<RefCell<Node>>>,
    node: Rc<RefCell<Node>>,
) {
    // Borrow the node mutably to modify its fields
    let mut node_ref = node.borrow_mut();
    
    // Set the weight of the node
    node_ref.weight = weight;
    
    // Set the count of the node
    node_ref.count = count;
    
    // Set the tail of the node, handling the possibility of it being `None`
    node_ref.tail = tail;
}


/// Performs a Boundary Package-Merge step. Puts a new chain in the given list. The
/// new chain is, depending on the weights, a leaf or a combination of two chains
/// from the previous list.
///
/// # Arguments
///
/// * `lists` - The lists of chains, each list containing two optional `Node` references.
/// * `leaves` - The leaves, one per symbol.
/// * `numsymbols` - Number of leaves (symbols).
/// * `pool` - The node memory pool.
/// * `index` - The index of the list in which a new chain or leaf is required.
///
/// This function recursively builds chains by either adding a new leaf or combining
/// two existing chains from the previous list, depending on the weights.
///
/// # Example
/// ```
/// // Example usage:
/// let mut lists = vec![NodePair { node0: None, node1: None }; maxbits];
/// let mut pool = NodePool::new(pool_size);
/// boundary_pm(&mut lists, &leaves, numsymbols, &mut pool, index);
/// ```
pub fn boundary_pm(
    lists: &mut Vec<NodePair>,                     // Mutable reference to the lists of chains
    leaves: &Vec<Rc<RefCell<Node>>>,              // Reference to the leaves
    numsymbols: usize,                             // Number of symbols
    pool: &mut NodePool,                           // Mutable reference to the node pool
    index: usize,                                  // Current index in the lists
) {
    // Ensure the current index is within the bounds of the lists
    if index >= lists.len() {
        panic!("Index out of bounds for lists");
    }

    // Retrieve the count of the last chain in the current list
    let lastcount = match &lists[index].node1 {
        Some(node_ref) => node_ref.borrow().count,
        None => 0, // Assuming a count of 0 if node1 is None
    };

    // If at the first list and the last count exceeds or equals the number of symbols, do nothing
    if index == 0 && (lastcount as usize) >= numsymbols {
        return;
    }

    // Retrieve the next available node from the pool and increment the pool's index
    if pool.index >= pool.next.len() {
        panic!("NodePool out of bounds");
    }
    let newchain = pool.next[pool.index].clone();
    pool.index += 1;

    // Retrieve the old chain from the current list's node1
    let oldchain = lists[index].node1.clone();

    // Update the current list's node0 to point to the old chain
    lists[index].node0 = oldchain.clone();

    // Update the current list's node1 to point to the new chain
    lists[index].node1 = Some(newchain.clone());

    if index == 0 {
        // If at the first list, initialize a new leaf node
        let leaf_weight = leaves[lastcount as usize]
            .borrow()
            .weight;
        init_node(
            leaf_weight,                   // Weight of the leaf
            lastcount + 1,                 // Incremented count
            None,                          // No tail since it's a leaf
            newchain.clone(),              // The new chain node
        );
    } else {
        // Calculate the sum of weights from the previous list's two chains
        let prev_node0 = match &lists[index - 1].node0 {
            Some(node_ref) => node_ref.borrow().weight,
            None => 0, // Assuming a weight of 0 if node0 is None
        };
        let prev_node1 = match &lists[index - 1].node1 {
            Some(node_ref) => node_ref.borrow().weight,
            None => 0, // Assuming a weight of 0 if node1 is None
        };
        let sum = prev_node0 + prev_node1;

        // Determine whether to insert a new leaf or combine chains
        if (lastcount as usize) < numsymbols && sum > leaves[lastcount as usize].borrow().weight {
            // Insert a new leaf node in the current list
            let tail = match &oldchain {
                Some(node_ref) => node_ref.borrow().tail.clone(),
                None => None, // No tail if oldchain is None
            };
            let leaf_weight = leaves[lastcount as usize]
                .borrow()
                .weight;
            init_node(
                leaf_weight,                   // Weight from the leaf
                lastcount + 1,                 // Incremented count
                tail,                          // Tail from the old chain
                newchain.clone(),              // The new chain node
            );
        } else {
            // Combine two chains from the previous list to form a new chain
            let prev_node1_clone = lists[index - 1].node1.clone();
            init_node(
                sum,                           // Combined weight
                lastcount,                     // Current count
                prev_node1_clone,              // Tail points to the second chain from previous list
                newchain.clone(),              // The new chain node
            );

            // Recursively perform BoundaryPM on the previous list twice
            if index >= 1 {
                boundary_pm(lists, leaves, numsymbols, pool, index - 1);
                boundary_pm(lists, leaves, numsymbols, pool, index - 1);
            } else {
                panic!("Index underflow in boundary_pm recursive call");
            }
        }
    }
}


/// Finalizes the BoundaryPM and updates the lists accordingly.
///
/// This function mirrors the behavior of the C `BoundaryPMFinal` function. It updates the
/// `lists` based on the weights of the nodes and the specified conditions. If a new chain
/// needs to be created, it retrieves a node from the `pool` and updates the corresponding
/// entries in `lists`. Otherwise, it links the current list's tail to the previous list's tail.
///
/// # Arguments
///
/// * `lists` - A mutable reference to a vector of `NodePair`s representing the current lists.
/// * `leaves` - A slice of `Rc<RefCell<Node>>` representing the leaf nodes.
/// * `numsymbols` - The number of symbols.
/// * `pool` - A mutable reference to the `NodePool` from which nodes are allocated.
/// * `index` - The current index to process in the `lists`.
pub fn boundary_pm_final(
    lists: &mut Vec<NodePair>,
    leaves: &[Rc<RefCell<Node>>],
    numsymbols: i32,
    pool: &mut NodePool,
    index: usize,
) {
    // Ensure that the index is within bounds for `lists` and `lists[index][1]`.
    assert!(index < lists.len(), "Index out of bounds for lists");
    let node1 = lists[index]
        .node1
        .as_ref()
        .expect("node1 is None at the given index");

    // Retrieve the count of the last chain of the current list at `index`.
    let lastcount = node1.borrow().count;

    // Ensure that there is a previous index to access.
    assert!(
        index > 0,
        "Index must be greater than 0 to access lists[index - 1]"
    );

    // Calculate the sum of weights from the previous index's node0 and node1.
    let previous_node0 = lists[index - 1]
        .node0
        .as_ref()
        .expect("node0 is None at the previous index");
    let previous_node1 = lists[index - 1]
        .node1
        .as_ref()
        .expect("node1 is None at the previous index");

    let sum = previous_node0.borrow().weight + previous_node1.borrow().weight;

    // Check if `lastcount` is less than `numsymbols` and `sum` exceeds the weight of the corresponding leaf.
    if lastcount < numsymbols && sum > leaves[lastcount as usize].borrow().weight {
        // Obtain a new chain node from the pool without modifying the pool index.
        // This ensures that the pool's state remains consistent with the C implementation.
        let newchain = pool.next[pool.index].clone();
        // Note: We do NOT increment `pool.index` here to match the behavior expected by the C code and the test.

        // Retrieve the old chain from the current list's node1 tail.
        let oldchain = node1.borrow().tail.clone();

        // Update the current list's node1 to point to the new chain.
        lists[index].node1 = Some(newchain.clone());

        // Set the new chain's count and tail.
        {
            let mut newchain_mut = newchain.borrow_mut();
            newchain_mut.count = lastcount + 1;
            newchain_mut.tail = oldchain;
        }
    } else {
        // Otherwise, link the current list's node1 tail to the previous index's node1.
        let previous_node1_clone = previous_node1.clone();
        {
            let mut node1_mut = node1.borrow_mut();
            node1_mut.tail = Some(previous_node1_clone);
        }
    }
}


/// Initializes each list with lookahead chains using the two leaves with the lowest weights.
///
/// # Arguments
///
/// * `pool` - A mutable reference to the `NodePool` from which nodes are allocated.
/// * `leaves` - A slice of `Node` structs representing the leaves, assumed to be sorted by weight.
/// * `maxbits` - The maximum number of bits, determining how many lists to initialize.
/// * `lists` - A mutable slice of `NodePair` where each pair will be initialized with the two lowest-weight leaves.
///
/// This function performs the following steps:
/// 1. Allocates two nodes from the `NodePool`.
/// 2. Initializes these nodes with the weights and counts from the first two leaves.
/// 3. Assigns the initialized nodes to each `NodePair` in the `lists`.
///
/// # Example
/// ```
/// let mut pool = NodePool::new(10);
/// let leaves = vec![Node { weight: 5, tail: None, count: 1 },
///                  Node { weight: 3, tail: None, count: 2 }];
/// let mut lists = vec![NodePair { node0: None, node1: None }; 4];
/// init_lists(&mut pool, &leaves, 4, &mut lists);
/// assert_eq!(lists[0].node0.as_ref().unwrap().borrow().weight, 5);
/// assert_eq!(lists[0].node1.as_ref().unwrap().borrow().weight, 3);
/// ```
pub fn init_lists(
    pool: &mut NodePool,
    leaves: &[Node],
    maxbits: usize,
    lists: &mut [NodePair],
) {
    // Ensure that there are at least two nodes available in the pool
    assert!(
        pool.index + 2 <= pool.next.len(),
        "Not enough nodes in the pool to initialize lists."
    );

    // Allocate the first node from the pool and increment the pool's index
    let node0 = pool.next[pool.index].clone();
    pool.index += 1;

    // Allocate the second node from the pool and increment the pool's index
    let node1 = pool.next[pool.index].clone();
    pool.index += 1;

    // Initialize the first node with the first leaf's weight, count, and no tail
    init_node(
        leaves[0].weight,
        1,
        None, // `tail` is set to `None` as per the C code's `0`
        node0.clone(),
    );

    // Initialize the second node with the second leaf's weight, count, and no tail
    init_node(
        leaves[1].weight,
        2,
        None, // `tail` is set to `None` as per the C code's `0`
        node1.clone(),
    );

    // Iterate over each list up to `maxbits` and assign the initialized nodes
    for list in lists.iter_mut().take(maxbits) {
        // Assign `node0` to the first position of the current list
        list.node0 = Some(node0.clone());

        // Assign `node1` to the second position of the current list
        list.node1 = Some(node1.clone());
    }
}


/// Converts result of boundary package-merge to the bitlengths. The result in the
/// last chain of the last list contains the amount of active leaves in each list.
///
/// # Arguments
///
/// * `chain` - Chain to extract the bit length from (last chain from last list).
/// * `leaves` - The leaves nodes.
/// * `bitlengths` - A mutable slice to store the bit lengths.
pub fn extract_bit_lengths(
    chain: Option<Rc<RefCell<Node>>>,
    leaves: &[Node],
    bitlengths: &mut [u32],
) {
    // Initialize counts array with zeros, size 16
    let mut counts: [i32; 16] = [0; 16];
    let mut end: usize = 16;    // Corresponds to the 'end' variable in C code
    let mut ptr: usize = 15;    // Corresponds to the 'ptr' variable in C code
    let mut value: u32 = 1;     // Corresponds to the 'value' variable in C code
    let mut node = chain;       // Start with the given chain
    let mut val: i32;

    // Iterate through the chain and fill counts array from the end
    while let Some(current_node) = node {
        end -= 1;
        counts[end] = current_node.borrow().count;
        node = current_node.borrow().tail.clone();
    }

    // Initialize val with the count from the last position
    val = counts[15];

    // Loop while ptr is greater than or equal to end
    while ptr >= end {
        // Prevent underflow when ptr is 0
        if ptr == 0 {
            break;
        }
        // Inner loop: set bit lengths for leaves where val > counts[ptr - 1]
        while val > counts[ptr - 1] {
            let leaf_index = (val - 1) as usize;
            let leaf_count = leaves[leaf_index].count as usize;
            bitlengths[leaf_count] = value;
            val -= 1;
        }
        // Decrement ptr and increment value for the next bit length
        ptr -= 1;
        value += 1;
    }
}


/// Comparator for sorting the leaves. Mimics the function signature of C's `qsort` comparator.
/// 
/// This function takes two references to `Node` and returns an `i32` representing the difference
/// in their `weight` fields. A negative value indicates that the first node has a smaller weight,
/// zero indicates equal weights, and a positive value indicates that the first node has a larger weight.
/// 
/// # Arguments
/// 
/// * `a` - A reference to the first `Node` to compare.
/// * `b` - A reference to the second `Node` to compare.
/// 
/// # Returns
/// 
/// * `i32` - The difference between the `weight` of `a` and `b`.
pub fn leaf_comparator(a: &Node, b: &Node) -> i32 {
    // Cast weights to i32 and subtract to get the comparator result
    a.weight as i32 - b.weight as i32
}


use std::cmp::Ordering;

pub fn zopfli_length_limited_code_lengths(
    frequencies: &[usize],    // const size_t *
    n: i32,                   // int
    maxbits: i32,             // int
    bitlengths: &mut [u32],   // unsigned *
) -> i32 {
    // Initialize all bitlengths to 0.
    for i in 0..(n as usize).min(bitlengths.len()) {
        bitlengths[i] = 0;
    }

    // Initialize a vector to hold the leaves (symbols with frequency > 0).
    let mut leaves_rc: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(n as usize);
    let mut numsymbols = 0; // Number of symbols with frequency > 0.

    // Populate the leaves with symbols that have a non-zero frequency.
    for i in 0..n as usize {
        if frequencies[i] > 0 {
            leaves_rc.push(Rc::new(RefCell::new(Node {
                weight: frequencies[i],
                tail: None,
                count: i as i32, // Index of the symbol.
            })));
            numsymbols += 1;
        }
    }

    // Check special cases and error conditions.
    if (1usize << maxbits) < numsymbols {
        // Error: too few maxbits to represent symbols.
        return 1;
    }
    if numsymbols == 0 {
        // No symbols at all. OK.
        return 0;
    }
    if numsymbols == 1 {
        // Only one symbol, give it bitlength 1, not 0.
        let count = leaves_rc[0].borrow().count as usize;
        if count < bitlengths.len() {
            bitlengths[count] = 1;
        }
        return 0;
    }
    if numsymbols == 2 {
        // Two symbols, assign a bitlength of 1 to each.
        let count0 = leaves_rc[0].borrow().count as usize;
        let count1 = leaves_rc[1].borrow().count as usize;
        if count0 < bitlengths.len() {
            bitlengths[count0] += 1;
        }
        if count1 < bitlengths.len() {
            bitlengths[count1] += 1;
        }
        return 0;
    }

    // Sort the leaves from lightest to heaviest.
    // To ensure stable sorting, incorporate the count into the weight.
    let max_shift = (std::mem::size_of::<usize>() * 8) - 9; // Equivalent to sizeof(weight)*CHAR_BIT - 9
    for leaf in leaves_rc.iter() {
        let mut node = leaf.borrow_mut();
        if node.weight >= (1usize << max_shift) {
            // Error: need 9 bits for the count.
            return 1;
        }
        node.weight = (node.weight << 9) | (node.count as usize);
    }

    // Sort the leaves based on the modified weight.
    leaves_rc.sort_by(|a, b| {
        let cmp_result = leaf_comparator(&a.borrow(), &b.borrow());
        if cmp_result < 0 {
            Ordering::Less
        } else if cmp_result > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // Shift the weights back by 9 bits to get the original weights.
    for leaf in leaves_rc.iter() {
        let mut node = leaf.borrow_mut();
        node.weight >>= 9;
    }

    // Create a plain vector of Nodes for functions that require &[Node].
    let leaves_plain: Vec<Node> = leaves_rc.iter().map(|leaf_rc| leaf_rc.borrow().clone()).collect();

    // Adjust maxbits if necessary.
    let mut maxbits_usize = maxbits as usize;
    if (numsymbols - 1) < maxbits_usize {
        maxbits_usize = numsymbols - 1;
    }

    // Initialize node memory pool with enough nodes.
    let pool_size = maxbits_usize * 2 * numsymbols;
    let mut pool = NodePool::new(pool_size);

    // Initialize the lists with the two lowest-weight leaves.
    let mut lists: Vec<NodePair> = vec![NodePair { node0: None, node1: None }; maxbits_usize];
    init_lists(&mut pool, &leaves_plain, maxbits_usize, &mut lists);

    // Calculate the number of BoundaryPM runs needed.
    let num_boundary_pm_runs = 2 * numsymbols - 4;

    // Perform BoundaryPM steps.
    for _ in 0..(num_boundary_pm_runs - 1) {
        // Perform a BoundaryPM step on the last list (maxbits_usize - 1).
        boundary_pm(
            &mut lists,
            &leaves_rc,
            numsymbols,
            &mut pool,
            maxbits_usize - 1,
        );
    }

    // Perform the final BoundaryPM step.
    boundary_pm_final(
        &mut lists,
        &leaves_rc,
        numsymbols as i32,
        &mut pool,
        maxbits_usize - 1,
    );

    // Extract the bit lengths from the resulting chains.
    if let Some(last_list) = lists.get(maxbits_usize - 1) {
        extract_bit_lengths(last_list.node1.clone(), &leaves_plain, bitlengths);
    } else {
        // If the last list does not exist, return an error.
        return 1;
    }

    // If everything succeeded, return 0.
    0
}


/// Calculates the bitlengths for the Huffman tree, based on the counts of each symbol.
///
/// This function calls `zopfli_length_limited_code_lengths` and asserts that there is no error,
/// equivalent to the C function `ZopfliCalculateBitLengths`.
///
/// # Arguments
///
/// * `count` - The amount of occurrences of each symbol.
/// * `n` - The number of symbols.
/// * `maxbits` - Maximum bit length, inclusive.
/// * `bitlengths` - Output, the bitlengths for the symbol prefix codes.
///
/// # Panics
///
/// Panics if `zopfli_length_limited_code_lengths` returns an error.
pub fn zopfli_calculate_bit_lengths(
    count: &[usize],    // const size_t *
    n: i32,             // size_t converted to i32
    maxbits: i32,       // int
    bitlengths: &mut [u32], // unsigned *
) {
    // Call ZopfliLengthLimitedCodeLengths and capture the error code.
    let error = zopfli_length_limited_code_lengths(count, n, maxbits, bitlengths);
    
    // Ignore the error value and assert that there was no error.
    assert_eq!(error, 0, "ZopfliCalculateBitLengths: unexpected error.");
}


/// Encodes the Huffman tree and returns the number of bits its encoding takes.
///
/// If `out` is `None`, the function only calculates the size and runs faster.
///
/// # Arguments
///
/// * `ll_lengths` - A slice of unsigned integers representing the bit lengths of the literal/length Huffman tree.
/// * `d_lengths` - A slice of unsigned integers representing the bit lengths of the distance Huffman tree.
/// * `use_16` - An integer flag indicating whether to use repeat codes of length 3 (code 16).
/// * `use_17` - An integer flag indicating whether to use repeat codes of length 3 (code 17).
/// * `use_18` - An integer flag indicating whether to use repeat codes of length 11 (code 18).
/// * `bp` - An optional mutable reference to the current bit position within the last byte of the output.
/// * `out` - An optional mutable reference to the output buffer (`Vec<u8>`) where the encoded tree bits will be stored.
/// * `outsize` - An optional mutable reference to the current size of the `out` buffer.
///
/// # Returns
///
/// The total number of bits used to encode the Huffman tree.
///
/// # Panics
///
/// This function will panic if any of the bit lengths in `ll_lengths` or `d_lengths` exceed their respective maxima,
/// or if the provided slices do not have sufficient length as per the expected counts.
pub fn encode_tree(
    ll_lengths: &[u32],
    d_lengths: &[u32],
    use_16: i32,
    use_17: i32,
    use_18: i32,
    bp: Option<&mut u8>,
    out: Option<&mut Vec<u8>>,
    outsize: Option<&mut usize>,
) -> usize {
    // Initialize variables
    let mut rle: Vec<u32> = Vec::new(); // Run-length encoded symbols
    let mut rle_bits: Vec<u32> = Vec::new(); // Extra bits for run-length encoded symbols
    let mut rle_size: usize = 0; // Size of the run-length encoded symbols
    let mut rle_bits_size: usize = 0; // Size of the extra bits for run-length encoded symbols
    let mut hlit: usize = 29; // Number of literal/length codes (maximum 286 - 257 = 29)
    let mut hdist: usize = 29; // Number of distance codes (maximum 32 - 1 = 31, but gzip does not like hdist > 29)
    let mut hclen: usize; // Number of code length code lengths
    let mut hlit2: usize; // Total number of literal/length codes (hlit + 257)
    let mut lld_total: usize; // Total amount of literal, length, distance codes
    let mut clcounts: [usize; 19] = [0; 19]; // Counts of each code length code
    let mut clcl: [u32; 19] = [0; 19]; // Code length code lengths
    let mut clsymbols: [u32; 19] = [0; 19]; // Code length code symbols
    // The order in which code length code lengths are encoded as per deflate
    let order: [usize; 19] = [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
    ];
    let mut result_size: usize = 0; // Total size in bits

    // Trim zeros from hlit and hdist
    while hlit > 0 && ll_lengths[257 + hlit - 1] == 0 {
        hlit -= 1;
    }
    while hdist > 0 && d_lengths[1 + hdist - 1] == 0 {
        hdist -= 1;
    }
    hlit2 = hlit + 257;

    // Total number of literal, length, and distance codes
    lld_total = hlit2 + hdist + 1;

    // Run-length encode the lengths of the literal/length and distance trees
    let mut i: usize = 0;
    while i < lld_total {
        // Determine the current symbol based on whether it's a literal/length or distance code
        let symbol: u32 = if i < hlit2 {
            ll_lengths[i]
        } else {
            d_lengths[i - hlit2]
        };
        let mut count: usize = 1;

        // Check if we can use repeat codes (16, 17, 18)
        if (use_16 != 0) || (symbol == 0 && (use_17 != 0 || use_18 != 0)) {
            // Count how many consecutive symbols are the same
            while (i + count) < lld_total {
                let next_symbol: u32 = if (i + count) < hlit2 {
                    ll_lengths[i + count]
                } else {
                    d_lengths[i + count - hlit2]
                };
                if next_symbol != symbol {
                    break;
                }
                count += 1;
            }
        }

        // Advance the index by the number of counted symbols
        i += count;

        // Handle repetitions of zeros with run-length encoding using codes 17 and 18
        if symbol == 0 && count >= 3 {
            if use_18 != 0 {
                while count >= 11 {
                    let count2: usize = if count > 138 { 138 } else { count };
                    if out.is_some() && outsize.is_some() {
                        rle.push(18);
                        rle_bits.push((count2 - 11) as u32);
                        rle_size += 1;
                        rle_bits_size += 1;
                    }
                    clcounts[18] += 1;
                    count -= count2;
                }
            }
            if use_17 != 0 {
                while count >= 3 {
                    let count2: usize = if count > 10 { 10 } else { count };
                    if out.is_some() && outsize.is_some() {
                        rle.push(17);
                        rle_bits.push((count2 - 3) as u32);
                        rle_size += 1;
                        rle_bits_size += 1;
                    }
                    clcounts[17] += 1;
                    count -= count2;
                }
            }
        }

        // Handle repetitions of any symbol with run-length encoding using code 16
        if use_16 != 0 && count >= 4 {
            count -= 1; // The first occurrence is stored as the symbol itself
            clcounts[symbol as usize] += 1;
            if out.is_some() && outsize.is_some() {
                rle.push(symbol);
                rle_bits.push(0);
                rle_size += 1;
                rle_bits_size += 1;
            }
            while count >= 3 {
                let count2: usize = if count > 6 { 6 } else { count };
                if out.is_some() && outsize.is_some() {
                    rle.push(16);
                    rle_bits.push((count2 - 3) as u32);
                    rle_size += 1;
                    rle_bits_size += 1;
                }
                clcounts[16] += 1;
                count -= count2;
            }
        }

        // Handle remaining repetitions and single symbols
        clcounts[symbol as usize] += count;
        while count > 0 {
            if out.is_some() && outsize.is_some() {
                rle.push(symbol);
                rle_bits.push(0);
                rle_size += 1;
                rle_bits_size += 1;
            }
            count -= 1;
        }
    }

    // Calculate the bit lengths for the code length codes
    zopfli_calculate_bit_lengths(&clcounts, 19, 7, &mut clcl);

    // Assign symbols based on the calculated bit lengths
    zopfli_lengths_to_symbols(&clcl, 19, 7, &mut clsymbols);

    // Determine the number of code length codes (hclen) by trimming trailing zeros
    hclen = 15;
    while hclen > 0 && clcounts[order[hclen + 4 - 1]] == 0 {
        hclen -= 1;
    }

    // If `out` is provided, write the encoded tree bits
    if let (Some(bp_ref), Some(out_ref), Some(outsize_ref)) = (bp, out, outsize) {
        // Add hlit (5 bits), hdist (5 bits), and hclen (4 bits) to the output
        add_bits(hlit as u32, 5, bp_ref, out_ref, outsize_ref);
        add_bits(hdist as u32, 5, bp_ref, out_ref, outsize_ref);
        add_bits(hclen as u32, 4, bp_ref, out_ref, outsize_ref);

        // Add the code length code lengths in the specified order (3 bits each)
        for i in 0..(hclen + 4) {
            add_bits(clcl[order[i]], 3, bp_ref, out_ref, outsize_ref);
        }

        // Encode the run-length encoded symbols and their extra bits
        for i in 0..rle_size {
            let symbol = clsymbols[rle[i] as usize];
            add_huffman_bits(symbol, clcl[rle[i] as usize], bp_ref, out_ref, outsize_ref);

            // Add the extra bits based on the run-length symbol
            if rle[i] == 16 {
                add_bits(rle_bits[i], 2, bp_ref, out_ref, outsize_ref);
            } else if rle[i] == 17 {
                add_bits(rle_bits[i], 3, bp_ref, out_ref, outsize_ref);
            } else if rle[i] == 18 {
                add_bits(rle_bits[i], 7, bp_ref, out_ref, outsize_ref);
            }
        }
    }

    // Calculate the total size in bits
    result_size += 14; // hlit (5 bits) + hdist (5 bits) + hclen (4 bits)
    result_size += (hclen + 4) * 3; // Each clcl entry is 3 bits
    for i in 0..19 {
        result_size += clcl[i] as usize * clcounts[i];
    }
    // Add the bits for extra information in run-length encoding
    result_size += clcounts[16] * 2;
    result_size += clcounts[17] * 3;
    result_size += clcounts[18] * 7;

    // The `rle` and `rle_bits` vectors are automatically freed when they go out of scope

    // Return the total size in bits
    result_size
}


/// Calculates the exact size of the Huffman tree in bits as it will be encoded in DEFLATE.
///
/// This function iterates through all possible combinations of the `use_16`, `use_17`, and `use_18` flags,
/// encodes the Huffman tree for each combination using the `encode_tree` function, and returns the minimum
/// size in bits among all encoded trees.
///
/// # Arguments
///
/// * `ll_lengths` - A slice of unsigned integers representing the bit lengths of the literal/length Huffman tree.
/// * `d_lengths` - A slice of unsigned integers representing the bit lengths of the distance Huffman tree.
///
/// # Returns
///
/// The exact size of the Huffman tree in bits as it will be encoded in DEFLATE.
pub fn calculate_tree_size(ll_lengths: &[u32], d_lengths: &[u32]) -> usize {
    // Initialize the result size to zero.
    let mut result: usize = 0;

    // Iterate over all 8 possible combinations of use_16, use_17, and use_18 flags.
    for i in 0..8 {
        // Determine the flags by extracting individual bits from the loop index.
        let use_16: i32 = (i & 1) as i32;
        let use_17: i32 = (i & 2) as i32;
        let use_18: i32 = (i & 4) as i32;

        // Encode the tree with the current combination of flags.
        // Pass `None` for `bp`, `out`, and `outsize` to calculate size only.
        let size = encode_tree(
            ll_lengths,
            d_lengths,
            use_16,
            use_17,
            use_18,
            None,
            None,
            None,
        );

        // If this is the first iteration or the current size is smaller than the previously stored result,
        // update the result with the current size.
        if result == 0 || size < result {
            result = size;
        }
    }

    // Return the minimum size found across all flag combinations.
    result
}


/// Computes the absolute difference between two `usize` values.
///
/// # Arguments
///
/// * `x` - The first `usize` value.
/// * `y` - The second `usize` value.
///
/// # Returns
///
/// The absolute difference between `x` and `y`.
pub fn abs_diff(x: usize, y: usize) -> usize {
    // Check if `x` is greater than `y`
    if x > y {
        // If `x` is greater, return `x - y`
        x - y
    } else {
        // Otherwise, return `y - x`
        y - x
    }
}


/// Changes the population counts in a way that the consequent Huffman tree
/// compression, especially its RLE part, will be more likely to compress this data
/// more efficiently. The `counts` slice contains the size of the histogram.
///
/// # Arguments
///
/// * `counts` - A mutable slice of `usize` representing the population counts.
///
/// # Description
///
/// This function optimizes the Huffman tree's population counts to enhance the efficiency
/// of subsequent RLE compression. It performs the following steps:
///
/// 1. Removes trailing zeros from the `counts` slice to prevent breaking the format rules.
/// 2. Marks sequences in `counts` that are already suitable for RLE encoding.
/// 3. Replaces population counts to create better RLE runs based on certain heuristics.
pub fn optimize_huffman_for_rle(counts: &mut [usize]) {
    let mut length = counts.len();

    // Step 1: Remove trailing zeros
    // We don't want to touch the trailing zeros. We may break the
    // rules of the format by adding more data in the distance codes.
    while length > 0 && counts[length - 1] == 0 {
        length -= 1;
    }
    // If all counts are zero, return early
    if length == 0 {
        return;
    }

    // Step 2: Initialize good_for_rle array
    // This array will mark positions in counts that are already good for RLE encoding
    let mut good_for_rle = vec![0i32; length];

    // Step 2: Mark counts that are already good for RLE
    // We mark any sequence of zeros longer than 5 and any sequence of non-zeros longer than 7
    let mut symbol = counts[0];
    let mut stride = 0usize;

    // Iterate over counts from 0 to length (inclusive)
    for i in 0..=length {
        if i == length || counts[i] != symbol {
            if (symbol == 0 && stride >= 5) || (symbol != 0 && stride >= 7) {
                // Mark positions in good_for_rle where we have good RLE sequences
                for k in 0..stride {
                    // Ensure we don't underflow
                    if i >= k + 1 {
                        good_for_rle[i - k - 1] = 1;
                    }
                }
            }
            stride = 1;
            if i != length {
                symbol = counts[i];
            }
        } else {
            stride += 1;
        }
    }

    // Step 3: Replace counts to create better RLE runs
    stride = 0;
    let mut limit = counts[0];
    let mut sum = 0usize;

    for i in 0..=length {
        let good_rle = if i < length { good_for_rle[i] != 0 } else { false };
        let counts_i = if i < length { counts[i] } else { 0 };

        // Heuristic for selecting the stride ranges to collapse
        if i == length || good_rle || abs_diff(counts_i, limit) >= 4 {
            if stride >= 4 || (stride >= 3 && sum == 0) {
                // The stride must end; collapse what we have if we have enough (at least 4)
                // Calculate the average count over the stride
                let mut count = (sum + (stride / 2)) / stride;
                if count < 1 {
                    count = 1;
                }
                if sum == 0 {
                    // Don't upgrade an all-zeros stride to ones
                    count = 0;
                }
                // Replace counts in the stride with the calculated count
                for k in 0..stride {
                    if i >= k + 1 {
                        counts[i - k - 1] = count;
                    }
                }
            }
            stride = 0;
            sum = 0;
            // Update the limit for the next stride
            if i + 3 < length {
                // All interesting strides have a count of at least 4, at least when non-zeros
                limit = (counts[i] + counts[i + 1] + counts[i + 2] + counts[i + 3] + 2) / 4;
            } else if i < length {
                limit = counts[i];
            } else {
                limit = 0;
            }
        }
        stride += 1;
        if i < length {
            sum += counts[i];
        }
    }
}


/// Tries out `optimize_huffman_for_rle` for the given block. If the optimized result is smaller,
/// it replaces the original Huffman lengths with the optimized ones. Otherwise, it retains the
/// original lengths. The function returns the total size of the encoded tree and data in bits,
/// excluding the 3-bit block header.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block.
/// * `ll_counts` - A slice containing the counts of each literal/length symbol.
/// * `d_counts` - A slice containing the counts of each distance symbol.
/// * `ll_lengths` - A mutable slice containing the bit lengths for each literal/length symbol.
/// * `d_lengths` - A mutable slice containing the bit lengths for each distance symbol.
///
/// # Returns
///
/// * The total size of the encoded tree and data in bits as a `f64`.
///
/// # Panics
///
/// * Panics if any of the indices used for accessing the slices are out of bounds.
/// * Panics if `d_lengths2` does not have exactly 32 elements.
pub fn try_optimize_huffman_for_rle(
    lz77: &ZopfliLZ77Store,    // Reference to the LZ77 store.
    lstart: usize,             // Starting index for the block.
    lend: usize,               // Ending index for the block.
    ll_counts: &[usize],       // Counts of each literal/length symbol.
    d_counts: &[usize],        // Counts of each distance symbol.
    ll_lengths: &mut [u32],    // Bit lengths for each literal/length symbol.
    d_lengths: &mut [u32],     // Bit lengths for each distance symbol.
) -> f64 {
    // Calculate the original size of the Huffman tree.
    let treesize = calculate_tree_size(ll_lengths, d_lengths) as f64;
    
    // Calculate the original size of the block symbols.
    let datasize = calculate_block_symbol_size_given_counts(
        ll_counts,
        d_counts,
        &ll_lengths.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
        &d_lengths.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
        lz77,
        lstart,
        lend,
    ) as f64;
    
    // Initialize copies of the literal/length and distance counts.
    let mut ll_counts2 = ll_counts.to_vec(); // Copy of ll_counts.
    let mut d_counts2 = d_counts.to_vec();   // Copy of d_counts.
    
    // Optimize the copied counts for RLE compression.
    optimize_huffman_for_rle(&mut ll_counts2);
    optimize_huffman_for_rle(&mut d_counts2);
    
    // Initialize fixed-size arrays to hold the optimized bit lengths for literal/length and distance symbols.
    let mut ll_lengths2 = [0u32; ZOPFLI_NUM_LL]; // Optimized bit lengths for literal/length symbols.
    let mut d_lengths2 = [0u32; ZOPFLI_NUM_D];   // Optimized bit lengths for distance symbols.
    
    // Calculate the bit lengths for the optimized literal/length counts.
    zopfli_calculate_bit_lengths(
        &ll_counts2,
        ZOPFLI_NUM_LL as i32,
        15,
        &mut ll_lengths2,
    );
    
    // Calculate the bit lengths for the optimized distance counts.
    zopfli_calculate_bit_lengths(
        &d_counts2,
        ZOPFLI_NUM_D as i32,
        15,
        &mut d_lengths2,
    );
    
    // Patch the distance codes to ensure compatibility with buggy decoders.
    patch_distance_codes_for_buggy_decoders(&mut d_lengths2);
    
    // Calculate the size of the optimized Huffman tree.
    let treesize2 = calculate_tree_size(&ll_lengths2, &d_lengths2) as f64;
    
    // Calculate the size of the optimized block symbols.
    let datasize2 = calculate_block_symbol_size_given_counts(
        ll_counts,
        d_counts,
        &ll_lengths2.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
        &d_lengths2.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
        lz77,
        lstart,
        lend,
    ) as f64;
    
    // Compare the original size with the optimized size.
    if treesize2 + datasize2 < treesize + datasize {
        // If the optimized size is smaller, replace the original bit lengths with the optimized ones.
        ll_lengths.copy_from_slice(&ll_lengths2);
        d_lengths.copy_from_slice(&d_lengths2);
        
        // Return the optimized total size.
        treesize2 + datasize2
    } else {
        // If the original size is smaller or equal, retain the original bit lengths.
        
        // Return the original total size.
        treesize + datasize
    }
}


/// Calculates the bit lengths for the symbols for dynamic blocks.
/// Chooses bit lengths that give the smallest size of tree encoding + encoding of all the
/// symbols to have the smallest output size. These are not necessarily the ideal Huffman
/// bit lengths. Returns the size of the encoded tree and data in bits, not including the
/// 3-bit block header.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block.
/// * `ll_lengths` - A mutable reference to an array containing the bit lengths for literal/length symbols.
/// * `d_lengths` - A mutable reference to an array containing the bit lengths for distance symbols.
///
/// # Returns
///
/// * The size of the encoded tree and data in bits as a `f64`.
pub fn get_dynamic_lengths(
    lz77: &ZopfliLZ77Store,
    lstart: usize,
    lend: usize,
    ll_lengths: &mut [u32; ZOPFLI_NUM_LL],
    d_lengths: &mut [u32; ZOPFLI_NUM_D],
) -> f64 {
    // Initialize the literal/length and distance counts arrays with zeros.
    let mut ll_counts = [0usize; ZOPFLI_NUM_LL];
    let mut d_counts = [0usize; ZOPFLI_NUM_D];

    // Get the literal/length and distance counts for the specified range.
    zopfli_lz77_get_histogram(lz77, lstart, lend, &mut ll_counts, &mut d_counts);

    // Add one count for the end symbol (256).
    // This ensures that the end symbol is always present in the histogram.
    ll_counts[256] += 1;

    // Calculate the bit lengths for the literal/length codes based on their counts.
    // The maximum bit length is set to 15, as per the DEFLATE specification.
    zopfli_calculate_bit_lengths(&ll_counts, ZOPFLI_NUM_LL as i32, 15, ll_lengths);

    // Calculate the bit lengths for the distance codes based on their counts.
    // The maximum bit length is also set to 15.
    zopfli_calculate_bit_lengths(&d_counts, ZOPFLI_NUM_D as i32, 15, d_lengths);

    // Patch the distance codes to ensure compatibility with buggy decoders.
    // This ensures that there are at least two distance codes, as some decoders require it.
    patch_distance_codes_for_buggy_decoders(d_lengths);

    // Optimize the Huffman lengths for run-length encoding (RLE).
    // This may reduce the total size of the encoded tree and data.
    let size = try_optimize_huffman_for_rle(
        lz77,        // Reference to the LZ77 store containing the data.
        lstart,      // Starting index for the block.
        lend,        // Ending index for the block.
        &ll_counts,  // Reference to the literal/length counts.
        &d_counts,   // Reference to the distance counts.
        ll_lengths,  // Mutable reference to the literal/length bit lengths.
        d_lengths,   // Mutable reference to the distance bit lengths.
    );

    // Return the total size of the encoded tree and data in bits.
    size
}


/// Calculates the size of an LZ77 block in bits.
///
/// This function mirrors the C function `ZopfliCalculateBlockSize` and determines the total
/// size of an LZ77 block based on the block type (`btype`). The block type can be:
/// - `0`: Uncompressed block.
/// - `1`: Compressed block with fixed Huffman codes.
/// - `2`: Compressed block with dynamic Huffman codes.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block (not inclusive).
/// * `btype` - The block type (`0`, `1`, or `2`).
///
/// # Returns
///
/// * The total size of the block in bits as a `f64`.
pub fn zopfli_calculate_block_size(
    lz77: &ZopfliLZ77Store, // Reference to the LZ77 store
    lstart: usize,          // Start index of the block
    lend: usize,            // End index of the block (not inclusive)
    btype: i32,             // Block type: 0 (uncompressed), 1 (fixed), 2 (dynamic)
) -> f64 {
    // Initialize the result with 3 bits for the block header (bfinal and btype).
    let mut result: f64 = 3.0;

    if btype == 0 {
        // Block type 0: Uncompressed block.

        // Calculate the total number of raw bytes spanned by the LZ77 symbols in the range [lstart, lend).
        let length = zopfli_lz77_get_byte_range(lz77, lstart, lend);

        // Calculate the remainder when dividing the length by 65535.
        let rem = length % 65535;

        // Determine the number of blocks needed (each block can be at most 65535 bytes).
        let blocks = (length / 65535) + if rem != 0 { 1 } else { 0 };

        // Each uncompressed block header is 5 bytes (40 bits):
        // - 3 bits: bfinal and btype
        // - 5 bytes: LEN and NLEN fields
        // Total size in bits: (blocks * 5 * 8) + (length * 8)
        //
        // Note:
        // - An uncompressed block must be split into multiple blocks if it's larger than 65535 bytes.
        // - The first block may have padding bits, but they are ignored in this calculation.
        return (blocks * 5 * 8 + length * 8) as f64;
    }

    if btype == 1 {
        // Block type 1: Compressed block with fixed Huffman codes.

        // Initialize vectors to store the bit lengths for literal/length and distance symbols.
        let mut ll_lengths = vec![0u32; ZOPFLI_NUM_LL]; // Vec<u32> of size 288
        let mut d_lengths = vec![0u32; ZOPFLI_NUM_D];   // Vec<u32> of size 32

        // Initialize the fixed Huffman trees by setting specific bit lengths for literals/lengths and distances.
        get_fixed_tree(&mut ll_lengths, &mut d_lengths);

        // Convert the bit lengths from u32 to usize to match the expected types of `calculate_block_symbol_size`.
        let ll_lengths_usize: Vec<usize> = ll_lengths.iter().map(|&x| x as usize).collect();
        let d_lengths_usize: Vec<usize> = d_lengths.iter().map(|&x| x as usize).collect();

        // Calculate the size of the block symbols using the fixed Huffman trees.
        // This includes the size of the encoded symbols (literals, lengths, distances).
        result += calculate_block_symbol_size(
            &ll_lengths_usize, // Bit lengths for literal/length symbols as &[usize]
            &d_lengths_usize,  // Bit lengths for distance symbols as &[usize]
            lz77,              // Reference to the LZ77 store
            lstart,            // Start index of the block
            lend,              // End index of the block
        ) as f64;
    } else {
        // Block type 2: Compressed block with dynamic Huffman codes.

        // Initialize arrays to store the bit lengths for literal/length and distance symbols.
        let mut ll_lengths_array = [0u32; ZOPFLI_NUM_LL]; // [u32; 288]
        let mut d_lengths_array = [0u32; ZOPFLI_NUM_D];   // [u32; 32]

        // Calculate the size of the block symbols using dynamic Huffman trees.
        // This involves determining optimal bit lengths for the Huffman codes based on symbol frequencies.
        result += get_dynamic_lengths(
            lz77,                  // Reference to the LZ77 store
            lstart,                // Start index of the block
            lend,                  // End index of the block
            &mut ll_lengths_array, // Mutable reference to array [u32; 288]
            &mut d_lengths_array,  // Mutable reference to array [u32; 32]
        );

        // Note: In the dynamic block case, `get_dynamic_lengths` already returns the total size,
        // including the size of the encoded tree and the data, so we don't need to call
        // `calculate_block_symbol_size` here.
    }

    // Return the total size of the block in bits, including the header and symbol encoding.
    result
}


/// Calculates the block size in bits, automatically selecting the best block type (`btype`).
///
/// This function mirrors the C function `ZopfliCalculateBlockSizeAutoType` and determines
/// the optimal block type (`btype`) by comparing the costs of different compression strategies:
/// - Uncompressed block (`btype = 0`)
/// - Compressed block with fixed Huffman codes (`btype = 1`)
/// - Compressed block with dynamic Huffman codes (`btype = 2`)
///
/// The function selects the block type that results in the smallest total size in bits.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for the block.
/// * `lend` - The ending index for the block (not inclusive).
///
/// # Returns
///
/// * The total size of the block in bits as a `f64`.
pub fn zopfli_calculate_block_size_auto_type(
    lz77: &ZopfliLZ77Store, // Reference to the LZ77 store
    lstart: usize,          // Start index of the block
    lend: usize,            // End index of the block (not inclusive)
) -> f64 {
    // Calculate the block size using uncompressed block type (btype = 0).
    // This represents the cost if the block is stored without compression.
    let uncompressedcost = zopfli_calculate_block_size(lz77, lstart, lend, 0);

    // Determine the cost for fixed Huffman block type (btype = 1).
    //
    // If the size of the LZ77 store is greater than 1000, it's assumed that
    // using an uncompressed block may be more efficient, so `fixedcost` is set to
    // `uncompressedcost` to avoid the expensive fixed Huffman cost calculation.
    // Otherwise, calculate the block size with fixed Huffman codes.
    let fixedcost = if lz77.size > 1000 {
        uncompressedcost
    } else {
        zopfli_calculate_block_size(lz77, lstart, lend, 1)
    };

    // Calculate the block size using dynamic Huffman block type (btype = 2).
    // This represents the cost if the block is stored with dynamically generated Huffman codes.
    let dyncost = zopfli_calculate_block_size(lz77, lstart, lend, 2);

    // Determine the smallest cost among the three block types.
    // - If the uncompressed cost is less than both fixed and dynamic costs, select uncompressed.
    // - Otherwise, select the smaller of fixed and dynamic costs.
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        uncompressedcost
    } else if fixedcost < dyncost {
        fixedcost
    } else {
        dyncost
    }
}


/// Returns estimated cost of a block in bits. It includes the size to encode the
/// tree and the size to encode all literal, length and distance symbols and their
/// extra bits.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index of the block.
/// * `lend` - The ending index of the block (not inclusive).
///
/// # Returns
///
/// * The estimated cost of the block in bits as a `f64`.
pub fn estimate_cost(lz77: &ZopfliLZ77Store, lstart: usize, lend: usize) -> f64 {
    // Call the function to calculate the block size automatically selecting the best block type.
    zopfli_calculate_block_size_auto_type(lz77, lstart, lend)
}


/// Calculates the total cost by summing the cost of the left and right sections of the data.
///
/// This function takes a split index and a context, then estimates the cost of the data
/// from the start to the split index and from the split index to the end. The total cost
/// is the sum of these two estimates.
///
/// # Arguments
///
/// * `i` - The index at which to split the data.
/// * `context` - A reference to the `SplitCostContext` containing the necessary context
///               for cost calculation.
///
/// # Returns
///
/// * The total estimated cost as a `f64`.
pub fn split_cost(i: usize, context: &SplitCostContext) -> f64 {
    // Retrieve a reference to the ZopfliLZ77Store from the context
    let lz77 = &context.lz77;

    // Estimate the cost of the left section from `start` to `i`
    let left_cost = estimate_cost(lz77, context.start, i);

    // Estimate the cost of the right section from `i` to `end`
    let right_cost = estimate_cost(lz77, i, context.end);

    // Sum the costs of both sections and return the total
    left_cost + right_cost
}


/// Type alias for the `FindMinimumFun` function pointer in C.
///
/// This function takes a `usize` representing the current parameter `i`,
/// and a reference to some context data encapsulated in `dyn Any`.
/// It returns a `f64` representing the minimum cost.
///
/// # Arguments
///
/// * `i` - The current parameter of the function `f(i)`.
/// * `context` - A reference to the context data, allowing passing arbitrary data.
///
/// # Returns
///
/// * `f64` - The minimum cost.
pub type FindMinimumFun = Box<dyn Fn(usize, &dyn std::any::Any) -> f64>;


/// Appends the length and distance to the LZ77 arrays of the `ZopfliLZ77Store`.
///
/// # Arguments
///
/// * `length` - The length value to append.
/// * `dist` - The distance value to append.
/// * `pos` - The position in data where this LZ77 command begins.
/// * `store` - Mutable reference to the `ZopfliLZ77Store`.
///
/// This function updates the `litlens`, `dists`, `pos`, `ll_symbol`, `d_symbol`,
/// `ll_counts`, and `d_counts` vectors of the store accordingly.
///
/// It manages the internal histograms (`ll_counts` and `d_counts`) by appending
/// new counts when the current size wraps around based on `ZOPFLI_NUM_LL` and
/// `ZOPFLI_NUM_D`. It then appends the provided `length`, `dist`, and `pos` to
/// their respective vectors. Depending on whether `dist` is zero, it updates
/// the symbols and increments the appropriate histogram counts.
pub fn zopfli_store_lit_len_dist(
    length: u16,
    dist: u16,
    pos: usize,
    store: &mut ZopfliLZ77Store,
) {
    // Store the original size before any modifications.
    let origsize = store.size;

    // Calculate the starting indices for `ll_counts` and `d_counts`.
    let llstart = ZOPFLI_NUM_LL * (origsize / ZOPFLI_NUM_LL);
    let dstart = ZOPFLI_NUM_D * (origsize / ZOPFLI_NUM_D);

    // Every time the index wraps around for literals/lengths, append to `ll_counts`.
    if origsize % ZOPFLI_NUM_LL == 0 {
        let mut llsize = origsize;
        for i in 0..ZOPFLI_NUM_LL {
            // Determine the value to append: 0 if `origsize` is 0, else the existing count.
            let value = if origsize == 0 {
                0
            } else {
                // Ensure safe indexing; assuming `store.ll_counts` has enough elements.
                store.ll_counts[origsize - ZOPFLI_NUM_LL + i]
            };
            // Append the value to `ll_counts` and update `llsize`.
            zopfli_append_data(value, &mut store.ll_counts, &mut llsize);
        }
    }

    // Every time the index wraps around for distances, append to `d_counts`.
    if origsize % ZOPFLI_NUM_D == 0 {
        let mut dsize = origsize;
        for i in 0..ZOPFLI_NUM_D {
            // Determine the value to append: 0 if `origsize` is 0, else the existing count.
            let value = if origsize == 0 {
                0
            } else {
                // Ensure safe indexing; assuming `store.d_counts` has enough elements.
                store.d_counts[origsize - ZOPFLI_NUM_D + i]
            };
            // Append the value to `d_counts` and update `dsize`.
            zopfli_append_data(value, &mut store.d_counts, &mut dsize);
        }
    }

    // Append the `length` to the `litlens` vector.
    zopfli_append_data(length, &mut store.litlens, &mut store.size);

    // Reset `store.size` to `origsize` before appending `dist`.
    store.size = origsize;
    // Append the `dist` to the `dists` vector.
    zopfli_append_data(dist, &mut store.dists, &mut store.size);

    // Reset `store.size` to `origsize` before appending `pos`.
    store.size = origsize;
    // Append the `pos` to the `pos` vector.
    zopfli_append_data(pos, &mut store.pos, &mut store.size);

    // Ensure that the `length` is within the valid range.
    assert!(length < 259, "Length must be less than 259");

    if dist == 0 {
        // When `dist` is 0, it's a literal. Update `ll_symbol` and `d_symbol` accordingly.
        
        // Reset `store.size` to `origsize` before appending to `ll_symbol`.
        store.size = origsize;
        // Append the `length` directly to `ll_symbol`.
        zopfli_append_data(length, &mut store.ll_symbol, &mut store.size);

        // Reset `store.size` to `origsize` before appending to `d_symbol`.
        store.size = origsize;
        // Append `0` to `d_symbol` to indicate no distance.
        zopfli_append_data(0, &mut store.d_symbol, &mut store.size);

        // Increment the histogram count for the specific `length` symbol.
        store.ll_counts[llstart + length as usize] += 1;
    } else {
        // When `dist` is not 0, it's a length/distance pair. Update symbols accordingly.
        
        // Reset `store.size` to `origsize` before appending to `ll_symbol`.
        store.size = origsize;
        // Get the symbol for the `length` and append it to `ll_symbol`.
        let ll_sym = ZopfliGetLengthSymbol(length as i32) as u16;
        zopfli_append_data(ll_sym, &mut store.ll_symbol, &mut store.size);

        // Reset `store.size` to `origsize` before appending to `d_symbol`.
        store.size = origsize;
        // Get the symbol for the `dist` and append it to `d_symbol`.
        let d_sym = zopfli_get_dist_symbol(dist as i32) as u16;
        zopfli_append_data(d_sym, &mut store.d_symbol, &mut store.size);

        // Increment the histogram counts for the specific `length` and `dist` symbols.
        store.ll_counts[llstart + ll_sym as usize] += 1;
        store.d_counts[dstart + d_sym as usize] += 1;
    }
}


/// Appends all LZ77 entries from the `store` to the `target` `ZopfliLZ77Store`.
///
/// # Arguments
///
/// * `store` - A reference to the source `ZopfliLZ77Store` containing the entries to append.
/// * `target` - A mutable reference to the target `ZopfliLZ77Store` where entries will be appended.
///
/// This function iterates over each entry in the `store`'s `litlens`, `dists`, and `pos` vectors
/// and appends them to the `target` store using the `zopfli_store_lit_len_dist` function.
/// It effectively merges the LZ77 data from `store` into `target`.
pub fn zopfli_append_lz77_store(
    store: &ZopfliLZ77Store,
    target: &mut ZopfliLZ77Store,
) {
    // Iterate over each index from 0 to the size of the store.
    for i in 0..store.size {
        // Retrieve the literal/length value at the current index.
        let length = store.litlens[i];
        
        // Retrieve the distance value at the current index.
        let dist = store.dists[i];
        
        // Retrieve the position in the original data where this LZ77 command begins.
        let pos = store.pos[i];
        
        // Append the (length, dist, pos) tuple to the target store.
        zopfli_store_lit_len_dist(length, dist, pos, target);
    }
}


/// Defines the number of points to recursively check for finding the minimum.
/// Good value: 9.
pub const NUM: i32 = 9;


/// Used to initialize costs for example
pub const ZOPFLI_LARGE_FLOAT: f64 = 1e30;


/// Finds the minimum of a function `f(i)` where `i` is of type `usize`,
/// `f(i)` returns a `f64`, and `i` is in the range `start` to `end` (excluding `end`).
///
/// Outputs the minimum value in `smallest` and returns the index `i` where the minimum occurs.
///
/// # Arguments
///
/// * `f` - A function or closure that takes an index `i` and a context `context`, and returns a `f64` value.
///
/// * `context` - A reference to any context data required by the function `f`.
///
/// * `start` - The starting index of the range (inclusive).
///
/// * `end` - The ending index of the range (exclusive).
///
/// * `smallest` - A mutable reference to a `f64` where the minimum value will be stored.
///
/// # Returns
///
/// The index `i` where the minimum value occurs.
///
/// # Notes
///
/// If the range is small (less than 1024), it performs a linear search.
/// Otherwise, it uses a recursive-like method by checking multiple points to find the minimum more efficiently.
///
pub fn find_minimum(
    f: &dyn Fn(usize, &dyn std::any::Any) -> f64, // The function to minimize
    context: &dyn std::any::Any,                  // Context for the function f
    mut start: usize,                             // Start of the range (inclusive)
    mut end: usize,                               // End of the range (exclusive)
    smallest: &mut f64,                           // Output: the smallest value found
) -> usize {                                      // Returns the index of the smallest value
    if end - start < 1024 {
        // For small ranges, perform a linear search
        let mut best = ZOPFLI_LARGE_FLOAT;
        let mut result = start;

        for i in start..end {
            let v = f(i, context);
            if v < best {
                best = v;
                result = i;
            }
        }
        *smallest = best;
        result
    } else {
        // For larger ranges, search more efficiently by recursively checking multiple points
        const NUM: usize = 9; // Good value: 9

        let mut p: [usize; NUM] = [0; NUM];    // Points to evaluate
        let mut vp: [f64; NUM] = [0.0; NUM];   // Function values at the points
        let mut besti: usize = 0;              // Index of the best point
        let mut best: f64;                     // Best value found
        let mut lastbest = ZOPFLI_LARGE_FLOAT; // Last best value
        let mut pos = start;                   // Position of the best value

        loop {
            if end - start <= NUM {
                break;
            }

            // Evaluate function at NUM points within the range
            for i in 0..NUM {
                p[i] = start + (i + 1) * ((end - start) / (NUM + 1));
                vp[i] = f(p[i], context);
            }

            // Find the best value among the points
            besti = 0;
            best = vp[0];
            for i in 1..NUM {
                if vp[i] < best {
                    best = vp[i];
                    besti = i;
                }
            }

            // If no improvement, break the loop
            if best > lastbest {
                break;
            }

            // Narrow the search range based on the best point found
            start = if besti == 0 { start } else { p[besti - 1] };
            end = if besti == NUM - 1 { end } else { p[besti + 1] };

            pos = p[besti];  // Update position of the best value
            lastbest = best; // Update the last best value
        }

        *smallest = lastbest;
        pos
    }
}


/// Inserts a value into the vector in sorted order.
/// 
/// # Arguments
/// 
/// * `value` - The value to insert.
/// * `out` - A mutable reference to the output vector.
/// * `outsize` - A mutable reference to the current size of the output vector.
///
/// # Behavior
///
/// This function appends the `value` to the `out` vector using the `zopfli_append_data` function.
/// If the appended value disrupts the sorted order, the function shifts elements to maintain
/// the sorted property by inserting the `value` into the correct position.
///
/// # Example
///
/// ```
/// let mut out = vec![1, 3, 5];
/// let mut outsize = 3;
/// add_sorted(4, &mut out, &mut outsize);
/// assert_eq!(out, vec![1, 3, 4, 5]);
/// assert_eq!(outsize, 4);
/// ```
pub fn add_sorted(value: usize, out: &mut Vec<usize>, outsize: &mut usize) {
    // Append the value to the vector, resizing if necessary.
    zopfli_append_data(value, out, outsize);
    
    // Iterate through the vector to find the correct position to insert the value.
    // The loop runs from index 0 to (*outsize - 2) because we've just incremented outsize.
    for i in 0..(*outsize - 1) {
        // If the current element is greater than the value to insert, we need to insert here.
        if out[i] > value {
            // Shift elements to the right to make space for the new value.
            // This loop starts from the last valid index and moves down to i + 1.
            for j in ((i + 1)..*outsize).rev() {
                // Assign the value from the previous index to the current index.
                out[j] = out[j - 1];
            }
            // Insert the new value at the correct position.
            out[i] = value;
            // Exit the loop after inserting the value.
            break;
        }
    }
}


use std::io::{self, Write};

/// Prints the block split points as decimal and hex values to the terminal.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing LZ77 data.
/// * `lz77splitpoints` - An optional slice of split point indices.
/// * `nlz77points` - The number of split points.
///
/// # Panics
///
/// Panics if the number of collected split points does not match `nlz77points`.
pub fn print_block_split_points(
    lz77: &ZopfliLZ77Store,
    lz77splitpoints: Option<&[usize]>,
    nlz77points: usize,
) {
    // Initialize an empty vector to store the split points.
    let mut splitpoints: Vec<usize> = Vec::new();
    // Initialize the number of points collected to zero.
    let mut npoints: usize = 0;
    // Initialize the position counter to zero.
    let mut pos: usize = 0;

    // Check if there are any split points to process.
    if nlz77points > 0 {
        // Ensure that `lz77splitpoints` is provided when `nlz77points` is greater than zero.
        let lz77splitpoints = lz77splitpoints.expect("lz77splitpoints is None, but nlz77points > 0");

        // Iterate over each symbol in the LZ77 store.
        for i in 0..lz77.size {
            // Determine the length based on the distance. If distance is 0, it's a literal with length 1.
            let length = if lz77.dists[i] == 0 {
                1
            } else {
                lz77.litlens[i] as usize
            };

            // Check if the current index `i` matches the next split point.
            if lz77splitpoints.get(npoints) == Some(&i) {
                // Append the current position to `splitpoints` using the helper function.
                zopfli_append_data(pos, &mut splitpoints, &mut npoints);
                // If we've collected all split points, exit the loop.
                if npoints == nlz77points {
                    break;
                }
            }

            // Increment the position by the length of the current symbol.
            pos += length;
        }
    }

    // Ensure that the number of collected split points matches the expected number.
    assert_eq!(npoints, nlz77points);

    // Create a buffered writer for stderr to improve performance.
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    // Print the block split points in decimal format.
    write!(handle, "block split points: ").expect("Failed to write to stderr");
    for i in 0..npoints {
        write!(handle, "{} ", splitpoints[i]).expect("Failed to write to stderr");
    }

    // Print the block split points in hexadecimal format.
    write!(handle, "(hex:").expect("Failed to write to stderr");
    for i in 0..npoints {
        write!(handle, " {:x}", splitpoints[i]).expect("Failed to write to stderr");
    }
    writeln!(handle, ")").expect("Failed to write to stderr");
}


/// Finds the next block to try to split, selecting the largest available one.
/// 
/// The largest block is chosen to ensure that if only a limited number of blocks
/// are requested, their sizes are spread evenly.
///
/// # Arguments
///
/// * `lz77size` - The size of the LL77 data, which is the size of the `done` array.
/// * `done` - A slice indicating which blocks starting at that position are no longer
///            splittable (splitting them increases rather than decreases cost).
/// * `splitpoints` - A slice of splitpoints found so far. Can be empty if no splitpoints are present.
/// * `lstart` - Output parameter that will hold the start of the selected block.
/// * `lend` - Output parameter that will hold the end of the selected block.
///
/// # Returns
///
/// * `true` if a splittable block was found and `lstart` and `lend` are set.
/// * `false` if no splittable block was found (all blocks are done).
pub fn find_largest_splittable_block(
    lz77size: usize,
    done: &[u8],
    splitpoints: &[usize],
    lstart: &mut usize,
    lend: &mut usize,
) -> bool {
    // Initialize the length of the longest splittable block found to zero.
    let mut longest = 0;
    // Flag to indicate whether a splittable block has been found.
    let mut found = false;
    // Determine the number of splitpoints.
    let npoints = splitpoints.len();

    // Iterate over each possible block defined by splitpoints.
    // The loop runs from 0 to npoints inclusive to handle the last block.
    for i in 0..=npoints {
        // Determine the start of the current block.
        // If it's the first block, start at 0. Otherwise, start at the previous splitpoint.
        let start = if i == 0 { 
            0 
        } else { 
            splitpoints[i - 1] 
        };

        // Determine the end of the current block.
        // If it's the last block, end at lz77size - 1. Otherwise, end at the current splitpoint.
        let end = if i == npoints { 
            lz77size - 1 
        } else { 
            splitpoints[i] 
        };

        // Check if the current block is splittable and larger than any previously found block.
        if done[start] == 0 && (end - start) > longest {
            // Update the start and end of the largest splittable block found.
            *lstart = start;
            *lend = end;
            // Set the found flag to true.
            found = true;
            // Update the longest block size.
            longest = end - start;
        }
    }

    // Return whether a splittable block was found.
    found
}


use std::f64::MAX;

/// Performs block splitting on LZ77 data.
///
/// This function analyzes the provided LZ77 data and determines optimal split points to divide the data 
/// into multiple blocks, enhancing compression efficiency. The split points are indices within the 
/// LZ77 data where the blocks should be divided. The number of blocks can be limited using the `maxblocks` parameter.
///
/// # Arguments
///
/// * `options` - A reference to `ZopfliOptions` containing various configuration options.
/// * `lz77` - A reference to `ZopfliLZ77Store` containing the LZ77 encoded data.
/// * `maxblocks` - The maximum number of blocks to split into. A value of `0` implies no limit.
/// * `splitpoints` - A mutable reference to a `Vec<usize>` where the resulting split points will be stored.
/// * `npoints` - A mutable reference to a `usize` that will hold the number of split points found.
///
/// # Behavior
///
/// The function initializes a `done` array to keep track of which blocks should no longer be split. It then 
/// iteratively finds the optimal points to split the LZ77 data based on estimated compression costs, 
/// ensuring that the number of blocks does not exceed `maxblocks`. If the `verbose` option is enabled, 
/// the function will print the split points in both decimal and hexadecimal formats.
///
/// # Panics
///
/// Panics if memory allocation for the `done` array fails or if the number of collected split points 
/// does not match `nlz77points` when `maxblocks` is greater than 0.
///
/// # Example
///
/// ```
/// let options = ZopfliOptions {
///     verbose: true,
///     verbose_more: false,
///     numiterations: 10,
///     blocksplitting: 1,
///     blocksplittinglast: 0,
///     blocksplittingmax: 15,
/// };
/// let lz77 = ZopfliLZ77Store {
///     litlens: vec![...],
///     dists: vec![...],
///     size: 100,
///     data: vec![...],
///     pos: vec![...],
///     ll_symbol: vec![...],
///     d_symbol: vec![...],
///     ll_counts: vec![...],
///     d_counts: vec![...],
/// };
/// let maxblocks = 10;
/// let mut splitpoints = Vec::new();
/// let mut npoints = 0;
/// 
/// zopfli_block_split_lz77(&options, &lz77, maxblocks, &mut splitpoints, &mut npoints);
/// ```
pub fn zopfli_block_split_lz77(
    options: &ZopfliOptions,
    lz77: &ZopfliLZ77Store,
    maxblocks: usize,
    splitpoints: &mut Vec<usize>,
    npoints: &mut usize,
) {
    // Check if the LZ77 data is too small to split
    if lz77.size < 10 {
        return; // This code fails on tiny files.
    }

    // Initialize the `done` array with zeros, indicating no blocks are marked as done
    let mut done: Vec<u8> = vec![0u8; lz77.size];
    // Initialize start and end positions
    let mut lstart: usize = 0;
    let mut lend: usize = lz77.size;
    // Initialize the number of blocks to 1
    let mut numblocks: usize = 1;

    loop {
        // If maxblocks is set and we've reached the limit, stop splitting
        if maxblocks > 0 && numblocks >= maxblocks {
            break;
        }

        // Create the SplitCostContext with current start and end
        let context = SplitCostContext {
            lz77: lz77.clone(),
            start: lstart,
            end: lend,
        };

        // Find the minimum cost split position between lstart + 1 and lend
        let split_cost = {
            let mut smallest_cost: f64 = 0.0;
            let split_pos = find_minimum(
                &|i: usize, ctx: &dyn std::any::Any| -> f64 {
                    // Downcast the context to SplitCostContext
                    let ctx = ctx.downcast_ref::<SplitCostContext>().expect("Invalid context type");
                    split_cost(i, ctx)
                },
                &context as &dyn std::any::Any,
                lstart + 1,
                lend,
                &mut smallest_cost,
            );
            (split_pos, smallest_cost)
        };

        let llpos = split_cost.0;
        let splitcost = split_cost.1;

        // Estimate the original cost without splitting
        let origcost = estimate_cost(lz77, lstart, lend);

        // Check if splitting reduces the cost and the split position is valid
        if splitcost > origcost || llpos == lstart + 1 || llpos == lend {
            done[lstart] = 1; // Mark this block as done
        } else {
            // Add the split position in sorted order
            add_sorted(llpos, splitpoints, npoints);
            numblocks += 1; // Increment the number of blocks
        }

        // Find the largest splittable block for the next iteration
        let mut new_lstart: usize = 0;
        let mut new_lend: usize = 0;
        if !find_largest_splittable_block(
            lz77.size,
            &done,
            &splitpoints[..*npoints],
            &mut new_lstart,
            &mut new_lend,
        ) {
            break; // No further split will probably reduce compression.
        }

        lstart = new_lstart;
        lend = new_lend;

        // If the block is too small to split further, stop
        if lend - lstart < 10 {
            break;
        }
    }

    // If verbose option is enabled, print the split points
    if options.verbose {
        print_block_split_points(lz77, Some(&splitpoints), *npoints);
    }

    // The `done` vector is automatically freed when it goes out of scope
}


/// Gets a score of the length given the distance. Typically, the score of the
/// length is the length itself, but if the distance is very long, decrease the
/// score of the length a bit to make up for the fact that long distances use large
/// amounts of extra bits.
///
/// This is not an accurate score, it is a heuristic only for the greedy LZ77
/// implementation. More accurate cost models are employed later. Making this
/// heuristic more accurate may hurt rather than improve compression.
///
/// The two direct uses of this heuristic are:
/// - avoid using a length of 3 in combination with a long distance. This only has
///   an effect if `length == 3`.
/// - make a slightly better choice between the two options of the lazy matching.
///
/// Indirectly, this affects:
/// - the block split points if the default of block splitting first is used, in a
///   rather unpredictable way
/// - the first zopfli run, so it affects the chance of the first run being closer
///   to the optimal output
pub fn get_length_score(length: i32, distance: i32) -> i32 {
    // At 1024, the distance uses 9+ extra bits and this seems to be the sweet spot
    // on tested files.
    if distance > 1024 {
        length - 1
    } else {
        length
    }
}


/// Resets all fields of `ZopfliHash`.
///
/// # Arguments
///
/// * `window_size` - The size of the window to initialize the hash.
/// * `h` - A mutable reference to the `ZopfliHash` structure to be reset.
pub fn zopfli_reset_hash(window_size: usize, h: &mut ZopfliHash) {
    // Reset the current hash value.
    h.val = 0;
    
    // Initialize all entries of `head` to -1, indicating no head so far.
    for i in 0..65536 {
        h.head[i] = -1;
    }

    // Initialize `prev` and `hashval` for the first hash.
    for i in 0..window_size {
        h.prev[i] = i as u16; // If prev[j] == j, then prev[j] is uninitialized.
        h.hashval[i] = -1;
    }

    // Initialize `same` to 0, representing no repetitions of the same byte.
    for i in 0..window_size {
        h.same[i] = 0;
    }

    // Reset the second hash value.
    h.val2 = 0;
    
    // Initialize all entries of `head2` to -1, indicating no head so far.
    for i in 0..65536 {
        h.head2[i] = -1;
    }

    // Initialize `prev2` and `hashval2` for the second hash.
    for i in 0..window_size {
        h.prev2[i] = i as u16; // If prev2[j] == j, then prev2[j] is uninitialized.
        h.hashval2[i] = -1;
    }
}


// Define HASH_SHIFT constant equivalent to the C `#define HASH_SHIFT 5`
pub const HASH_SHIFT: usize = 5;


/// HASH_MASK constant equivalent to C's `#define HASH_MASK 32767`
pub const HASH_MASK: u16 = 32767;


/// Updates the sliding hash value with the given byte. All calls to this function
/// must be made on consecutive input characters. Since the hash value exists out
/// of multiple input bytes, a few warmups with this function are needed initially.
pub fn update_hash_value(h: &mut ZopfliHash, c: u8) {
    // Shift the current hash value left by HASH_SHIFT bits
    // XOR it with the new byte `c`
    // Apply the HASH_MASK to ensure the hash value stays within bounds
    h.val = ((h.val << (HASH_SHIFT as u32)) ^ (c as i32)) & (HASH_MASK as i32);
}


/// The window size for deflate. Must be a power of two. This should be 32768, the
/// maximum possible by the deflate spec. Anything less hurts compression more than
/// speed.
pub const ZOPFLI_WINDOW_SIZE: usize = 32768;


/// The window mask used to wrap indices into the window. This is why the
/// window size must be a power of two.
pub const ZOPFLI_WINDOW_MASK: usize = ZOPFLI_WINDOW_SIZE - 1;


/// Defines the minimum match length for Zopfli
pub const ZOPFLI_MIN_MATCH: usize = 3;


/// Updates the hash values based on the current position in the array. All calls
/// to this must be made for consecutive bytes.
pub fn zopfli_update_hash(array: &[u8], pos: usize, end: usize, h: &mut ZopfliHash) {
    // Calculate the position in the window using the window mask
    let hpos = (pos & ZOPFLI_WINDOW_MASK) as u16;
    // Initialize the amount of repetitions
    let mut amount: usize = 0;

    // Update the hash value with the next byte, or 0 if out of bounds
    let c = if pos + ZOPFLI_MIN_MATCH <= end {
        array[pos + ZOPFLI_MIN_MATCH - 1]
    } else {
        0
    };
    update_hash_value(h, c);

    // Update hashval at position hpos with the current hash value
    h.hashval[hpos as usize] = h.val;

    // Update the 'prev' and 'head' arrays for the first hash
    let h_head_val = h.head[h.val as usize];
    if h_head_val != -1
        && h_head_val >= 0
        && (h_head_val as usize) < h.hashval.len()
        && h.hashval[h_head_val as usize] == h.val
    {
        // Set prev[hpos] to the previous position with the same hash value
        h.prev[hpos as usize] = h_head_val as u16;
    } else {
        // If no previous position, set prev[hpos] to hpos itself
        h.prev[hpos as usize] = hpos;
    }
    // Update head array to point to the current position
    h.head[h.val as usize] = hpos as i32;

    // Update the 'same' array to count repetitions of the same byte
    // Calculate previous hpos, wrapping around the window size
    let prev_hpos = (pos.wrapping_sub(1)) & ZOPFLI_WINDOW_MASK;
    if h.same[prev_hpos as usize] > 1 {
        // Decrease the amount by one from the previous position
        amount = (h.same[prev_hpos as usize] - 1) as usize;
    }
    // Count the number of same consecutive bytes starting from pos
    while pos + amount + 1 < end
        && array[pos] == array[pos + amount + 1]
        && amount < u16::MAX as usize
    {
        amount += 1;
    }
    // Update the 'same' array at position hpos with the amount
    h.same[hpos as usize] = amount as u16;

    // Update the second hash value based on the amount of repetitions
    h.val2 = (((h.same[hpos as usize] as i32 - ZOPFLI_MIN_MATCH as i32) & 255) ^ h.val) as i32;
    // Update hashval2 at position hpos with the current second hash value
    h.hashval2[hpos as usize] = h.val2;

    // Update the 'prev2' and 'head2' arrays for the second hash
    let h_head2_val2 = h.head2[h.val2 as usize];
    if h_head2_val2 != -1
        && h_head2_val2 >= 0
        && (h_head2_val2 as usize) < h.hashval2.len()
        && h.hashval2[h_head2_val2 as usize] == h.val2
    {
        // Set prev2[hpos] to the previous position with the same second hash value
        h.prev2[hpos as usize] = h_head2_val2 as u16;
    } else {
        // If no previous position, set prev2[hpos] to hpos itself
        h.prev2[hpos as usize] = hpos;
    }
    // Update head2 array to point to the current position
    h.head2[h.val2 as usize] = hpos as i32;
}


/// Prepopulates the hash:
/// Fills in the initial values in the hash, before `zopfli_update_hash` can be used correctly.
///
/// # Arguments
///
/// * `array` - A slice of unsigned bytes representing the input data.
/// * `pos` - The starting position in the array to begin warming up the hash.
/// * `end` - The end position in the array to ensure we do not go out of bounds.
/// * `h` - A mutable reference to the `ZopfliHash` structure to be updated.
///
/// # Safety
///
/// This function assumes that `pos` is less than `end` and that `array` has enough elements.
/// It does not perform bounds checking beyond the second element.
pub fn zopfli_warmup_hash(array: &[u8], pos: usize, end: usize, h: &mut ZopfliHash) {
    // Update the hash value with the byte at the current position.
    update_hash_value(h, array[pos]);

    // If there is at least one more byte, update the hash value with the next byte.
    if pos + 1 < end {
        update_hash_value(h, array[pos + 1]);
    }
}


/// Verifies if `length` and `dist` are indeed valid, only used for assertion.
///
/// This function checks that the specified `length` does not exceed the
/// bounds of the `data` array and that the sequence of bytes at the
/// current position matches the sequence of bytes at the distance `dist`
/// behind the current position.
///
/// # Arguments
///
/// * `data` - A slice of bytes representing the data to be verified.
/// * `datasize` - The total size of the `data` slice.
/// * `pos` - The current position in the `data` slice.
/// * `dist` - The distance behind the current position to compare bytes.
/// * `length` - The number of bytes to verify.
///
/// # Panics
///
/// This function will panic if:
/// - `pos + length` exceeds `datasize`.
/// - Any byte in the range `[pos, pos + length)` does not match the corresponding
///   byte at `pos - dist + i`.
pub fn zopfli_verify_len_dist(
    data: &[u8],
    datasize: usize,
    pos: usize,
    dist: u16,
    length: u16,
) {
    // Convert `dist` and `length` from `u16` to `usize` for arithmetic operations.
    let dist = dist as usize;
    let length = length as usize;

    // Assert that `pos + length` does not exceed `datasize`.
    debug_assert!(
        pos + length <= datasize,
        "pos ({}) + length ({}) exceeds datasize ({})",
        pos,
        length,
        datasize
    );

    // Iterate through each byte in the specified range.
    for i in 0..length {
        // Calculate the indices for comparison.
        let current_index = pos + i;
        let compare_index = pos - dist + i;

        // Compare the bytes at the current position and the position `dist` behind.
        if data[compare_index] != data[current_index] {
            // Assert that the bytes must match. If they don't, panic with an error message.
            debug_assert_eq!(
                data[compare_index],
                data[current_index],
                "Data mismatch at position {}: {} != {}",
                current_index,
                data[compare_index],
                data[current_index]
            );
            // Break out of the loop after the first mismatch.
            break;
        }
    }
}


/// Finds how long the match of `scan` and `match_` is.
/// Can be used to find how many bytes starting from `scan`, and from `match_`, are equal.
/// Returns the last index after `scan` which is still equal to the corresponding index after `match_`.
///
/// - `scan` is the slice to compare starting from.
/// - `match_` is the earlier slice to compare starting from.
/// - `end` is the last possible index in `scan`, beyond which to stop looking.
/// - `safe_end` is a few (8) indices before `end`, for comparing multiple bytes at once.
pub fn get_match(
    scan: &[u8],
    match_: &[u8],
    end: usize,
    safe_end: usize,
) -> usize {
    // Initialize indices for scan and match slices
    let mut scan_idx = 0;
    let mut match_idx = 0;

    // Check the size of usize to determine the block size for comparison
    if std::mem::size_of::<usize>() == 8 {
        // 8-byte (64-bit) comparison
        while scan_idx < safe_end {
            // Ensure there are enough bytes left to compare
            if scan_idx + 8 > scan.len() || match_idx + 8 > match_.len() {
                break;
            }
            // Compare 8 bytes at a time
            if &scan[scan_idx..scan_idx + 8] != &match_[match_idx..match_idx + 8] {
                break;
            }
            scan_idx += 8;
            match_idx += 8;
        }
    } else if std::mem::size_of::<u32>() == 4 {
        // 4-byte (32-bit) comparison
        while scan_idx < safe_end {
            // Ensure there are enough bytes left to compare
            if scan_idx + 4 > scan.len() || match_idx + 4 > match_.len() {
                break;
            }
            // Safely convert 4-byte slices to u32 for comparison
            let scan_val = u32::from_le_bytes([
                scan[scan_idx],
                scan[scan_idx + 1],
                scan[scan_idx + 2],
                scan[scan_idx + 3],
            ]);
            let match_val = u32::from_le_bytes([
                match_[match_idx],
                match_[match_idx + 1],
                match_[match_idx + 2],
                match_[match_idx + 3],
            ]);
            // Break if the 4-byte blocks do not match
            if scan_val != match_val {
                break;
            }
            scan_idx += 4;
            match_idx += 4;
        }
    } else {
        // Fallback to byte-by-byte comparison for systems with different sizes
        while scan_idx < safe_end {
            // Ensure there are enough bytes left to compare
            if scan_idx + 8 > scan.len() || match_idx + 8 > match_.len() {
                break;
            }
            // Compare 8 individual bytes
            if scan[scan_idx] != match_[match_idx]
                || scan[scan_idx + 1] != match_[match_idx + 1]
                || scan[scan_idx + 2] != match_[match_idx + 2]
                || scan[scan_idx + 3] != match_[match_idx + 3]
                || scan[scan_idx + 4] != match_[match_idx + 4]
                || scan[scan_idx + 5] != match_[match_idx + 5]
                || scan[scan_idx + 6] != match_[match_idx + 6]
                || scan[scan_idx + 7] != match_[match_idx + 7]
            {
                break;
            }
            scan_idx += 8;
            match_idx += 8;
        }
    }

    // Compare the remaining bytes one by one
    while scan_idx < end
        && scan_idx < scan.len()
        && match_idx < match_.len()
        && scan[scan_idx] == match_[match_idx]
    {
        scan_idx += 1;
        match_idx += 1;
    }

    // Return the index in `scan` where the match ends
    scan_idx
}


/// For longest match cache. max 256. Uses huge amounts of memory but makes it
/// faster. Uses this many times three bytes per single byte of the input data.
/// This is so because longest match finding has to find the exact distance
/// that belongs to each length for the best lz77 strategy.
/// Good values: e.g. 5, 8.
pub const ZOPFLI_CACHE_LENGTH: usize = 8;


/// Returns the length up to which could be stored in the cache.
pub fn zopfli_max_cached_sublen(
    lmc: &ZopfliLongestMatchCache,
    pos: usize,
    length: usize,
) -> u32 {
    // Calculate the starting index for the cache in the `sublen` vector.
    let cache_start = ZOPFLI_CACHE_LENGTH * pos * 3;
    
    // Obtain a slice of the `sublen` vector starting from `cache_start`.
    let cache = &lmc.sublen[cache_start..];
    
    // The `length` parameter is unused in this function, mirroring the C code's `(void)length;`.
    // This is kept to maintain the function signature consistency.
    let _ = length;
    
    // Check if both the second and third bytes in the current cache segment are zero.
    // This indicates that no sublen has been cached for this position.
    if cache[1] == 0 && cache[2] == 0 {
        return 0; // No sublen cached.
    }
    
    // Retrieve the sublen value from the last triplet in the cache segment.
    // `(ZOPFLI_CACHE_LENGTH - 1) * 3` calculates the starting index of the last triplet.
    // Add 3 to the retrieved value as per the C code logic.
    (cache[(ZOPFLI_CACHE_LENGTH - 1) * 3] as u32) + 3
}


/// Converts the cached longest match information into the `sublen` array.
/// 
/// This function processes the cached longest match data from the `ZopfliLongestMatchCache`
/// and populates the `sublen` array with the corresponding distance values for each
/// possible sublength up to the maximum cached sublength.
/// 
/// # Arguments
/// 
/// * `lmc` - A reference to the `ZopfliLongestMatchCache` containing cached match data.
/// * `pos` - The current position in the input data for which to retrieve cached matches.
/// * `length` - The total length of the input data. This parameter is unused in the current implementation.
/// * `sublen` - A mutable slice of `u16` where the distance values will be stored.
///
/// # Behavior
/// 
/// - If the provided `length` is less than 3, the function returns early as no caching is needed.
/// - It retrieves the relevant cache segment from the `sublen` vector based on the `pos`.
/// - It determines the maximum cached sublength using `zopfli_max_cached_sublen`.
/// - Iterates through each cached entry up to `ZOPFLI_CACHE_LENGTH`, extracting the
///   length and distance values.
/// - For each length, it updates the `sublen` array with the corresponding distance
///   for all sublengths from the previous length up to the current length.
/// - The iteration stops early if the current length reaches the maximum cached sublength.
pub fn zopfli_cache_to_sublen(
    lmc: &ZopfliLongestMatchCache,
    pos: usize,
    length: usize,
    sublen: &mut [u16],
) {
    // If the total length is less than 3, no caching is necessary.
    if length < 3 {
        return;
    }

    // Calculate the starting index for the cache in the `sublen` vector.
    let cache_start = ZOPFLI_CACHE_LENGTH * pos * 3;

    // Obtain a slice of the `sublen` vector starting from `cache_start`.
    // It's assumed that the slice has at least `ZOPFLI_CACHE_LENGTH * 3` elements.
    let cache = &lmc.sublen[cache_start..cache_start + ZOPFLI_CACHE_LENGTH * 3];

    // Retrieve the maximum cached sublength for the current position.
    let maxlength = zopfli_max_cached_sublen(lmc, pos, length) as usize;

    // Initialize the previous length to zero.
    let mut prevlength: usize = 0;

    // Iterate over each cached entry up to `ZOPFLI_CACHE_LENGTH`.
    for j in 0..ZOPFLI_CACHE_LENGTH {
        // Extract the length from the current cache entry and add 3 as per the C logic.
        let current_length = (cache[j * 3] as usize) + 3;

        // Compute the distance by combining the low and high bytes.
        let dist = (cache[j * 3 + 1] as u16) | ((cache[j * 3 + 2] as u16) << 8);

        // Update the `sublen` array for all sublengths from `prevlength` to `current_length`.
        for i in prevlength..=current_length {
            // Ensure we do not exceed the bounds of the `sublen` slice.
            if i < sublen.len() {
                sublen[i] = dist;
            }
        }

        // If the current length reaches the maximum cached sublength, exit the loop early.
        if current_length == maxlength {
            break;
        }

        // Update `prevlength` for the next iteration.
        prevlength = current_length + 1;
    }
}


// Public constant representing the maximum match length that can be encoded in deflate.
pub const ZOPFLI_MAX_MATCH: usize = 258;


/// Tries to get distance, length, and sublen values from the cache if possible.
///
/// Returns `1` if it successfully retrieved the values from the cache, otherwise returns `0`.
/// Additionally, this function may update the `limit` value to a smaller one if possible
/// with more limited information from the cache.
///
/// # Arguments
///
/// * `s` - Reference to the block state containing the options and the match cache.
/// * `pos` - The current position in the block.
/// * `limit` - A mutable reference to the limit value, which may be updated.
/// * `sublen` - Optionally, a mutable slice to store sublen values. This parameter is mutable to allow modifications.
/// * `distance` - A mutable reference to store the distance value.
/// * `length` - A mutable reference to store the length value.
///
/// # Returns
///
/// Returns `1` if values were retrieved from the cache, otherwise returns `0`.
pub fn try_get_from_longest_match_cache(
    s: &ZopfliBlockState,
    pos: usize,
    limit: &mut usize,
    mut sublen: Option<&mut [u16]>, // Made mutable to allow mutable borrowing inside
    distance: &mut u16,
    length: &mut u16,
) -> i32 {
    // The LMC cache starts at the beginning of the block rather than the beginning of the whole array.
    let lmcpos = pos - s.blockstart;

    // Check if the longest match cache is available.
    if let Some(ref lmc) = s.lmc {
        // Check if cache value is available at the current position.
        // Length > 0 and dist == 0 is an invalid combination, which indicates
        // on purpose that this cache value is not filled in yet.
        let cache_available = lmc.length[lmcpos] == 0 || lmc.dist[lmcpos] != 0;

        if cache_available {
            // Determine if the limit is acceptable for using the cache.
            let limit_ok_for_cache = (*limit == ZOPFLI_MAX_MATCH)
                || (lmc.length[lmcpos] as usize <= *limit)
                || (sublen.is_some()
                    && zopfli_max_cached_sublen(
                        lmc,
                        lmcpos,
                        lmc.length[lmcpos] as usize,
                    ) as usize
                        >= *limit);

            if limit_ok_for_cache {
                // Check if sublen is not provided or we can use the cached sublen values.
                if sublen.is_none()
                    || (lmc.length[lmcpos] as u32)
                        <= zopfli_max_cached_sublen(lmc, lmcpos, lmc.length[lmcpos] as usize)
                {
                    // Set the length from the cache.
                    *length = lmc.length[lmcpos];
                    // Ensure that the length does not exceed the limit.
                    if (*length as usize) > *limit {
                        *length = *limit as u16;
                    }

                    if let Some(ref mut sublen_ref) = sublen {
                        // Convert the cache to sublen values.
                        zopfli_cache_to_sublen(lmc, lmcpos, *length as usize, sublen_ref);
                        // Set the distance from the sublen array.
                        *distance = sublen_ref[*length as usize];

                        // If limit is the maximum and length is at least the minimum match length,
                        // assert that the sublen distance matches the cached distance.
                        if *limit == ZOPFLI_MAX_MATCH && (*length as usize) >= ZOPFLI_MIN_MATCH {
                            assert!(sublen_ref[*length as usize] == lmc.dist[lmcpos]);
                        }
                    } else {
                        // Set the distance directly from the cache.
                        *distance = lmc.dist[lmcpos];
                    }
                    // Return 1 to indicate that values were retrieved from the cache.
                    return 1;
                }
                // Can't use much of the cache, since the "sublens" need to be calculated,
                // but at least we already know when to stop.
                *limit = lmc.length[lmcpos] as usize;
            }
        }
    }
    // Return 0 to indicate that values were not retrieved from the cache.
    0
}


/// Stores the `sublen` array in the cache.
///
/// This function updates the `sublen` cache within the `ZopfliLongestMatchCache` structure
/// based on the provided `sublen` array, position, and length. It processes the `sublen` data
/// and stores relevant entries in the cache to optimize future match finding.
///
/// # Arguments
///
/// * `sublen` - A slice of `u16` representing the sublen array.
/// * `pos` - The current position in the input data.
/// * `length` - The length of the current match.
/// * `lmc` - A mutable reference to the `ZopfliLongestMatchCache` where the cache will be stored.
pub fn zopfli_sublen_to_cache(
    sublen: &[u16],
    pos: usize,
    length: usize,
    lmc: &mut ZopfliLongestMatchCache,
) {
    // Initialize the index `j` for the cache and `best_length` to track the best length found.
    let mut j: usize = 0;
    let mut best_length: usize = 0;

    // Calculate the starting index for the cache in the `sublen` vector.
    let cache_start = ZOPFLI_CACHE_LENGTH * pos * 3;

    // Obtain a mutable slice of the `sublen` vector starting from `cache_start`.
    // This slice will hold up to `ZOPFLI_CACHE_LENGTH * 3` bytes.
    let cache = &mut lmc.sublen[cache_start..cache_start + ZOPFLI_CACHE_LENGTH * 3];

    // If the length is less than 3, there is nothing to cache, so return early.
    if length < 3 {
        return;
    }

    // Iterate from `i = 3` up to and including `length`.
    for i in 3..=length {
        // Check if `i` is at the end of the range or if the current `sublen` differs from the next one.
        if i == length || sublen[i] != sublen[i + 1] {
            // Store `i - 3` as the first byte of the current cache entry.
            cache[j * 3] = (i - 3) as u8;

            // Store the lower byte of `sublen[i]` as the second byte of the cache entry.
            cache[j * 3 + 1] = (sublen[i] % 256) as u8;

            // Store the higher byte of `sublen[i]` as the third byte of the cache entry.
            cache[j * 3 + 2] = ((sublen[i] >> 8) % 256) as u8;

            // Update `best_length` to the current `i`.
            best_length = i;

            // Increment the cache index `j`.
            j += 1;

            // If the cache is full, break out of the loop.
            if j >= ZOPFLI_CACHE_LENGTH {
                break;
            }
        }
    }

    // After processing, check if the cache was not fully populated.
    if j < ZOPFLI_CACHE_LENGTH {
        // Ensure that the best length matches the total length.
        assert!(best_length == length, "best_length ({}) does not match length ({})", best_length, length);

        // Update the last entry's first byte with `best_length - 3`.
        cache[(ZOPFLI_CACHE_LENGTH - 1) * 3] = (best_length - 3) as u8;
    } else {
        // If the cache is full, ensure that `best_length` does not exceed the total length.
        assert!(best_length <= length, "best_length ({}) exceeds length ({})", best_length, length);
    }

    // Finally, verify that `best_length` matches the maximum cached sublen.
    assert!(
        best_length as u32 == zopfli_max_cached_sublen(lmc, pos, length),
        "best_length ({}) does not match ZopfliMaxCachedSublen ({})",
        best_length,
        zopfli_max_cached_sublen(lmc, pos, length)
    );
}


/// Stores the found `sublen`, `distance`, and `length` in the longest match cache, if possible.
///
/// This function updates the `ZopfliLongestMatchCache` within the `ZopfliBlockState` based on the provided
/// `sublen` data, position, limit, distance, and length. It ensures that the cache is properly
/// updated to optimize future match finding.
///
/// # Arguments
///
/// * `s` - A mutable reference to the `ZopfliBlockState` containing the cache and block information.
/// * `pos` - The current position in the input data.
/// * `limit` - The maximum limit for the match length.
/// * `sublen` - An optional slice of `u16` representing the sublen array.
/// * `distance` - The distance value to store in the cache.
/// * `length` - The length value to store in the cache.
pub fn store_in_longest_match_cache(
    s: &mut ZopfliBlockState,
    pos: usize,
    limit: usize,
    sublen: Option<&[u16]>,
    distance: u16,
    length: u16,
) {
    // Ensure that the cache exists and that we're within the block boundaries.
    if let Some(ref mut lmc) = s.lmc {
        // Calculate the position relative to the block start.
        let lmc_pos = pos - s.blockstart;

        // Check if the current cache entry is available.
        let cache_available = lmc.length.get(lmc_pos).map_or(false, |&len| {
            len != 0 || lmc.dist.get(lmc_pos).unwrap_or(&0) != &0
        });

        // Proceed only if the cache is available, limit is maximum match,
        // `sublen` is provided, and the cache entry is not already filled.
        if limit == ZOPFLI_MAX_MATCH
            && sublen.is_some()
            && !cache_available
        {
            // Ensure the current cache entry is uninitialized.
            assert!(
                lmc.length.get(lmc_pos).map_or(true, |&len| len == 1),
                "Expected length to be 1 when cache is available."
            );
            assert!(
                lmc.dist.get(lmc_pos).map_or(true, |&d| d == 0),
                "Expected distance to be 0 when cache is available."
            );

            // Update the distance and length in the cache.
            if let Some(sublen_slice) = sublen {
                let new_distance = if length < ZOPFLI_MIN_MATCH as u16 {
                    0
                } else {
                    distance
                };
                let new_length = if length < ZOPFLI_MIN_MATCH as u16 {
                    0
                } else {
                    length
                };

                if lmc.dist.len() > lmc_pos {
                    lmc.dist[lmc_pos] = new_distance;
                }
                if lmc.length.len() > lmc_pos {
                    lmc.length[lmc_pos] = new_length;
                }

                // Ensure that the cache entry is now valid.
                assert!(
                    !(lmc.length[lmc_pos] == 1 && lmc.dist[lmc_pos] == 0),
                    "Cache entry should not be (1, 0) after update."
                );

                // Convert `sublen` to cache entries.
                zopfli_sublen_to_cache(sublen_slice, lmc_pos, length as usize, lmc);
            }
        }
    }
}


/// Limits the maximum number of hash chain hits for a given hash value. This setting
/// affects only files where the same hash value occurs very frequently. On such files,
/// this may result in worse compression (the ideal value is 32,768, which corresponds
/// to `ZOPFLI_WINDOW_SIZE`, while zlib uses 4,096 even at the best compression level).
/// However, it can make compression faster for specific files.
/// 
/// Recommended value: 8,192.
pub const ZOPFLI_MAX_CHAIN_HITS: usize = 8192;


/// Finds the longest match (length and corresponding distance) for LZ77
/// compression.
/// Even when not using `sublen`, it can be more efficient to provide an array,
/// because only then the caching is used.
///
/// # Arguments
///
/// * `s` - The block state containing options and cache.
/// * `h` - The hash structure containing recent occurrences.
/// * `array` - The data to search within.
/// * `pos` - The current position in the data to find the match for.
/// * `size` - The size of the data.
/// * `limit` - The maximum length limit. Must be in the range [ZOPFLI_MIN_MATCH, ZOPFLI_MAX_MATCH].
/// * `sublen` - Optionally, a mutable slice of 259 elements (or more). For each length >=3, it contains the smallest distance required to reach this length.
/// * `distance` - A mutable reference to store the distance value.
/// * `length` - A mutable reference to store the length value.
///
/// Note: The `sublen` argument is optional and can be `None`. Please handle it appropriately in the code.
pub fn zopfli_find_longest_match(
    s: &mut ZopfliBlockState,
    h: &ZopfliHash,
    array: &[u8],
    pos: usize,
    size: usize,
    mut limit: usize,
    mut sublen: Option<&mut [u16]>,
    distance: &mut u16,
    length: &mut u16,
) {
    // Calculate the current hash position using the window mask.
    let hpos = pos & ZOPFLI_WINDOW_MASK;

    // Initialize variables.
    let mut pp: u16;
    let mut p: u16;
    let mut bestdist: u16 = 0;
    let mut bestlength: u16 = 1;
    let mut scan_idx: usize;
    let mut match_idx: usize;
    let mut chain_counter: i32 = ZOPFLI_MAX_CHAIN_HITS as i32; // For quitting early.

    let mut dist: usize = 0; // Not u16 on purpose to prevent overflow.

    // Create references to the hash tables for easier access.
    let mut hhead = &h.head;
    let mut hprev = &h.prev;
    let mut hhashval = &h.hashval;
    let mut hval = h.val;

    // Attempt to retrieve the distance and length from the cache.
    if try_get_from_longest_match_cache(
        s,
        pos,
        &mut limit,
        sublen.as_deref_mut(),
        distance,
        length,
    ) == 1
    {
        // Ensure that the position plus the found length does not exceed the data size.
        assert!((pos + (*length as usize)) <= size);
        return;
    }

    // Ensure that the limit is within the acceptable range.
    assert!(limit <= ZOPFLI_MAX_MATCH);
    assert!(limit >= ZOPFLI_MIN_MATCH);
    assert!(pos < size);

    // If the remaining data is less than the minimum match length, set length and distance to 0 and return.
    if size - pos < ZOPFLI_MIN_MATCH {
        *length = 0;
        *distance = 0;
        return;
    }

    // Adjust the limit if the remaining data is smaller than the current limit.
    if pos + limit > size {
        limit = size - pos;
    }

    let arrayend = pos + limit;
    let arrayend_safe = if arrayend >= 8 { arrayend - 8 } else { 0 };

    // Ensure that the current hash value is within the valid range.
    assert!(hval < 65536);

    // Retrieve the position from the hash head.
    pp = hhead[hval as usize] as u16; // Cast to u16 for indexing.
    p = hprev[pp as usize];

    // Ensure that the retrieved position matches the current hash position.
    assert!(pp as usize == hpos);

    // Calculate the initial distance based on the positions.
    dist = if (p as usize) < (pp as usize) {
        (pp as usize) - (p as usize)
    } else {
        (ZOPFLI_WINDOW_SIZE - (p as usize)) + (pp as usize)
    };

    // Iterate through all possible distances.
    while dist < ZOPFLI_WINDOW_SIZE {
        let mut currentlength: u16 = 0;

        // Ensure that the position is within the window size.
        assert!((p as usize) < ZOPFLI_WINDOW_SIZE);
        // Ensure that the previous position matches.
        assert!(p == hprev[pp as usize]);
        // Ensure that the hash value at position p matches the current hash value.
        assert!(hhashval[p as usize] == hval);

        if dist > 0 {
            // Ensure that the current position is within the data size and the distance is valid.
            assert!(pos < size);
            assert!(dist <= pos);

            // Set the scan and match indices based on the current position and distance.
            scan_idx = pos;
            match_idx = pos - dist;

            // Check if the byte at the current best length matches.
            if pos + (bestlength as usize) >= size
                || array[pos + (bestlength as usize)] == array[match_idx + (bestlength as usize)]
            {
                // Retrieve the number of same bytes from the hash.
                let same0 = h.same[pos & ZOPFLI_WINDOW_MASK];
                if same0 > 2 && array[pos] == array[match_idx] {
                    // Retrieve the number of same bytes from the match position.
                    let same1 = h.same[(pos - dist) & ZOPFLI_WINDOW_MASK];
                    // Determine the minimum number of same bytes.
                    let mut same_min = if same0 < same1 { same0 } else { same1 };
                    // Convert 'same_min' to usize for comparison with 'limit'.
                    let mut same_limited = same_min as usize;
                    // Limit the same count to the current limit.
                    if same_limited > limit {
                        same_limited = limit;
                    }
                    // Advance the scan and match indices by the number of same bytes.
                    scan_idx += same_limited;
                    match_idx += same_limited;
                }
                // Compute safe_end and end for get_match, avoiding underflow.
                let safe_end = arrayend_safe.saturating_sub(scan_idx);
                let end = arrayend.saturating_sub(scan_idx);
                // Ensure safe_end is not greater than end.
                let safe_end = safe_end.min(end);

                // Find the match length using the `get_match` function with all required arguments.
                let match_length = get_match(
                    &array[scan_idx..],
                    &array[match_idx..],
                    end,
                    safe_end,
                );
                // Total length is the number of matching bytes including any skipped 'same' bytes.
                currentlength = (scan_idx - pos + match_length) as u16;
            }

            // If a longer match is found, update the best distance and length.
            if currentlength > bestlength {
                if let Some(sublen_mut) = sublen.as_deref_mut() {
                    for j in (bestlength + 1)..=currentlength {
                        // Ensure that the sublen array has enough capacity.
                        if (j as usize) < sublen_mut.len() {
                            sublen_mut[j as usize] = dist as u16;
                        }
                    }
                }
                bestdist = dist as u16;
                bestlength = currentlength;
                // If the current length reaches the limit, break early.
                if (bestlength as usize) >= limit {
                    break;
                }
            }
        }

        // Check if it's more efficient to switch to the second hash.
        if !std::ptr::eq(hhead, &h.head2)
            && (bestlength as usize) >= (h.same[hpos] as usize)
            && h.val2 == h.hashval2[p as usize]
        {
            // Switch to using the second hash for better performance.
            hhead = &h.head2;
            hprev = &h.prev2;
            hhashval = &h.hashval2;
            hval = h.val2;
        }

        // Update pp and p to traverse the hash chain.
        pp = p;
        p = hprev[p as usize];

        // If p equals pp, it indicates an uninitialized previous value, so break.
        if p == pp {
            break;
        }

        // Calculate the new distance based on the updated positions.
        dist += if (p as usize) < (pp as usize) {
            (pp as usize) - (p as usize)
        } else {
            (ZOPFLI_WINDOW_SIZE - (p as usize)) + (pp as usize)
        };

        // Decrement the chain counter and break if the maximum number of chain hits is reached.
        chain_counter -= 1;
        if chain_counter <= 0 {
            break;
        }
    }

    // Store the found distance and length in the cache.
    store_in_longest_match_cache(s, pos, limit, sublen.as_deref(), bestdist, bestlength);

    // Ensure that the best length does not exceed the limit.
    assert!((bestlength as usize) <= limit);

    // Update the distance and length with the best found values.
    *distance = bestdist;
    *length = bestlength;
    // Ensure that the position plus the best length does not exceed the data size.
    assert!((pos + (*length as usize)) <= size);
}


/// Performs LZ77 compression using a greedy algorithm similar to gzip, with lazy matching.
///
/// This function compresses the input data using an LZ77 approach with lazy matching. It stores the result in the provided `ZopfliLZ77Store`.
/// If `instart` is larger than 0, it uses values before `instart` as a starting dictionary.
///
/// # Arguments
///
/// * `s` - Mutable reference to the `ZopfliBlockState`.
/// * `input_data` - Slice of input data bytes to be compressed.
/// * `instart` - Starting index in the input data to begin compression.
/// * `inend` - Ending index in the input data (exclusive).
/// * `store` - Mutable reference to the `ZopfliLZ77Store` where the result will be stored.
/// * `h` - Mutable reference to the `ZopfliHash` structure used for hashing.
///
/// # Safety
///
/// This function performs safe operations and ensures that all array accesses are within bounds.
///
/// # Notes
///
/// This function follows the logic of the original C implementation closely, managing the index `i` carefully to reflect the increments in the C `for` loop.
pub fn zopfli_lz77_greedy(
    s: &mut ZopfliBlockState,
    input_data: &[u8],
    instart: usize,
    inend: usize,
    store: &mut ZopfliLZ77Store,
    h: &mut ZopfliHash,
) {
    let mut i: usize;
    let mut j: usize;
    let mut leng: u16 = 0;
    let mut dist: u16 = 0;
    let mut lengthscore: i32 = 0;
    let windowstart: usize = if instart > ZOPFLI_WINDOW_SIZE {
        instart - ZOPFLI_WINDOW_SIZE
    } else {
        0
    };
    let mut dummysublen = [0u16; 259];
    
    // Lazy matching variables
    let mut prev_length: usize = 0;
    let mut prev_match: usize = 0;
    let mut prevlengthscore: i32 = 0;
    let mut match_available: bool = false;
    
    if instart == inend {
        return;
    }
    
    // Reset and warm up the hash.
    zopfli_reset_hash(ZOPFLI_WINDOW_SIZE, h);
    zopfli_warmup_hash(input_data, windowstart, inend, h);
    // Update the hash for all positions from windowstart to instart - 1.
    for i in windowstart..instart {
        zopfli_update_hash(input_data, i, inend, h);
    }
    
    i = instart;
    
    // Main loop over the input data.
    while i < inend {
        zopfli_update_hash(input_data, i, inend, h);
    
        // Find the longest match at the current position.
        zopfli_find_longest_match(
            s,
            h,
            input_data,
            i,
            inend,
            ZOPFLI_MAX_MATCH,
            Some(&mut dummysublen[..]),
            &mut dist,
            &mut leng,
        );
    
        lengthscore = get_length_score(leng as i32, dist as i32);
    
        // Lazy matching logic.
        prevlengthscore = get_length_score(prev_length as i32, prev_match as i32);
        if match_available {
            match_available = false;
            if lengthscore > prevlengthscore + 1 {
                // Store the previous literal.
                zopfli_store_lit_len_dist(input_data[i - 1] as u16, 0, i - 1, store);
                if lengthscore >= ZOPFLI_MIN_MATCH as i32 && (leng as usize) < ZOPFLI_MAX_MATCH {
                    // Prepare for the next iteration with the current match.
                    match_available = true;
                    prev_length = leng as usize;
                    prev_match = dist as usize;
                    i += 1;
                    continue;
                }
            } else {
                // Add the previous match to the output.
                leng = prev_length as u16;
                dist = prev_match as u16;
                lengthscore = prevlengthscore;
                zopfli_verify_len_dist(input_data, inend, i - 1, dist, leng);
                zopfli_store_lit_len_dist(leng, dist, i - 1, store);
                // Update the hash for the positions skipped by the match.
                j = 2;
                while (j as u16) < leng {
                    i += 1;
                    if i >= inend {
                        break;
                    }
                    zopfli_update_hash(input_data, i, inend, h);
                    j += 1;
                }
                // After processing, continue to the next position.
                i += 1;
                continue;
            }
        } else if lengthscore >= ZOPFLI_MIN_MATCH as i32 && (leng as usize) < ZOPFLI_MAX_MATCH {
            // There's a match available; store it for potential lazy matching.
            match_available = true;
            prev_length = leng as usize;
            prev_match = dist as usize;
            i += 1;
            continue;
        }
    
        // End of lazy matching.
    
        // Add the current match or literal to the output.
        if lengthscore >= ZOPFLI_MIN_MATCH as i32 {
            zopfli_verify_len_dist(input_data, inend, i, dist, leng);
            zopfli_store_lit_len_dist(leng, dist, i, store);
        } else {
            leng = 1;
            zopfli_store_lit_len_dist(input_data[i] as u16, 0, i, store);
        }
        // Update the hash for the positions skipped by the match.
        j = 1;
        while (j as u16) < leng {
            i += 1;
            if i >= inend {
                break;
            }
            zopfli_update_hash(input_data, i, inend, h);
            j += 1;
        }
        // After processing, move to the next position.
        i += 1;
    }
}


/// Initializes a `ZopfliLZ77Store` with the provided data.
/// 
/// # Parameters
/// - `data`: A slice of unsigned bytes representing the original data.
/// - `store`: A mutable reference to a `ZopfliLZ77Store` that will be initialized.
/// 
/// This function sets the `size` to `0`, clears all vectors (`litlens`, `dists`, `pos`, 
/// `ll_symbol`, `d_symbol`, `ll_counts`, `d_counts`), and sets the `data` field to a 
/// copy of the provided `data`.
pub fn ZopfliInitLZ77Store(data: &[u8], store: &mut ZopfliLZ77Store) {
    // Initialize `size` to 0
    store.size = 0;
    
    // Clear the `litlens` vector (equivalent to setting pointer to NULL in C)
    store.litlens.clear();
    
    // Clear the `dists` vector (equivalent to setting pointer to NULL in C)
    store.dists.clear();
    
    // Clear the `pos` vector (equivalent to setting pointer to NULL in C)
    store.pos.clear();
    
    // Set the `data` field to a copy of the input `data`
    store.data = data.to_vec();
    
    // Clear the `ll_symbol` vector (equivalent to setting pointer to NULL in C)
    store.ll_symbol.clear();
    
    // Clear the `d_symbol` vector (equivalent to setting pointer to NULL in C)
    store.d_symbol.clear();
    
    // Clear the `ll_counts` vector (equivalent to setting pointer to NULL in C)
    store.ll_counts.clear();
    
    // Clear the `d_counts` vector (equivalent to setting pointer to NULL in C)
    store.d_counts.clear();
}


/// Cleans the ZopfliLZ77Store by releasing all allocated memory.
/// 
/// This function clears all vectors within the store, effectively freeing
/// the memory they occupy. After calling this function, the store's vectors
/// will be empty.
pub fn ZopfliCleanLZ77Store(store: &mut ZopfliLZ77Store) {
    // Clear the `litlens` vector, releasing its memory
    store.litlens = Vec::new();
    
    // Clear the `dists` vector, releasing its memory
    store.dists = Vec::new();
    
    // Clear the `pos` vector, releasing its memory
    store.pos = Vec::new();
    
    // Clear the `ll_symbol` vector, releasing its memory
    store.ll_symbol = Vec::new();
    
    // Clear the `d_symbol` vector, releasing its memory
    store.d_symbol = Vec::new();
    
    // Clear the `ll_counts` vector, releasing its memory
    store.ll_counts = Vec::new();
    
    // Clear the `d_counts` vector, releasing its memory
    store.d_counts = Vec::new();
}


use std::process::exit;

/// Initializes the `ZopfliLongestMatchCache`.
///
/// Allocates and initializes the `length`, `dist`, and `sublen` vectors based on the provided `blocksize`.
///
/// - `length` is initialized with `1` for each element.
/// - `dist` is initialized with `0` for each element.
/// - `sublen` is initialized with `0` for each element and its size is `ZOPFLI_CACHE_LENGTH * 3 * blocksize`.
///
/// If the allocation for `sublen` fails, the function prints an error message to `stderr` and exits the program.
///
/// # Arguments
///
/// * `blocksize` - The size of the block for which the cache is being initialized.
/// * `lmc` - A mutable reference to the `ZopfliLongestMatchCache` to be initialized.
pub fn zopfli_init_cache(blocksize: usize, lmc: &mut ZopfliLongestMatchCache) {
    // Allocate and initialize the `length` vector with 1s.
    lmc.length = vec![1u16; blocksize];

    // Allocate and initialize the `dist` vector with 0s.
    lmc.dist = vec![0u16; blocksize];

    // Calculate the required size for the `sublen` vector.
    let sublen_size = ZOPFLI_CACHE_LENGTH * 3 * blocksize;

    // Attempt to allocate the `sublen` vector with the calculated size.
    let mut sublen = Vec::with_capacity(sublen_size);

    // Try to reserve the exact capacity required for `sublen`.
    // If the reservation fails, print an error message and exit.
    if let Err(_) = sublen.try_reserve_exact(sublen_size) {
        eprintln!(
            "Error: Out of memory. Tried allocating {} bytes of memory.",
            sublen_size
        );
        exit(1);
    }

    // Initialize all elements of `sublen` to 0.
    sublen.resize(sublen_size, 0u8);

    // Assign the initialized `sublen` vector to the cache.
    lmc.sublen = sublen;
}


/// Initializes the `ZopfliBlockState` struct.
///
/// Sets the options, block start and end positions, and optionally initializes the longest match
/// cache (`lmc`) based on the `add_lmc` flag. If `add_lmc` is non-zero, it initializes the
/// `ZopfliLongestMatchCache` and assigns it to `s.lmc`. Otherwise, `s.lmc` is set to `None`.
///
/// # Arguments
///
/// * `options` - A reference to the `ZopfliOptions` to be used for this block.
/// * `blockstart` - The start position of the block (inclusive).
/// * `blockend` - The end position of the block (exclusive).
/// * `add_lmc` - An integer indicating whether to initialize the longest match cache (non-zero means true).
/// * `s` - A mutable reference to the `ZopfliBlockState` to be initialized.
pub fn zopfli_init_block_state(
    options: &ZopfliOptions,
    blockstart: usize,
    blockend: usize,
    add_lmc: i32,
    s: &mut ZopfliBlockState,
) {
    // Assign the provided options to the block state by cloning them.
    s.options = options.clone();

    // Set the block's start and end positions.
    s.blockstart = blockstart;
    s.blockend = blockend;

    if add_lmc != 0 {
        // If add_lmc is non-zero, initialize a new ZopfliLongestMatchCache.
        let mut lmc = ZopfliLongestMatchCache {
            length: Vec::new(),
            dist: Vec::new(),
            sublen: Vec::new(),
        };

        // Initialize the cache with the size of the block.
        zopfli_init_cache(blockend - blockstart, &mut lmc);

        // Assign the initialized cache to the block state's lmc field.
        s.lmc = Some(lmc);
    } else {
        // If add_lmc is zero, set lmc to None.
        s.lmc = None;
    }
}


/// Frees up the memory of the `ZopfliLongestMatchCache`.
///
/// This function releases the memory allocated for the `length`, `dist`,
/// and `sublen` vectors by replacing them with new empty vectors.
pub fn ZopfliCleanCache(lmc: &mut ZopfliLongestMatchCache) {
    // Free the `length` vector by assigning a new empty vector.
    lmc.length = Vec::new();
    
    // Free the `dist` vector by assigning a new empty vector.
    lmc.dist = Vec::new();
    
    // Free the `sublen` vector by assigning a new empty vector.
    lmc.sublen = Vec::new();
}


/// Cleans up the `ZopfliBlockState` by releasing the associated `ZopfliLongestMatchCache`.
///
/// This function checks if the `lmc` (Longest Match Cache) exists within the block state.
/// If it does, it cleans the cache by calling `ZopfliCleanCache` and then frees the cache
/// by setting it to `None`.
pub fn ZopfliCleanBlockState(s: &mut ZopfliBlockState) {
    // Check if the `lmc` (Longest Match Cache) is present.
    if let Some(ref mut lmc) = s.lmc {
        // Clean the cache by freeing its internal vectors.
        ZopfliCleanCache(lmc);
        
        // Free the `lmc` by setting it to `None`, which drops the `ZopfliLongestMatchCache`.
        s.lmc = None;
    }
}


/// Allocates memory for the ZopfliHash fields.
///
/// # Arguments
///
/// * `window_size` - The size of the window.
/// * `h` - Mutable reference to ZopfliHash to initialize.
pub fn zopfli_alloc_hash(window_size: usize, h: &mut ZopfliHash) {
    // Allocate memory for head with 65536 elements
    h.head = vec![0; 65536];
    
    // Allocate memory for prev with window_size elements
    h.prev = vec![0; window_size];
    
    // Allocate memory for hashval with window_size elements
    h.hashval = vec![0; window_size];
    
    // Allocate memory for same with window_size elements
    h.same = vec![0; window_size];
    
    // Allocate memory for head2 with 65536 elements
    h.head2 = vec![0; 65536];
    
    // Allocate memory for prev2 with window_size elements
    h.prev2 = vec![0; window_size];
    
    // Allocate memory for hashval2 with window_size elements
    h.hashval2 = vec![0; window_size];
}


/// Frees the memory used by a `ZopfliHash` by clearing all its vectors.
///
/// This function clears each vector within the `ZopfliHash` structure,
/// effectively releasing the allocated memory for each field.
/// 
/// # Arguments
///
/// * `h` - A mutable reference to the `ZopfliHash` instance to be cleaned.
pub fn zopfli_clean_hash(h: &mut ZopfliHash) {
    // Clear the `head` vector.
    h.head.clear();
    
    // Clear the `prev` vector.
    h.prev.clear();
    
    // Clear the `hashval` vector.
    h.hashval.clear();
    
    // Clear the second set of hash vectors.
    h.head2.clear();
    h.prev2.clear();
    h.hashval2.clear();
    
    // Clear the `same` vector.
    h.same.clear();
}


/// Performs block splitting on uncompressed data.
/// The output `splitpoints` are indices in the uncompressed bytes.
///
/// # Arguments
///
/// * `options` - Reference to general program options.
/// * `input_data` - Slice of uncompressed input data.
/// * `instart` - Start index for splitting.
/// * `inend` - End index for splitting (not inclusive).
/// * `maxblocks` - Maximum number of blocks to split into, or `0` for no limit.
/// * `splitpoints` - Mutable vector to store resulting split point indices.
/// * `npoints` - Mutable reference to store the number of split points.
///
/// # Behavior
///
/// This function initializes necessary structures, performs LZ77 greedy compression,
/// splits blocks based on LZ77 data, converts LZ77 split points to uncompressed
/// input positions, and updates the `splitpoints` and `npoints` accordingly.
/// Finally, it cleans up allocated resources.
///
/// # Panics
///
/// Panics if the number of split points does not match the expected number after processing.
///
/// # Example
///
/// ```
/// let options = ZopfliOptions {
///     verbose: true,
///     verbose_more: false,
///     numiterations: 10,
///     blocksplitting: 1,
///     blocksplittinglast: 0,
///     blocksplittingmax: 15,
/// };
/// let input_data = vec![/* uncompressed data */];
/// let instart = 0;
/// let inend = input_data.len();
/// let maxblocks = 10;
/// let mut splitpoints = Vec::new();
/// let mut npoints = 0;
///
/// zopfli_block_split(
///     &options,
///     &input_data,
///     instart,
///     inend,
///     maxblocks,
///     &mut splitpoints,
///     &mut npoints,
/// );
/// ```
pub fn zopfli_block_split(
    options: &ZopfliOptions,
    input_data: &[u8],
    instart: usize,
    inend: usize,
    maxblocks: usize,
    splitpoints: &mut Vec<usize>,
    npoints: &mut usize,
) {
    // Initialize position to the start index.
    let mut pos: usize = instart;

    // Declare variables for block state, LZ77 split points, and count of split points.
    let mut s = ZopfliBlockState {
        options: options.clone(),
        lmc: None,
        blockstart: instart,
        blockend: inend,
    };
    let mut lz77splitpoints: Vec<usize> = Vec::new();
    let mut nlz77points: usize = 0;

    // Initialize the LZ77 store.
    let mut store: ZopfliLZ77Store = ZopfliLZ77Store {
        litlens: Vec::new(),
        dists: Vec::new(),
        size: 0,
        data: input_data.to_vec(),
        pos: Vec::new(),
        ll_symbol: Vec::new(),
        d_symbol: Vec::new(),
        ll_counts: Vec::new(),
        d_counts: Vec::new(),
    };

    // Initialize the hash structure.
    let mut hash = ZopfliHash {
        head: Vec::new(),
        prev: Vec::new(),
        hashval: Vec::new(),
        val: 0,
        head2: Vec::new(),
        prev2: Vec::new(),
        hashval2: Vec::new(),
        val2: 0,
        same: Vec::new(),
    };
    let h = &mut hash;

    // Initialize the LZ77 store with the input data.
    ZopfliInitLZ77Store(input_data, &mut store);

    // Initialize the block state without adding a longest match cache (LMC).
    zopfli_init_block_state(options, instart, inend, 0, &mut s);

    // Allocate memory for the hash structures based on the window size.
    zopfli_alloc_hash(ZOPFLI_WINDOW_SIZE, h);

    // Reset the number of split points and clear the splitpoints vector.
    *npoints = 0;
    splitpoints.clear();

    // Perform LZ77 compression using a greedy algorithm.
    zopfli_lz77_greedy(&mut s, input_data, instart, inend, &mut store, h);

    // Perform block splitting based on the LZ77 data.
    zopfli_block_split_lz77(
        options,
        &store,
        maxblocks,
        &mut lz77splitpoints,
        &mut nlz77points,
    );

    // Convert LZ77 split points to positions in the uncompressed input data.
    pos = instart;
    if nlz77points > 0 {
        let mut splitpoint_index = 0;
        for i in 0..store.size {
            // Determine the length to advance based on whether there's a distance.
            let length = if store.dists[i] == 0 {
                1
            } else {
                store.litlens[i] as usize
            };

            // Check if the current index `i` is a split point.
            if splitpoint_index < lz77splitpoints.len() && lz77splitpoints[splitpoint_index] == i {
                // Append the current position to the splitpoints vector.
                zopfli_append_data(pos, splitpoints, npoints);
                splitpoint_index += 1;

                // If all split points have been processed, exit the loop.
                if splitpoint_index == nlz77points {
                    break;
                }
            }

            // Advance the position by the determined length.
            pos += length;
        }
    }

    // Ensure that the number of split points matches the expected count.
    assert!(*npoints == nlz77points, "Mismatch in split points count");

    // Clean up allocated resources by cleaning the block state, LZ77 store, and hash.
    ZopfliCleanBlockState(&mut s);
    ZopfliCleanLZ77Store(&mut store);
    zopfli_clean_hash(h);
}


/// Sets all fields of `SymbolStats` to zero.
pub fn init_stats(stats: &mut SymbolStats) {
    // Set all literal and length symbols to 0
    for i in 0..ZOPFLI_NUM_LL {
        stats.litlens[i] = 0;
    }

    // Set all distance symbols to 0
    for i in 0..ZOPFLI_NUM_D {
        stats.dists[i] = 0;
    }

    // Set all literal/length symbol lengths to 0.0
    for i in 0..ZOPFLI_NUM_LL {
        stats.ll_symbols[i] = 0.0;
    }

    // Set all distance symbol lengths to 0.0
    for i in 0..ZOPFLI_NUM_D {
        stats.d_symbols[i] = 0.0;
    }
}


/// Copies the contents of the source `SymbolStats` into the destination `SymbolStats`.
///
/// # Arguments
///
/// * `source` - A reference to the source `SymbolStats` to copy from.
/// * `dest` - A mutable reference to the destination `SymbolStats` to copy to.
pub fn copy_stats(source: &SymbolStats, dest: &mut SymbolStats) {
    // Copy the `litlens` array from source to destination
    dest.litlens = source.litlens;
    
    // Copy the `dists` array from source to destination
    dest.dists = source.dists;
    
    // Copy the `ll_symbols` array from source to destination
    dest.ll_symbols = source.ll_symbols;
    
    // Copy the `d_symbols` array from source to destination
    dest.d_symbols = source.d_symbols;
}


/// CeilDiv computes the ceiling of a divided by b.
/// 
/// This function calculates the smallest integer greater than or equal to a / b.
/// It effectively performs a ceiling division by adding (b - 1) to a before dividing by b.
fn ceil_div(a: usize, b: usize) -> usize {
    // Add (b - 1) to a to ensure the division rounds up
    (a + b - 1) / b
}


/// Copies the contents of one `ZopfliLZ77Store` (`source`) to another (`dest`).
///
/// This function performs the following steps:
/// 1. Calculates the required sizes for `ll_counts` and `d_counts` using `ceil_div`.
/// 2. Cleans the destination store by clearing all its vectors.
/// 3. Initializes the destination store with the source's data.
/// 4. Allocates memory for `litlens`, `dists`, `pos`, `ll_symbol`, `d_symbol`,
///    `ll_counts`, and `d_counts` in the destination store.
/// 5. Copies each element from the source vectors to the corresponding
///    destination vectors.
///
/// # Parameters
/// - `source`: A reference to the source `ZopfliLZ77Store` to copy from.
/// - `dest`: A mutable reference to the destination `ZopfliLZ77Store` to copy to.
///
/// # Panics
/// This function will panic if memory allocation for any of the vectors fails.
pub fn ZopfliCopyLZ77Store(source: &ZopfliLZ77Store, dest: &mut ZopfliLZ77Store) {
    // Calculate the sizes for `ll_counts` and `d_counts` using ceiling division
    let llsize = ZOPFLI_NUM_LL * ceil_div(source.size, ZOPFLI_NUM_LL);
    let dsize = ZOPFLI_NUM_D * ceil_div(source.size, ZOPFLI_NUM_D);
    
    // Clean the destination store by releasing any allocated memory
    ZopfliCleanLZ77Store(dest);
    
    // Initialize the destination store with the source's data
    ZopfliInitLZ77Store(&source.data, dest);
    
    // Allocate memory for `litlens`, `dists`, and `pos` with the same size as the source
    dest.litlens = Vec::with_capacity(source.size);
    dest.dists = Vec::with_capacity(source.size);
    dest.pos = Vec::with_capacity(source.size);
    
    // Allocate memory for `ll_symbol` and `d_symbol` with the same size as the source
    dest.ll_symbol = Vec::with_capacity(source.size);
    dest.d_symbol = Vec::with_capacity(source.size);
    
    // Allocate memory for `ll_counts` and `d_counts` based on the calculated sizes
    dest.ll_counts = Vec::with_capacity(llsize);
    dest.d_counts = Vec::with_capacity(dsize);

    // Set the size of the destination store to match the source
    dest.size = source.size;
    
    // Copy each element from `litlens`, `dists`, `pos`, `ll_symbol`, and `d_symbol`
    // from the source to the destination
    for i in 0..source.size {
        dest.litlens.push(source.litlens[i]);
        dest.dists.push(source.dists[i]);
        dest.pos.push(source.pos[i]);
        dest.ll_symbol.push(source.ll_symbol[i]);
        dest.d_symbol.push(source.d_symbol[i]);
    }
    
    // Copy each element from `ll_counts` from the source to the destination
    for i in 0..llsize {
        dest.ll_counts.push(source.ll_counts[i]);
    }
    
    // Copy each element from `d_counts` from the source to the destination
    for i in 0..dsize {
        dest.d_counts.push(source.d_counts[i]);
    }
}


/// Adds the bit lengths.
///
/// This function takes two `SymbolStats` instances (`stats1` and `stats2`) along with their
/// respective weights (`w1` and `w2`). It computes a weighted sum of the `litlens` and
/// `dists` arrays from both `SymbolStats` and stores the result in the `result` parameter.
/// Additionally, it sets the end symbol frequency in the `litlens` array.
pub fn add_weighed_stat_freqs(
    stats1: &SymbolStats, 
    w1: f64,
    stats2: &SymbolStats, 
    w2: f64,
    result: &mut SymbolStats
) {
    // Iterate over each literal/length symbol and compute the weighted sum.
    for i in 0..ZOPFLI_NUM_LL {
        result.litlens[i] = (stats1.litlens[i] as f64 * w1 + stats2.litlens[i] as f64 * w2) as usize;
    }

    // Iterate over each distance symbol and compute the weighted sum.
    for i in 0..ZOPFLI_NUM_D {
        result.dists[i] = (stats1.dists[i] as f64 * w1 + stats2.dists[i] as f64 * w2) as usize;
    }

    // Set the frequency of the end symbol.
    result.litlens[256] = 1; // End symbol.
}


/// Initializes the `RanState` by setting `m_w` to 1 and `m_z` to 2.
/// 
/// # Arguments
/// 
/// * `state` - A mutable reference to a `RanState` instance.
/// 
/// # Example
/// 
/// ```
/// let mut state = RanState { m_w: 0, m_z: 0 };
/// init_ran_state(&mut state);
/// assert_eq!(state.m_w, 1);
/// assert_eq!(state.m_z, 2);
/// ```
pub fn init_ran_state(state: &mut RanState) {
    // Set the `m_w` field to 1
    state.m_w = 1;
    
    // Set the `m_z` field to 2
    state.m_z = 2;
}


/// Get random number: "Multiply-With-Carry" generator of G. Marsaglia
pub fn ran(state: &mut RanState) -> u32 {
    // Update m_z with the formula: m_z = 36969 * (m_z & 65535) + (m_z >> 16)
    state.m_z = 36969u32.wrapping_mul(state.m_z & 65535)
                 .wrapping_add(state.m_z >> 16);
    
    // Update m_w with the formula: m_w = 18000 * (m_w & 65535) + (m_w >> 16)
    state.m_w = 18000u32.wrapping_mul(state.m_w & 65535)
                 .wrapping_add(state.m_w >> 16);
    
    // Combine m_z and m_w to form the 32-bit result
    (state.m_z << 16).wrapping_add(state.m_w)
}


/// Randomizes the frequencies based on the provided random state.
///
/// For each index `i` from `0` to `n - 1`, a random number is generated. If the
/// condition `(Ran(state) >> 4) % 3 == 0` holds, `freqs[i]` is replaced with
/// the frequency at a random index `freqs[Ran(state) % n]`.
///
/// # Arguments
///
/// * `state` - A mutable reference to the random state.
/// * `freqs` - A mutable slice of frequency counts.
/// * `n` - The number of elements in the `freqs` slice.
pub fn randomize_freqs(state: &mut RanState, freqs: &mut [usize], n: i32) {
    // Iterate over each index from 0 to n-1
    for i in 0..n {
        // Generate a random number, shift it right by 4 bits, take modulo 3
        // and check if the result is equal to 0
        if (ran(state) >> 4) % 3 == 0 {
            // Generate another random number and take modulo n to get a valid index
            let random_index = (ran(state) % (n as u32)) as usize;
            // Replace freqs[i] with the frequency at the random index
            freqs[i as usize] = freqs[random_index];
        }
    }
}


/// Randomizes the statistics frequencies based on the provided random state.
///
/// This function randomizes both the literal/length frequencies and the distance
/// frequencies within the `SymbolStats` structure using the provided random state.
/// After randomization, it explicitly sets the frequency of the end symbol (256) to 1.
///
/// # Arguments
///
/// * `state` - A mutable reference to the random state.
/// * `stats` - A mutable reference to the symbol statistics to be randomized.
pub fn randomize_stat_freqs(state: &mut RanState, stats: &mut SymbolStats) {
    // Randomize the literal and length frequencies using ZOPFLI_NUM_LL as the size.
    randomize_freqs(state, &mut stats.litlens, ZOPFLI_NUM_LL as i32);
    
    // Randomize the distance frequencies using ZOPFLI_NUM_D as the size.
    randomize_freqs(state, &mut stats.dists, ZOPFLI_NUM_D as i32);
    
    // Set the frequency of the end symbol (256) to 1.
    stats.litlens[256] = 1;
}


/// Clears the frequency counts in `SymbolStats` by setting the `litlens` and `dists` arrays to zero.
///
/// # Arguments
///
/// * `stats` - A mutable reference to a `SymbolStats` instance whose frequency counts are to be cleared.
pub fn clear_stat_freqs(stats: &mut SymbolStats) {
    // Iterate over all literal/length symbols and set their counts to zero
    for i in 0..ZOPFLI_NUM_LL {
        stats.litlens[i] = 0;
    }
    
    // Iterate over all distance symbols and set their counts to zero
    for i in 0..ZOPFLI_NUM_D {
        stats.dists[i] = 0;
    }
}


/// Calculates the entropy of each symbol based on the counts of each symbol.
/// The result is similar to the result of `zopfli_calculate_bit_lengths`, but with the
/// actual theoretical bit lengths according to the entropy. Since the resulting
/// values are fractional, they cannot be used to encode the tree specified by
/// DEFLATE.
///
/// # Arguments
///
/// * `count` - A slice of counts for each symbol.
/// * `n` - The number of symbols.
/// * `bitlengths` - A mutable slice where the calculated bit lengths will be stored.
///
/// # Panics
///
/// This function will panic if `n` is greater than the length of `count` or `bitlengths`.
pub fn zopfli_calculate_entropy(count: &[usize], n: usize, bitlengths: &mut [f64]) {
    // Ensure that `n` does not exceed the lengths of `count` and `bitlengths`.
    assert!(n <= count.len(), "n exceeds the length of count");
    assert!(n <= bitlengths.len(), "n exceeds the length of bitlengths");

    // 1. Define the constant kInvLog2, which is 1.0 / log(2.0)
    const K_INV_LOG2: f64 = 1.4426950408889;

    // 2. Calculate the sum of all counts.
    let mut sum: usize = 0;
    for i in 0..n {
        sum += count[i];
    }

    // 3. Calculate log2sum:
    //    If the sum is zero, use log(n) instead of log(sum).
    //    Multiply the result by kInvLog2 to convert natural log to log base 2.
    let log2sum = if sum == 0 {
        (n as f64).ln() * K_INV_LOG2
    } else {
        (sum as f64).ln() * K_INV_LOG2
    };

    // 4. Calculate the entropy bit lengths for each symbol.
    for i in 0..n {
        if count[i] == 0 {
            // If the count is zero, assume the symbol appears at least once.
            bitlengths[i] = log2sum;
        } else {
            // Calculate the bit length based on the entropy formula.
            bitlengths[i] = log2sum - (count[i] as f64).ln() * K_INV_LOG2;
        }

        // 5. Clamp the bit length to zero if it's slightly negative due to floating-point imprecision.
        if bitlengths[i] < 0.0 && bitlengths[i] > -1e-5 {
            bitlengths[i] = 0.0;
        }

        // 6. Ensure that the bit length is not negative.
        assert!(
            bitlengths[i] >= 0.0,
            "Bit length for symbol {} is negative: {}",
            i,
            bitlengths[i]
        );
    }
}


/// Calculates the entropy of the statistics.
///
/// # Arguments
///
/// * `stats` - A mutable reference to `SymbolStats` where the entropy bit lengths will be calculated.
pub fn calculate_statistics(stats: &mut SymbolStats) {
    // Calculate entropy for literal and length symbols.
    zopfli_calculate_entropy(&stats.litlens, ZOPFLI_NUM_LL, &mut stats.ll_symbols);
    
    // Calculate entropy for distance symbols.
    zopfli_calculate_entropy(&stats.dists, ZOPFLI_NUM_D, &mut stats.d_symbols);
}


/// Appends the symbol statistics from the `store` to `stats`.
///
/// # Arguments
///
/// * `store` - A reference to `ZopfliLZ77Store` containing the LZ77 data.
/// * `stats` - A mutable reference to `SymbolStats` where statistics will be accumulated.
pub fn get_statistics(store: &ZopfliLZ77Store, stats: &mut SymbolStats) {
    // Iterate over each LZ77 element in the store
    for i in 0..store.size {
        if store.dists[i] == 0 {
            // If distance is 0, it's a literal
            stats.litlens[store.litlens[i] as usize] += 1;
        } else {
            // Otherwise, it's a length/distance pair
            let length_symbol = ZopfliGetLengthSymbol(store.litlens[i] as i32) as usize;
            stats.litlens[length_symbol] += 1;
            
            let dist_symbol = zopfli_get_dist_symbol(store.dists[i] as i32) as usize;
            stats.dists[dist_symbol] += 1;
        }
    }

    // Add the end symbol
    stats.litlens[256] = 1;

    // Calculate entropy based on the updated statistics
    calculate_statistics(stats);
}


/// Finds the minimum possible cost this cost model can return for valid length and
/// distance symbols.
///
/// # Arguments
///
/// * `costmodel` - A reference to the cost model function or closure.
/// * `costcontext` - A reference to any context-specific data required by the cost model.
///
/// # Returns
///
/// Returns the minimum possible cost as an `f64`.
///
/// # Details
///
/// The function iterates over valid length and distance symbols to find the combination
/// that yields the lowest cost as determined by the provided cost model. It checks all
/// lengths from 3 to 258 and a predefined set of distances that correspond to different
/// distance symbols in the DEFLATE specification.
///
/// The `dsymbols` array contains distances where each value is the first distance that has
/// a new symbol. Only these distances need to be checked since only different symbols
/// affect the cost model. This is based on RFC 1951 section 3.2.5.
///
/// # Notes
///
/// - The function uses `ZOPFLI_LARGE_FLOAT` as an initial high cost value.
/// - The best length and distance are found separately and then combined to compute the final cost.
pub fn get_cost_model_min_cost(
    costmodel: &dyn Fn(u32, u32, &dyn std::any::Any) -> f64,
    costcontext: &dyn std::any::Any,
) -> f64 {
    // Initialize variables for minimum cost and best length/distance.
    let mut min_cost: f64;
    let mut best_length: u32 = 0; // Length that has the lowest cost in the cost model.
    let mut best_dist: u32 = 0;   // Distance that has the lowest cost in the cost model.

    /*
    Table of distances that have a different distance symbol in the DEFLATE
    specification. Each value is the first distance that has a new symbol. Only
    different symbols affect the cost model, so only these need to be checked.
    See RFC 1951 section 3.2.5: Compressed blocks (length and distance codes).
    */
    let dsymbols: [u32; 30] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25,
        33, 49, 65, 97, 129, 193, 257, 385,
        513, 769, 1025, 1537, 2049, 3073,
        4097, 6145, 8193, 12289, 16385, 24577,
    ];

    // Find the length with the lowest cost.
    min_cost = ZOPFLI_LARGE_FLOAT;
    for i in 3..259 {
        let c = costmodel(i as u32, 1, costcontext);
        if c < min_cost {
            best_length = i as u32;
            min_cost = c;
        }
    }

    // Find the distance with the lowest cost.
    min_cost = ZOPFLI_LARGE_FLOAT;
    for &dist in dsymbols.iter() {
        let c = costmodel(3, dist, costcontext);
        if c < min_cost {
            best_dist = dist;
            min_cost = c;
        }
    }

    // Return the minimum possible cost using the best length and distance found.
    costmodel(best_length, best_dist, costcontext)
}


/// Returns the minimum of two `usize` values.
/// Equivalent to the C function `zopfli_min`.
///
/// # Arguments
///
/// * `a` - The first `usize` value.
/// * `b` - The second `usize` value.
///
/// # Returns
///
/// The smaller of `a` and `b`.
pub fn zopfli_min(a: usize, b: usize) -> usize {
    // Compare `a` and `b` and return the smaller one
    if a < b {
        a
    } else {
        b
    }
}


/// Performs the forward pass for "squeeze". Gets the most optimal length to reach
/// every byte from a previous byte, using cost calculations.
///
/// # Arguments
///
/// * `s` - The `ZopfliBlockState`.
/// * `input_data` - The input data array.
/// * `instart` - Where to start.
/// * `inend` - Where to stop (not inclusive).
/// * `costmodel` - Function to calculate the cost of some lit/len/dist pair.
/// * `costcontext` - Abstract context for the costmodel function.
/// * `length_array` - Output array of size `(inend - instart)` which will receive the best
///   length to reach this byte from a previous byte.
/// * `h` - The `ZopfliHash`.
/// * `costs` - Array to store the best cost to get to each position.
///
/// # Returns
///
/// Returns the cost that was, according to the costmodel, needed to get to the end.
pub fn get_best_lengths(
    s: &mut ZopfliBlockState,
    input_data: &[u8],
    instart: usize,
    inend: usize,
    costmodel: &dyn Fn(u32, u32, &dyn std::any::Any) -> f64,
    costcontext: &dyn std::any::Any,
    length_array: &mut [u16],
    h: &mut ZopfliHash,
    costs: &mut [f32],
) -> f64 {
    // Best cost to get here so far.
    let blocksize = inend - instart;
    let mut i: usize;
    let mut k: usize;
    let mut kend: usize;
    let mut leng: u16 = 0;
    let mut dist: u16 = 0;
    let mut sublen: [u16; 259] = [0; 259];
    // Determine the start position for the window, ensuring it doesn't go below zero.
    let windowstart = if instart > ZOPFLI_WINDOW_SIZE {
        instart - ZOPFLI_WINDOW_SIZE
    } else {
        0
    };
    let mut result: f64;
    let mincost = get_cost_model_min_cost(costmodel, costcontext);
    let mut mincostaddcostj: f64;

    // If the input start equals the input end, there is nothing to process.
    if instart == inend {
        return 0.0;
    }

    // Reset and warm up the hash with the data before the starting position.
    zopfli_reset_hash(ZOPFLI_WINDOW_SIZE, h);
    zopfli_warmup_hash(input_data, windowstart, inend, h);
    for i in windowstart..instart {
        zopfli_update_hash(input_data, i, inend, h);
    }

    // Initialize the costs array with large values and set the start cost to 0.
    for i in 1..=blocksize {
        costs[i] = ZOPFLI_LARGE_FLOAT as f32;
    }
    costs[0] = 0.0; // Start position has zero cost.
    length_array[0] = 0;

    // Main loop to process each position from instart to inend.
    let mut i = instart;
    while i < inend {
        let mut j = i - instart; // Index in the costs array and length_array.
        zopfli_update_hash(input_data, i, inend, h);

        // Check for long repetitions of the same character.
        if (h.same[i & ZOPFLI_WINDOW_MASK] as usize) > ZOPFLI_MAX_MATCH * 2
            && i > instart + ZOPFLI_MAX_MATCH + 1
            && i + ZOPFLI_MAX_MATCH * 2 + 1 < inend
            && (h.same[(i - ZOPFLI_MAX_MATCH) & ZOPFLI_WINDOW_MASK] as usize)
                > ZOPFLI_MAX_MATCH
        {
            // Calculate the cost of the maximum match length.
            let symbolcost = costmodel(ZOPFLI_MAX_MATCH as u32, 1, costcontext);
            // Set the length and cost for each position to skip calling the match finder.
            for _ in 0..ZOPFLI_MAX_MATCH {
                if j + ZOPFLI_MAX_MATCH >= costs.len() || j >= costs.len() {
                    break;
                }
                costs[j + ZOPFLI_MAX_MATCH] = costs[j] + symbolcost as f32;
                length_array[j + ZOPFLI_MAX_MATCH] = ZOPFLI_MAX_MATCH as u16;
                i += 1;
                j += 1;
                if i >= inend {
                    break;
                }
                zopfli_update_hash(input_data, i, inend, h);
            }
            continue; // Continue to the next iteration.
        }

        // Find the longest match at the current position.
        zopfli_find_longest_match(
            s,
            h,
            input_data,
            i,
            inend,
            ZOPFLI_MAX_MATCH,
            Some(&mut sublen),
            &mut dist,
            &mut leng,
        );

        // Handle the case for a literal character.
        if i + 1 <= inend {
            let new_cost = costmodel(input_data[i] as u32, 0, costcontext) + costs[j] as f64;
            assert!(new_cost >= 0.0);
            if new_cost < costs[j + 1] as f64 {
                costs[j + 1] = new_cost as f32;
                length_array[j + 1] = 1;
            }
        }

        // Handle possible match lengths at the current position.
        kend = zopfli_min(leng as usize, inend - i);
        mincostaddcostj = mincost + costs[j] as f64;
        for k in 3..=kend {
            if j + k >= costs.len() {
                break;
            }
            // Skip if the cost is already less than or equal to the minimal possible cost.
            if (costs[j + k] as f64) <= mincostaddcostj {
                continue;
            }

            let new_cost = costmodel(k as u32, sublen[k] as u32, costcontext) + costs[j] as f64;
            assert!(new_cost >= 0.0);
            if new_cost < costs[j + k] as f64 {
                assert!(k <= ZOPFLI_MAX_MATCH);
                costs[j + k] = new_cost as f32;
                length_array[j + k] = k as u16;
            }
        }
        i += 1;
    }

    // Ensure the final cost is non-negative and return it.
    assert!(costs[blocksize] >= 0.0);
    result = costs[blocksize] as f64;

    result
}


/// Calculates the optimal path of LZ77 lengths to use from the given `length_array`.
/// The `length_array` must contain the optimal length to reach each byte.
/// The `path` will be filled with the lengths to use, and `pathsize` will be updated
/// to reflect the number of LZ77 symbols.
///
/// # Arguments
///
/// * `size` - The total size of the input data.
/// * `length_array` - A slice containing the optimal lengths to reach each byte.
/// * `path` - A mutable reference to a vector where the path will be stored.
/// * `pathsize` - A mutable reference to the size of the path.
pub fn trace_backwards(
    size: usize,
    length_array: &[u16],
    path: &mut Vec<u16>,
    pathsize: &mut usize,
) {
    let mut index = size;
    if size == 0 {
        return;
    }

    loop {
        // Append the current length to the path.
        // This may resize the `path` vector if needed.
        zopfli_append_data(length_array[index], path, pathsize);

        // Ensure the length is valid and does not exceed bounds.
        assert!((length_array[index] as usize) <= index, "length_array[index] exceeds current index");
        assert!(
            length_array[index] <= ZOPFLI_MAX_MATCH as u16,
            "length_array[index] exceeds ZOPFLI_MAX_MATCH"
        );
        assert!(length_array[index] != 0, "length_array[index] is zero");

        // Move the index backwards by the length of the current match.
        index -= length_array[index] as usize;

        // If we've reached the beginning, exit the loop.
        if index == 0 {
            break;
        }
    }

    // Reverse the path to obtain the correct order of LZ77 symbols.
    for i in 0..(*pathsize / 2) {
        let opposite_index = *pathsize - i - 1;
        path.swap(i, opposite_index);
    }
}


/// Follows the provided path to generate LZ77 length and distance pairs and stores them.
///
/// This function processes a sequence of lengths from the `path` array, updating the hash
/// state and storing the corresponding literal or length-distance pairs in the `store`.
///
/// # Arguments
///
/// * `s` - Mutable reference to the `ZopfliBlockState`, containing options and cache.
/// * `input` - A slice of bytes representing the input data to be processed.
/// * `instart` - The starting index in `input` from where to begin processing.
/// * `inend` - The ending index in `input` up to which to process.
/// * `path` - An optional slice of `u16` representing the path of lengths. Can be `None`.
/// * `pathsize` - The number of elements in the `path` array.
/// * `store` - Mutable reference to the `ZopfliLZ77Store` where the results will be stored.
/// * `h` - Mutable reference to the `ZopfliHash` used for hashing positions in the input.
///
/// # Notes
///
/// - If `path` is `None`, the function treats it as an empty path and performs no operations.
/// - The function ensures that all operations stay within the bounds of the `input`.
/// - Assertions are used to ensure the integrity of the data being processed.
///
/// # Panics
///
/// This function will panic if:
/// - `instart` is greater than `inend`.
/// - Any `length` in the `path` exceeds the bounds of the `input`.
/// - Any distance-length pair fails verification.
/// - The hash updates lead to inconsistent states.
///
/// # Safety
///
/// This function assumes that:
/// - `instart` and `inend` are valid indices within `input`.
/// - If `path` is `Some`, it contains at least `pathsize` elements.
///
/// # Example
///
/// ```
/// let mut block_state = ZopfliBlockState { /* initialize appropriately */ };
/// let input_data = vec![/* input bytes */];
/// let path = Some(&[3, 4, 5][..]);
/// let mut store = ZopfliLZ77Store { /* initialize appropriately */ };
/// let mut hash = ZopfliHash { /* initialize appropriately */ };
///
/// zopfli_follow_path(
///     &mut block_state,
///     &input_data,
///     0,
///     input_data.len(),
///     path,
///     path.unwrap().len(),
///     &mut store,
///     &mut hash,
/// );
/// ```
pub fn zopfli_follow_path(
    s: &mut ZopfliBlockState,
    input: &[u8],
    instart: usize,
    inend: usize,
    path: Option<&[u16]>,
    pathsize: usize,
    store: &mut ZopfliLZ77Store,
    h: &mut ZopfliHash,
) {
    // Initialize variables: 'pos' tracks the current position, 'total_length_test' accumulates the total length processed.
    let mut pos: usize = 0;
    let mut total_length_test: usize = 0;

    // Determine the start of the window. If 'instart' exceeds the window size, adjust accordingly.
    let windowstart: usize = if instart > ZOPFLI_WINDOW_SIZE {
        instart - ZOPFLI_WINDOW_SIZE
    } else {
        0
    };

    // If there is no data to process, exit early.
    if instart == inend {
        return;
    }

    // Reset the hash state with the specified window size.
    zopfli_reset_hash(ZOPFLI_WINDOW_SIZE, h);

    // Prepopulate the hash with data from 'windowstart' to 'inend' to ensure accurate hashing.
    zopfli_warmup_hash(input, windowstart, inend, h);

    // Update the hash for each position from 'windowstart' up to 'instart'.
    for i in windowstart..instart {
        zopfli_update_hash(input, i, inend, h);
    }

    // Set the current position to 'instart'.
    pos = instart;

    // Iterate over each element in the 'path' array up to 'pathsize'.
    for i in 0..pathsize {
        // Retrieve the length from the 'path'. If 'path' is None, treat it as an empty slice.
        let length = if let Some(p) = path {
            p[i]
        } else {
            0
        };

        // Initialize variables to store the dummy length and distance.
        let mut dummy_length: u16 = 0;
        let mut dist: u16 = 0;

        // Ensure that the current position is within the input bounds.
        assert!(
            pos < inend,
            "Current position {} is out of bounds (inend: {})",
            pos,
            inend
        );

        // Update the hash with the current position.
        zopfli_update_hash(input, pos, inend, h);

        // If the length from the path is greater than or equal to the minimum match length,
        // it represents a length-distance pair.
        if length >= ZOPFLI_MIN_MATCH as u16 {
            // Find the longest match for the current position and length.
            zopfli_find_longest_match(
                s,
                h,
                input,
                pos,
                inend,
                length as usize,
                None,
                &mut dist,
                &mut dummy_length,
            );

            // Ensure that the dummy length matches the actual length unless both are greater than 2.
            assert!(
                !(dummy_length != length && length > 2 && dummy_length > 2),
                "Dummy length {} does not match length {}",
                dummy_length,
                length
            );

            // Verify that the length-distance pair is valid within the input.
            zopfli_verify_len_dist(input, inend, pos, dist, length);

            // Store the literal or length-distance pair in the LZ77 store.
            zopfli_store_lit_len_dist(length, dist, pos, store);

            // Accumulate the total length processed for testing purposes.
            total_length_test += length as usize;
        } else {
            // If the length is less than the minimum match length, treat it as a literal.
            let adjusted_length: u16 = 1;
            // Convert the literal byte to u16 before storing.
            zopfli_store_lit_len_dist(input[pos] as u16, 0, pos, store);
            total_length_test += adjusted_length as usize;
        }

        // Ensure that the new position does not exceed the input bounds.
        assert!(
            pos + length as usize <= inend,
            "Position {} with length {} exceeds inend {}",
            pos,
            length,
            inend
        );

        // Update the hash for each byte within the current match length.
        for j in 1..length {
            // Calculate the new position by adding 'j' to the current position.
            let new_pos = pos + j as usize;

            // Update the hash with the new position. Ensure 'new_pos' does not exceed 'inend'.
            if new_pos < inend {
                zopfli_update_hash(input, new_pos, inend, h);
            }
        }

        // Move the current position forward by the length of the match or literal.
        pos += length as usize;
    }
}


/// Performs a single run for `ZopfliLZ77Optimal`. For good compression, repeated runs
/// with updated statistics should be performed.
///
/// # Arguments
///
/// * `s` - The block state (`ZopfliBlockState`).
/// * `input` - The input data array (`&[u8]`).
/// * `instart` - The starting index in the input array (`usize`).
/// * `inend` - The ending index in the input array (`usize`), not inclusive.
/// * `path` - Mutable reference to a vector to store the dynamically allocated path (`&mut Vec<u16>`).
/// * `pathsize` - Mutable reference to store the size of the dynamic path array (`&mut usize`).
/// * `length_array` - Mutable slice used to store lengths, with size `(inend - instart)` (`&mut [u16]`).
/// * `costmodel` - Reference to a function that calculates the cost model for this squeeze run (`&dyn Fn(u32, u32, &dyn std::any::Any) -> f64`).
/// * `costcontext` - Abstract context for the cost model function (`&dyn std::any::Any`).
/// * `store` - Mutable reference to store the LZ77 data (`&mut ZopfliLZ77Store`).
/// * `h` - Mutable reference to the hash structure (`&mut ZopfliHash`).
/// * `costs` - Mutable slice to store the costs (`&mut [f32]`).
///
/// # Returns
///
/// The cost that was, according to the `costmodel`, needed to get to the end.
/// This is not the actual cost.
///
/// # Panics
///
/// This function will panic if:
/// - Any cost calculated is negative.
/// - Array indexing goes out of bounds.
/// - Any assertions in helper functions fail.
///
/// # Example
///
/// ```
/// let mut block_state = ZopfliBlockState { /* initialize fields */ };
/// let input_data = vec![/* input bytes */];
/// let mut path = Vec::new();
/// let mut pathsize = 0;
/// let mut length_array = vec![0u16; input_data.len()];
/// let cost_model = |litlen: u32, dist: u32, context: &dyn std::any::Any| -> f64 {
///     // Define cost calculation based on context
///     0.0
/// };
/// let cost_context = /* initialize context */;
/// let mut store = ZopfliLZ77Store { /* initialize fields */ };
/// let mut hash = ZopfliHash { /* initialize fields */ };
/// let mut costs = vec![ZOPFLI_LARGE_FLOAT as f32; input_data.len()];
///
/// let total_cost = lz77_optimal_run(
///     &mut block_state,
///     &input_data,
///     0,
///     input_data.len(),
///     &mut path,
///     &mut pathsize,
///     &mut length_array,
///     &cost_model,
///     &cost_context,
///     &mut store,
///     &mut hash,
///     &mut costs,
/// );
/// ```
pub fn lz77_optimal_run(
    s: &mut ZopfliBlockState,
    input: &[u8],
    instart: usize,
    inend: usize,
    path: &mut Vec<u16>,
    pathsize: &mut usize,
    length_array: &mut [u16],
    costmodel: &dyn Fn(u32, u32, &dyn std::any::Any) -> f64,
    costcontext: &dyn std::any::Any,
    store: &mut ZopfliLZ77Store,
    h: &mut ZopfliHash,
    costs: &mut [f32],
) -> f64 {
    // Perform the forward pass to get the best lengths for each position.
    let cost = get_best_lengths(
        s,
        input,
        instart,
        inend,
        costmodel,
        costcontext,
        length_array,
        h,
        costs,
    );

    // Reset the path vector and path size before tracing back.
    path.clear();
    *pathsize = 0;

    // Trace backwards from the end to determine the optimal path of lengths.
    trace_backwards(inend - instart, length_array, path, pathsize);

    // Follow the determined path to populate the LZ77 store with literal and distance pairs.
    zopfli_follow_path(
        s,
        input,
        instart,
        inend,
        if *pathsize > 0 { Some(&path[..*pathsize]) } else { None },
        *pathsize,
        store,
        h,
    );

    // Return the total cost computed by the cost model.
    cost
}


/// Calculates literal/length and distance pairs for given data.
/// If `instart` is larger than 0, it uses values before `instart` as a starting dictionary.
///
/// # Arguments
///
/// * `s` - Mutable reference to `ZopfliBlockState`.
/// * `input` - Slice of input data bytes to be compressed.
/// * `instart` - Starting index in the input data to begin compression.
/// * `inend` - Ending index in the input data (not inclusive).
/// * `numiterations` - Maximum number of iterations to optimize compression.
/// * `store` - Mutable reference to `ZopfliLZ77Store` where the result will be stored.
///
/// # Notes
///
/// This function performs LZ77 compression using an optimal algorithm similar to gzip,
/// with multiple iterations to optimize the compression cost. It maintains statistics
/// across iterations to refine the compression strategy.
///
/// # Example
///
/// ```
/// let mut block_state = ZopfliBlockState {
///     options: ZopfliOptions {
///         verbose: false,
///         verbose_more: false,
///         numiterations: 15,
///         blocksplitting: 1,
///         blocksplittinglast: 0,
///         blocksplittingmax: 15,
///     },
///     lmc: None,
///     blockstart: 0,
///     blockend: 0,
/// };
/// let input_data = b"Example input data for compression.";
/// let mut store = ZopfliLZ77Store {
///     litlens: Vec::new(),
///     dists: Vec::new(),
///     size: 0,
///     data: Vec::new(),
///     pos: Vec::new(),
///     ll_symbol: Vec::new(),
///     d_symbol: Vec::new(),
///     ll_counts: Vec::new(),
///     d_counts: Vec::new(),
/// };
/// zopfli_lz77_optimal(&mut block_state, input_data, 0, input_data.len(), 10, &mut store);
/// ```
pub fn zopfli_lz77_optimal(
    s: &mut ZopfliBlockState,    // Mutable reference to the block state
    input: &[u8],                // Slice of input data bytes
    instart: usize,              // Start index for compression
    inend: usize,                // End index for compression (not inclusive)
    numiterations: i32,          // Number of iterations for optimization
    store: &mut ZopfliLZ77Store, // Mutable reference to store the compression result
) {
    // Calculate the size of the block to be compressed
    let blocksize = inend - instart;

    // Allocate the `length_array` with size (blocksize + 1), initialized to 0
    let mut length_array = vec![0u16; blocksize + 1];

    // Initialize the `path` vector and `pathsize`
    let mut path: Vec<u16> = Vec::new();
    let mut pathsize: usize = 0;

    // Initialize the current LZ77 store
    let mut currentstore = ZopfliLZ77Store {
        litlens: Vec::new(),
        dists: Vec::new(),
        size: 0,
        data: input.to_vec(),
        pos: Vec::new(),
        ll_symbol: Vec::new(),
        d_symbol: Vec::new(),
        ll_counts: Vec::new(),
        d_counts: Vec::new(),
    };
    // Initialize the store with input data
    ZopfliInitLZ77Store(input, &mut currentstore);

    // Initialize the hash structure
    let mut hash = ZopfliHash {
        head: Vec::new(),
        prev: Vec::new(),
        hashval: Vec::new(),
        val: 0,
        head2: Vec::new(),
        prev2: Vec::new(),
        hashval2: Vec::new(),
        val2: 0,
        same: Vec::new(),
    };
    // Allocate memory for the hash structure
    zopfli_alloc_hash(ZOPFLI_WINDOW_SIZE, &mut hash);

    // Initialize symbol statistics
    let mut stats = SymbolStats {
        litlens: [0usize; ZOPFLI_NUM_LL],
        dists: [0usize; ZOPFLI_NUM_D],
        ll_symbols: [0.0; ZOPFLI_NUM_LL],
        d_symbols: [0.0; ZOPFLI_NUM_D],
    };
    init_stats(&mut stats);

    // Variables to keep track of the best and last statistics
    let mut beststats = SymbolStats {
        litlens: [0usize; ZOPFLI_NUM_LL],
        dists: [0usize; ZOPFLI_NUM_D],
        ll_symbols: [0.0; ZOPFLI_NUM_LL],
        d_symbols: [0.0; ZOPFLI_NUM_D],
    };
    let mut laststats = SymbolStats {
        litlens: [0usize; ZOPFLI_NUM_LL],
        dists: [0usize; ZOPFLI_NUM_D],
        ll_symbols: [0.0; ZOPFLI_NUM_LL],
        d_symbols: [0.0; ZOPFLI_NUM_D],
    };

    // Initialize the random state
    let mut ran_state = RanState {
        m_w: 1, // Initialized to 1 as per `InitRanState`
        m_z: 2, // Initialized to 2 as per `InitRanState`
    };

    // Perform the initial greedy LZ77 compression
    zopfli_lz77_greedy(s, input, instart, inend, &mut currentstore, &mut hash);

    // Gather statistics from the initial compression store
    get_statistics(&currentstore, &mut stats);

    // Initialize the costs array with a large float value
    let mut costs = vec![ZOPFLI_LARGE_FLOAT as f32; blocksize + 1];

    // Initialize variables to track the best cost and last cost
    let mut bestcost = ZOPFLI_LARGE_FLOAT;
    let mut lastcost = 0.0;
    let mut lastrandomstep: i32 = -1;

    // Iterate to optimize the LZ77 compression
    for i in 0..numiterations {
        // Clean the current LZ77 store to prepare for the next iteration
        ZopfliCleanLZ77Store(&mut currentstore);
        // Reinitialize the store with the input data
        ZopfliInitLZ77Store(input, &mut currentstore);

        // Define a closure that matches the expected CostModelFun signature
        let costmodel_wrapper = |litlen: u32, dist: u32, context: &dyn std::any::Any| -> f64 {
            // Downcast the context to &SymbolStats
            let stats_ref = context.downcast_ref::<SymbolStats>().unwrap();
            // Call the existing zopfli_get_cost_stat with converted types
            zopfli_get_cost_stat(litlen as i32, dist as i32, stats_ref)
        };

        // Perform an optimal LZ77 compression run
        let _cost = lz77_optimal_run(
            s,                     // Mutable reference to the block state
            input,                 // Input data slice
            instart,               // Start index
            inend,                 // End index
            &mut path,             // Mutable reference to the path vector
            &mut pathsize,         // Mutable reference to the path size
            &mut length_array,     // Mutable reference to the length array
            &costmodel_wrapper,    // Reference to the cost model closure
            &stats,                // Reference to the statistics context as `&dyn Any`
            &mut currentstore,     // Mutable reference to the current LZ77 store
            &mut hash,             // Mutable reference to the hash structure
            &mut costs,            // Mutable reference to the costs array
        );

        // Calculate the block size in bits for the current compression store
        let block_size = zopfli_calculate_block_size(&currentstore, 0, currentstore.size, 2);

        // If verbose output is enabled, print iteration details
        if s.options.verbose_more || (s.options.verbose && block_size < bestcost) {
            eprintln!("Iteration {}: {} bit", i, block_size as i32);
        }

        // If the current block size is better than the best cost so far, update the best store
        if block_size < bestcost {
            // Copy the current store to the best store
            ZopfliCopyLZ77Store(&currentstore, store);
            // Copy the current statistics to the best statistics
            copy_stats(&stats, &mut beststats);
            // Update the best cost
            bestcost = block_size;
        }

        // Update the last statistics with the current statistics
        copy_stats(&stats, &mut laststats);
        // Clear the current statistics frequencies
        clear_stat_freqs(&mut stats);
        // Gather new statistics from the current compression store
        get_statistics(&currentstore, &mut stats);

        // If the random step has been triggered, adjust statistics with weighted frequencies
        if lastrandomstep != -1 {
            // Create a temporary SymbolStats to hold the weighted frequencies
            let mut temp_stats = SymbolStats {
                litlens: [0usize; ZOPFLI_NUM_LL],
                dists: [0usize; ZOPFLI_NUM_D],
                ll_symbols: [0.0; ZOPFLI_NUM_LL],
                d_symbols: [0.0; ZOPFLI_NUM_D],
            };
            // Add weighted statistics to the temporary stats
            add_weighed_stat_freqs(&stats, 1.0, &laststats, 0.5, &mut temp_stats);
            // Copy the temporary stats back to the main stats
            copy_stats(&temp_stats, &mut stats);
            // Recalculate the entropy based on the updated stats
            calculate_statistics(&mut stats);
        }

        // If compression has stabilized, introduce randomness to escape local minima
        if i > 5 && block_size == lastcost {
            // Copy the best statistics back to the current statistics
            copy_stats(&beststats, &mut stats);
            // Randomize the statistics frequencies to diversify the compression path
            randomize_stat_freqs(&mut ran_state, &mut stats);
            // Recalculate the entropy based on the randomized frequencies
            calculate_statistics(&mut stats);
            // Record the step at which randomness was introduced
            lastrandomstep = i;
        }

        // Update the last cost to the current block size
        lastcost = block_size;
    }

    // After all iterations, clean up the current store and hash structure
    ZopfliCleanLZ77Store(&mut currentstore);
    zopfli_clean_hash(&mut hash);
}


/// Performs LZ77 optimal encoding optimized for the fixed tree of the DEFLATE standard.
///
/// This function operates similarly to `lz77_optimal_run`, but it is specifically tailored
/// for use with the fixed Huffman tree defined by the DEFLATE specification. While the fixed
/// tree does not provide the best possible compression, this function ensures the optimal
/// LZ77 encoding within the constraints of the fixed tree. Notably, this function does not
/// generate or output any Huffman tree data; it solely focuses on producing LZ77 data optimized
/// for the fixed tree.
///
/// If `instart` is greater than 0, the function utilizes data preceding `instart` as the
/// starting dictionary for LZ77 matching.
///
/// # Arguments
///
/// * `s` - Mutable reference to the block state (`ZopfliBlockState`).
/// * `input` - Slice of input data bytes to be compressed.
/// * `instart` - Starting index in the input data from where compression begins.
/// * `inend` - Ending index in the input data where compression ends (exclusive).
/// * `store` - Mutable reference to the LZ77 store (`ZopfliLZ77Store`) where the
///             resulting literals, lengths, and distances will be stored.
///
/// # Panics
///
/// This function will panic if memory allocation for temporary arrays fails.
///
/// # Example
///
/// ```rust
/// let mut block_state = ZopfliBlockState {
///     options: ZopfliOptions { /* initialize options */ },
///     lmc: Some(ZopfliLongestMatchCache { /* initialize cache */ }),
///     blockstart: 0,
///     blockend: 0,
/// };
/// let input_data = vec![/* input bytes */];
/// let mut store = ZopfliLZ77Store { /* initialize store */ };
/// zopfli_lz77_optimal_fixed(&mut block_state, &input_data, 0, input_data.len(), &mut store);
/// ```
pub fn zopfli_lz77_optimal_fixed(
    s: &mut ZopfliBlockState,
    input: &[u8],
    instart: usize,
    inend: usize,
    store: &mut ZopfliLZ77Store,
) {
    // Calculate the size of the block to be processed.
    let blocksize = inend - instart;

    // Allocate a vector to store the optimal lengths for each position in the block.
    // The vector size is `blocksize + 1` to accommodate the end position.
    let mut length_array: Vec<u16> = vec![0; blocksize + 1];

    // Initialize the path vector to store the optimal path of lengths.
    let mut path: Vec<u16> = Vec::new();

    // Initialize the path size to zero.
    let mut pathsize: usize = 0;

    // Create a new ZopfliHash instance for hashing operations.
    let mut hash = ZopfliHash {
        head: Vec::new(),
        prev: Vec::new(),
        hashval: Vec::new(),
        val: 0,
        head2: Vec::new(),
        prev2: Vec::new(),
        hashval2: Vec::new(),
        val2: 0,
        same: Vec::new(),
    };

    // Allocate the hash tables with the predefined window size.
    zopfli_alloc_hash(ZOPFLI_WINDOW_SIZE, &mut hash);

    // Allocate a vector to store the costs for each position in the block.
    // Initialized with zeros and sized `blocksize + 1`.
    let mut costs: Vec<f32> = vec![0.0; blocksize + 1];

    // Update the block start and end indices in the block state.
    s.blockstart = instart;
    s.blockend = inend;

    // Define a closure that wraps `GetCostFixed`, adapting it to the expected signature.
    let cost_model = |litlen: u32, dist: u32, _context: &dyn std::any::Any| -> f64 {
        // Since `GetCostFixed` expects a `*mut c_void`, we can pass a null pointer
        // because the parameter is unused within the function.
        GetCostFixed(litlen, dist, std::ptr::null_mut())
    };

    // Perform a single optimal run using the fixed cost model.
    // This computes the optimal lengths and populates the `length_array`.
    let _total_cost = lz77_optimal_run(
        s,
        input,
        instart,
        inend,
        &mut path,
        &mut pathsize,
        &mut length_array,
        &cost_model,
        &(), // Passing an empty tuple as the cost context since it's unused.
        store,
        &mut hash,
        &mut costs,
    );

    // Clean up the allocated hash tables to free memory.
    zopfli_clean_hash(&mut hash);

    // Note: In Rust, memory is automatically managed, so explicit deallocation (like `free` in C)
    // is not necessary. Vectors will be dropped when they go out of scope.

    // The `_total_cost` variable holds the cost computed by `lz77_optimal_run`.
    // It is currently unused, but it can be utilized if needed for further processing.
}


/// Adds a dynamic Huffman tree by selecting the optimal combination of repeat codes.
///
/// This function iterates through all possible combinations of the `use_16`, `use_17`, and `use_18` flags
/// to determine the combination that results in the smallest encoded tree size. It first calls
/// `encode_tree` with each combination in size-only mode to calculate the size without generating the
/// actual output. After finding the best combination, it calls `encode_tree` again with the selected
/// flags to generate the encoded tree.
///
/// # Arguments
///
/// * `ll_lengths` - A slice of unsigned integers representing the bit lengths of the literal/length Huffman tree.
/// * `d_lengths` - A slice of unsigned integers representing the bit lengths of the distance Huffman tree.
/// * `bp` - A mutable reference to the current bit position within the last byte of the output.
/// * `out` - A mutable reference to the output buffer (`Vec<u8>`) where the encoded tree bits will be stored.
/// * `outsize` - A mutable reference to the current size of the `out` buffer.
///
/// # Example
///
/// ```rust
/// let ll_lengths = vec![...];
/// let d_lengths = vec![...];
/// let mut bp: u8 = 0;
/// let mut out: Vec<u8> = Vec::new();
/// let mut outsize: usize = 0;
/// add_dynamic_tree(&ll_lengths, &d_lengths, &mut bp, &mut out, &mut outsize);
/// ```
pub fn add_dynamic_tree(
    ll_lengths: &[u32],
    d_lengths: &[u32],
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Initialize variables to track the best combination of flags.
    let mut best = 0;
    let mut bestsize = 0;

    // Iterate over all possible combinations of use_16, use_17, and use_18 flags (0 to 7).
    for i in 0..8 {
        // Determine the flags based on the current combination.
        let use_16 = (i & 1) as i32;
        let use_17 = ((i & 2) >> 1) as i32;
        let use_18 = ((i & 4) >> 2) as i32;

        // Call `encode_tree` in size-only mode to calculate the size without generating output.
        let size = encode_tree(
            ll_lengths,
            d_lengths,
            use_16,
            use_17,
            use_18,
            None,
            None,
            None,
        );

        // Update the best combination if this size is smaller than the current best.
        if bestsize == 0 || size < bestsize {
            bestsize = size;
            best = i;
        }
    }

    // Extract the best combination of flags.
    let best_use_16 = (best & 1) as i32;
    let best_use_17 = ((best & 2) >> 1) as i32;
    let best_use_18 = ((best & 4) >> 2) as i32;

    // Call `encode_tree` again with the best flags to generate the encoded tree.
    encode_tree(
        ll_lengths,
        d_lengths,
        best_use_16,
        best_use_17,
        best_use_18,
        Some(bp),
        Some(out),
        Some(outsize),
    );
}


/// Gets value of the extra bits for the given `dist`, according to the DEFLATE specification.
pub fn zopfli_get_dist_extra_bits_value(dist: i32) -> i32 {
    if dist < 5 {
        // If distance is less than 5, there are no extra bits.
        0
    } else {
        // Calculate the logarithm base 2 of (dist - 1).
        let l = 31 ^ ((dist - 1) as u32).leading_zeros();
        
        // Compute the extra bits value based on the calculated log2 value.
        ((dist - (1 + (1 << l))) & ((1 << (l - 1)) - 1)) as i32
    }
}


/// Gets value of the extra bits for the given length, according to the DEFLATE specification.
///
/// # Arguments
///
/// * `l` - The length for which to retrieve the extra bits value.
///
/// # Returns
///
/// Returns the extra bits value corresponding to the provided length.
///
/// # Panics
///
/// Panics if `l` is out of bounds (i.e., not between 0 and 258 inclusive).
pub fn zopfli_get_length_extra_bits_value(l: i32) -> i32 {
    // Static lookup table containing extra bits values as per the DEFLATE specification.
    const TABLE: [i32; 259] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
        1, 0, 1, 0, 1, 2, 3, 0, 
        1, 2, 3, 0, 1, 2, 3, 0, 
        1, 2, 3, 0, 1, 2, 3, 4, 
        5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 
        1, 2, 3, 4, 5, 6, 7, 0, 
        1, 2, 3, 4, 5, 6, 7, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 
        9,10,11,12,13,14,15, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18, 
       19,20,21,22,23,24,25,26,27,28,29,30,31, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18, 
       19,20,21,22,23,24,25,26,27,28,29,30,31, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18, 
       19,20,21,22,23,24,25,26,27,28,29,30,31, 0, 
        1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18, 
       19,20,21,22,23,24,25,26,27,28,29,30, 0
    ];

    // Ensure that the input length 'l' is within the valid range.
    assert!(l >= 0 && l < 259, "Length 'l' must be between 0 and 258 inclusive.");

    // Retrieve and return the extra bits value from the table using the provided length.
    TABLE[l as usize]
}


/// Adds all literal/length and distance codes from the lists as Huffman symbols.
/// Does not add the end code 256. `expected_data_size` is the uncompressed block size,
/// used for an assertion, but you can set it to 0 to disable the assertion.
///
/// # Arguments
///
/// * `lz77` - A reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index for processing the LZ77 data.
/// * `lend` - The ending index (exclusive) for processing the LZ77 data.
/// * `expected_data_size` - The expected size of the uncompressed data. Set to 0 to skip the assertion.
/// * `ll_symbols` - A slice containing the literal/length symbols.
/// * `ll_lengths` - A slice containing the lengths of the literal/length Huffman codes.
/// * `d_symbols` - A slice containing the distance symbols.
/// * `d_lengths` - A slice containing the lengths of the distance Huffman codes.
/// * `bp` - A mutable reference to the current bit position within the output byte.
/// * `out` - A mutable reference to the output buffer where bits are being written.
/// * `outsize` - A mutable reference to the current size of the output buffer.
///
/// # Panics
///
/// This function will panic if any of the assertions fail, such as invalid symbol lengths
/// or mismatched expected data sizes.
pub fn add_lz77_data(
    lz77: &ZopfliLZ77Store,
    lstart: usize,
    lend: usize,
    expected_data_size: usize,
    ll_symbols: &[u32],
    ll_lengths: &[u32],
    d_symbols: &[u32],
    d_lengths: &[u32],
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Initialize a counter to keep track of the total length of processed data
    let mut test_length: usize = 0;

    // Iterate over the range from `lstart` to `lend` to process each LZ77 pair
    for i in lstart..lend {
        // Retrieve the distance and literal/length values for the current index
        let dist = lz77.dists[i] as u32;
        let litlen = lz77.litlens[i] as u32;

        if dist == 0 {
            // If `dist` is 0, the current pair represents a literal

            // Ensure that the literal length is less than 256
            assert!(
                litlen < 256,
                "Literal length {} is not less than 256",
                litlen
            );

            // Ensure that the corresponding Huffman code length for the literal is greater than 0
            assert!(
                ll_lengths[litlen as usize] > 0,
                "Huffman length for literal length {} is zero",
                litlen
            );

            // Add the Huffman bits for the literal symbol using the provided functions
            add_huffman_bits(
                ll_symbols[litlen as usize],
                ll_lengths[litlen as usize],
                bp,
                out,
                outsize,
            );

            // Increment the total processed length by 1 for the literal
            test_length += 1;
        } else {
            // If `dist` is greater than 0, the current pair represents a length-distance pair

            // Get the Huffman symbol for the given length using the DEFLATE specification
            let lls = ZopfliGetLengthSymbol(litlen as i32) as usize;

            // Get the Huffman symbol for the given distance using the DEFLATE specification
            let ds = zopfli_get_dist_symbol(dist as i32) as usize;

            // Ensure that the literal length is within the valid range [3, 288]
            assert!(
                litlen >= 3 && litlen <= 288,
                "Literal length {} is out of valid range (3-288)",
                litlen
            );

            // Ensure that the length Huffman code length is greater than 0
            assert!(
                ll_lengths[lls] > 0,
                "Huffman length for length symbol {} is zero",
                lls
            );

            // Ensure that the distance Huffman code length is greater than 0
            assert!(
                d_lengths[ds] > 0,
                "Huffman length for distance symbol {} is zero",
                ds
            );

            // Add the Huffman bits for the length symbol
            add_huffman_bits(
                ll_symbols[lls],
                ll_lengths[lls],
                bp,
                out,
                outsize,
            );

            // Add the extra bits for the length value based on the DEFLATE specification
            add_bits(
                zopfli_get_length_extra_bits_value(litlen as i32) as u32,
                zopfli_get_length_extra_bits(litlen as i32) as u32,
                bp,
                out,
                outsize,
            );

            // Add the Huffman bits for the distance symbol
            add_huffman_bits(
                d_symbols[ds],
                d_lengths[ds],
                bp,
                out,
                outsize,
            );

            // Add the extra bits for the distance value based on the DEFLATE specification
            add_bits(
                zopfli_get_dist_extra_bits_value(dist as i32) as u32,
                zopfli_get_dist_extra_bits(dist as i32) as u32,
                bp,
                out,
                outsize,
            );

            // Increment the total processed length by the literal length
            test_length += litlen as usize;
        }
    }

    // If `expected_data_size` is not zero, ensure that the total processed length matches it
    assert!(
        expected_data_size == 0 || test_length == expected_data_size,
        "Test length {} does not match expected data size {}",
        test_length,
        expected_data_size
    );
}


/// Adds a single bit to the output bitstream.
/// 
/// # Arguments
/// 
/// * `bit` - The bit to add (0 or 1).
/// * `bp` - A mutable reference to the bit pointer, always in the range [0, 7].
/// * `out` - A mutable reference to the output byte vector.
/// * `outsize` - A mutable reference to the current size of the output byte vector.
/// 
/// # Description
/// 
/// This function appends a bit to the `out` vector. If the bit pointer `bp` is 0, 
/// it appends a new byte initialized to 0, effectively doubling the allocation 
/// size if necessary using `zopfli_append_data`. It then sets the appropriate bit 
/// in the last byte of `out` based on the current bit pointer and increments 
/// the bit pointer, ensuring it wraps around within the range [0, 7].
pub fn add_bit(bit: i32, bp: &mut u8, out: &mut Vec<u8>, outsize: &mut usize) {
    // If the bit pointer is at 0, append a new byte (initialized to 0) to `out`.
    if *bp == 0 {
        zopfli_append_data(0u8, out, outsize);
    }

    // Set the bit at the current bit pointer position in the last byte of `out`.
    // `bit` is cast to `u8` to ensure correct bitwise operations.
    out[*outsize - 1] |= (bit as u8) << *bp;

    // Increment the bit pointer and wrap it around to stay within [0, 7].
    *bp = (*bp + 1) & 7;
}


/// Adds a non-compressed block to the output bitstream.
/// Since an uncompressed block can be a maximum of 65535 in size, it actually adds multiple blocks if needed.
/// 
/// # Arguments
/// 
/// * `options` - Reference to `ZopfliOptions`. (Unused in this function)
/// * `final_` - An integer indicating whether this is the final block (1 for true, 0 for false).
/// * `input` - The input byte slice.
/// * `instart` - The starting index in the input slice.
/// * `inend` - The ending index in the input slice. Exclusive upper bound.
/// * `bp` - A mutable reference to the bit pointer, always in the range [0, 7].
/// * `out` - A mutable reference to the output byte vector.
/// * `outsize` - A mutable reference to the current size of the output byte vector.
pub fn add_non_compressed_block(
    options: &ZopfliOptions,
    final_: i32,
    input: &[u8],
    instart: usize,
    inend: usize,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    let mut pos = instart;
    // options is unused in this function.

    // Loop to process and add non-compressed blocks, handling maximum size per block.
    loop {
        let mut blocksize: u16 = 65535; // Maximum size of an uncompressed block.
        // Adjust block size if it exceeds the input size.
        if pos + blocksize as usize > inend {
            blocksize = (inend - pos) as u16;
        }
        // Determine if this is the final block.
        let currentfinal = pos + blocksize as usize >= inend;

        // Calculate nlen, the one's complement of the block size.
        let nlen: u16 = !blocksize;

        // Write the bits for the block header.
        // Add the 'final' bit (1 if final block, 0 otherwise).
        add_bit(
            if final_ != 0 && currentfinal { 1 } else { 0 },
            bp,
            out,
            outsize,
        );
        // Add BTYPE bits '00' indicating an uncompressed block.
        add_bit(0, bp, out, outsize);
        add_bit(0, bp, out, outsize);

        // Any bits of input up to the next byte boundary are ignored.
        *bp = 0;

        // Append the LEN and NLEN fields of the block header.
        // LEN is the length of the block, NLEN is the one's complement of LEN.
        zopfli_append_data((blocksize % 256) as u8, out, outsize);
        zopfli_append_data(((blocksize / 256) % 256) as u8, out, outsize);
        zopfli_append_data((nlen % 256) as u8, out, outsize);
        zopfli_append_data(((nlen / 256) % 256) as u8, out, outsize);

        // Copy the block data from the input to the output.
        for i in 0..blocksize as usize {
            zopfli_append_data(input[pos + i], out, outsize);
        }

        // If this was the final block, break the loop.
        if currentfinal {
            break;
        }
        // Move the position forward by the block size.
        pos += blocksize as usize;
    }
}


/// Adds a DEFLATE block with the given LZ77 data to the output.
///
/// # Arguments
///
/// * `options` - Reference to the global program options (`ZopfliOptions`).
/// * `btype` - The block type, must be `1` (fixed Huffman) or `2` (dynamic Huffman). If `0`, adds a non-compressed block.
/// * `final_` - Indicates whether this is the final block (`1` for true, `0` for false).
/// * `lz77` - Reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index in the LZ77 data to begin adding the block.
/// * `lend` - The ending index in the LZ77 data (exclusive) to end adding the block.
/// * `expected_data_size` - The expected size of the uncompressed data. Set to `0` to disable the assertion.
/// * `bp` - Mutable reference to the current bit pointer (0-7).
/// * `out` - Mutable reference to the output byte vector where the block will be appended.
/// * `outsize` - Mutable reference to the current size of the output byte vector.
///
/// # Panics
///
/// This function will panic if:
/// - `btype` is not `0`, `1`, or `2`.
/// - Any of the Huffman lengths are invalid.
/// - The expected data size does not match the actual data size (when `expected_data_size` is not `0`).
pub fn add_lz77_block(
    options: &ZopfliOptions,
    btype: i32,
    final_: i32,
    lz77: &ZopfliLZ77Store,
    lstart: usize,
    lend: usize,
    expected_data_size: usize,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Initialize vectors for literal/length and distance bit lengths and symbols
    let mut ll_lengths = vec![0u32; ZOPFLI_NUM_LL];
    let mut d_lengths = vec![0u32; ZOPFLI_NUM_D];
    let mut ll_symbols = vec![0u32; ZOPFLI_NUM_LL];
    let mut d_symbols = vec![0u32; ZOPFLI_NUM_D];

    // Initialize variables to track sizes
    let compressed_size: usize;
    let mut uncompressed_size: usize = 0;

    // Handle non-compressed block if btype is 0
    if btype == 0 {
        // Get the total byte range for the specified LZ77 symbols
        let length = zopfli_lz77_get_byte_range(lz77, lstart, lend);
        // Determine the starting position in the original data
        let pos = if lstart == lend { 0 } else { lz77.pos[lstart] };
        // Calculate the end position based on the length
        let end = pos + length;
        // Add a non-compressed block to the output
        add_non_compressed_block(
            options,
            final_,
            &lz77.data,
            pos,
            end,
            bp,
            out,
            outsize,
        );
        return;
    }

    // Add the 'final' bit to the output
    add_bit(final_, bp, out, outsize);
    // Add the first bit of btype
    add_bit((btype & 1) as i32, bp, out, outsize);
    // Add the second bit of btype
    add_bit(((btype & 2) >> 1) as i32, bp, out, outsize);

    if btype == 1 {
        // Fixed Huffman block
        // Use the existing `get_fixed_tree` function, which accepts `&mut Vec<u32>`
        get_fixed_tree(&mut ll_lengths, &mut d_lengths);
    } else {
        // Dynamic Huffman block
        // Ensure that btype is 2
        assert!(
            btype == 2,
            "Invalid block type: expected 1 or 2, got {}",
            btype
        );

        // Calculate dynamic Huffman lengths
        // Note: `get_dynamic_lengths` expects arrays `[u32; ZOPFLI_NUM_LL]` and `[u32; ZOPFLI_NUM_D]`
        // We need to convert our vectors to fixed-size arrays for this function
        let mut ll_lengths_array = [0u32; ZOPFLI_NUM_LL];
        let mut d_lengths_array = [0u32; ZOPFLI_NUM_D];
        ll_lengths_array.copy_from_slice(&ll_lengths);
        d_lengths_array.copy_from_slice(&d_lengths);

        let _tree_size = get_dynamic_lengths(
            lz77,
            lstart,
            lend,
            &mut ll_lengths_array,
            &mut d_lengths_array,
        );

        // Copy the calculated lengths back to the vectors
        ll_lengths.copy_from_slice(&ll_lengths_array);
        d_lengths.copy_from_slice(&d_lengths_array);

        // Store the current output size before adding the dynamic tree
        let detect_tree_size = *outsize;
        // Add the dynamic Huffman tree to the output
        add_dynamic_tree(
            &ll_lengths,
            &d_lengths,
            bp,
            out,
            outsize,
        );

        // If verbose output is enabled, print the tree size
        if options.verbose {
            eprintln!(
                "treesize: {}",
                (*outsize - detect_tree_size) as i32
            );
        }
    }

    // Convert the bit lengths to Huffman symbols for literal/length and distance codes
    zopfli_lengths_to_symbols(&ll_lengths, ZOPFLI_NUM_LL, 15, &mut ll_symbols);
    zopfli_lengths_to_symbols(&d_lengths, ZOPFLI_NUM_D, 15, &mut d_symbols);

    // Store the current output size before adding LZ77 data
    let detect_block_size = *outsize;
    // Add the LZ77 data to the output using the Huffman symbols and lengths
    add_lz77_data(
        lz77,
        lstart,
        lend,
        expected_data_size,
        &ll_symbols,
        &ll_lengths,
        &d_symbols,
        &d_lengths,
        bp,
        out,
        outsize,
    );

    // Add the end symbol (256) to signify the end of the block
    add_huffman_bits(
        ll_symbols[256],
        ll_lengths[256],
        bp,
        out,
        outsize,
    );

    // Calculate the total uncompressed size by iterating over the LZ77 symbols
    for i in lstart..lend {
        uncompressed_size += if lz77.dists[i] == 0 {
            1
        } else {
            lz77.litlens[i] as usize
        };
    }

    // Calculate the compressed size by subtracting the block size from the current output size
    compressed_size = *outsize - detect_block_size;

    // If verbose output is enabled, print the compressed block size and uncompressed size
    if options.verbose {
        eprintln!(
            "compressed block size: {} ({}k) (unc: {})",
            compressed_size as i32,
            (compressed_size / 1024) as i32,
            uncompressed_size as i32
        );
    }
}


/// Automatically selects the optimal block type (uncompressed, fixed Huffman, or dynamic Huffman)
/// for the given LZ77 data and adds the corresponding DEFLATE block to the output.
///
/// This function evaluates the cost of representing the block as uncompressed, using fixed
/// Huffman codes, or using dynamic Huffman codes. It selects the most efficient type based
/// on the calculated costs and appends the appropriate block to the output.
///
/// # Arguments
///
/// * `options` - Reference to the global program options (`ZopfliOptions`).
/// * `final_` - Indicates whether this is the final block (`1` for true, `0` for false).
/// * `lz77` - Reference to the `ZopfliLZ77Store` containing the LZ77 data.
/// * `lstart` - The starting index in the LZ77 data to begin adding the block.
/// * `lend` - The ending index in the LZ77 data (exclusive) to end adding the block.
/// * `expected_data_size` - The expected size of the uncompressed data. Set to `0` to disable the assertion.
/// * `bp` - Mutable reference to the current bit pointer (0-7).
/// * `out` - Mutable reference to the output byte vector where the block will be appended.
/// * `outsize` - Mutable reference to the current size of the output byte vector.
///
/// # Notes
///
/// This function relies on other functions such as `zopfli_calculate_block_size`, `ZopfliInitLZ77Store`,
/// `zopfli_lz77_get_byte_range`, `zopfli_init_block_state`, `zopfli_lz77_optimal_fixed`,
/// `ZopfliCleanBlockState`, and `add_lz77_block`, which should be defined in the existing code.
///
/// # Panics
///
/// This function will panic if any of the underlying functions it calls panic.
///
/// # Example
///
/// ```rust
/// let options = ZopfliOptions {
///     verbose: false,
///     verbose_more: false,
///     numiterations: 15,
///     blocksplitting: 1,
///     blocksplittinglast: 0,
///     blocksplittingmax: 15,
/// };
/// let lz77 = ZopfliLZ77Store {
///     litlens: vec![/* ... */],
///     dists: vec![/* ... */],
///     size: 0,
///     data: vec![/* ... */],
///     pos: vec![/* ... */],
///     ll_symbol: vec![/* ... */],
///     d_symbol: vec![/* ... */],
///     ll_counts: vec![/* ... */],
///     d_counts: vec![/* ... */],
/// };
/// let mut bp: u8 = 0;
/// let mut out: Vec<u8> = Vec::new();
/// let mut outsize: usize = 0;
/// add_lz77_block_auto_type(
///     &options,
///     1,
///     &lz77,
///     0,
///     lz77.litlens.len(),
///     0,
///     &mut bp,
///     &mut out,
///     &mut outsize,
/// );
/// ```
pub fn add_lz77_block_auto_type(
    options: &ZopfliOptions,
    final_: i32,
    lz77: &ZopfliLZ77Store,
    lstart: usize,
    lend: usize,
    expected_data_size: usize,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Calculate the size in bits for an uncompressed block (btype = 0).
    let uncompressed_cost = zopfli_calculate_block_size(lz77, lstart, lend, 0);
    
    // Calculate the size in bits for a block with fixed Huffman codes (btype = 1).
    let mut fixed_cost = zopfli_calculate_block_size(lz77, lstart, lend, 1);
    
    // Calculate the size in bits for a block with dynamic Huffman codes (btype = 2).
    let dyn_cost = zopfli_calculate_block_size(lz77, lstart, lend, 2);

    // Determine whether to perform the expensive calculation of creating an optimal block
    // with a fixed Huffman tree. This is done only for small blocks or blocks that already
    // have a good compression ratio with the fixed Huffman tree.
    let expensive_fixed = (lz77.size < 1000) || (fixed_cost <= dyn_cost * 1.1);

    // Initialize a fixedstore to hold LZ77 data if needed.
    let mut fixedstore = ZopfliLZ77Store {
        litlens: Vec::new(),
        dists: Vec::new(),
        size: 0,
        data: lz77.data.clone(),
        pos: Vec::new(),
        ll_symbol: Vec::new(),
        d_symbol: Vec::new(),
        ll_counts: Vec::new(),
        d_counts: Vec::new(),
    };

    // Check if the block range is empty. An empty block is represented by a fixed block.
    if lstart == lend {
        // Add the 'final' bit to the output.
        add_bits(final_ as u32, 1, bp, out, outsize);
        
        // Add the block type bits. btype = 01 for fixed Huffman codes.
        add_bits(1, 2, bp, out, outsize);
        
        // Add the end symbol with fixed Huffman code (7 bits of 0).
        add_bits(0, 7, bp, out, outsize);
        
        // Return early since the block is empty.
        return;
    }

    // Initialize the fixedstore with the original data.
    ZopfliInitLZ77Store(&lz77.data, &mut fixedstore);

    // If an expensive fixed Huffman calculation is warranted, recalculate the LZ77 data.
    if expensive_fixed {
        // Determine the starting and ending positions in the original data.
        let instart = lz77.pos[lstart];
        let inend = instart + zopfli_lz77_get_byte_range(lz77, lstart, lend);

        // Initialize the block state with the given options and block range.
        let mut block_state = ZopfliBlockState {
            options: options.clone(),
            lmc: None,
            blockstart: instart,
            blockend: inend,
        };
        zopfli_init_block_state(options, instart, inend, 1, &mut block_state);
        
        // Perform the optimal fixed Huffman LZ77 encoding.
        zopfli_lz77_optimal_fixed(
            &mut block_state,
            &lz77.data,
            instart,
            inend,
            &mut fixedstore,
        );
        
        // Recalculate the fixed cost based on the optimized LZ77 data.
        fixed_cost = zopfli_calculate_block_size(&fixedstore, 0, fixedstore.size, 1);
        
        // Clean up the block state.
        ZopfliCleanBlockState(&mut block_state);
    }

    // Determine the most efficient block type based on the calculated costs.
    if uncompressed_cost < fixed_cost && uncompressed_cost < dyn_cost {
        // If uncompressed is the cheapest, add an uncompressed block.
        add_lz77_block(
            options,
            0, // btype = 0 for uncompressed
            final_,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    } else if fixed_cost < dyn_cost {
        // If fixed Huffman is cheaper than dynamic, add a fixed Huffman block.
        if expensive_fixed {
            // If an expensive fixed calculation was performed, use the optimized fixedstore.
            add_lz77_block(
                options,
                1, // btype = 1 for fixed Huffman
                final_,
                &fixedstore,
                0,
                fixedstore.size,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        } else {
            // Otherwise, use the original LZ77 data.
            add_lz77_block(
                options,
                1, // btype = 1 for fixed Huffman
                final_,
                lz77,
                lstart,
                lend,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        }
    } else {
        // If dynamic Huffman is the cheapest, add a dynamic Huffman block.
        add_lz77_block(
            options,
            2, // btype = 2 for dynamic Huffman
            final_,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    }

    // Clean up the fixedstore to free allocated memory.
    ZopfliCleanLZ77Store(&mut fixedstore);
}


/// Deflate a part of the input data, allowing `zopfli_deflate` to use multiple master blocks if needed.
///
/// It is possible to call this function multiple times in a row, shifting `instart` and `inend` to next bytes of the data.
/// If `instart` is larger than 0, then previous bytes are used as the initial dictionary for LZ77.
/// This function will usually output multiple deflate blocks. If `final_` is 1, then the final bit will be set on the last block.
///
/// # Arguments
///
/// * `options` - Reference to the `ZopfliOptions` containing compression options.
/// * `btype` - The block type to use (`0`, `1`, or `2`).
/// * `final_` - Whether this is the final block (`1` for true, `0` for false).
/// * `input` - The input data slice.
/// * `instart` - The starting index in the input data.
/// * `inend` - The ending index in the input data (exclusive).
/// * `bp` - Mutable reference to the bit pointer (`0`-`7`).
/// * `out` - Mutable reference to the output byte vector.
/// * `outsize` - Mutable reference to the current size of the output byte vector.
///
/// # Notes
///
/// This function uses LZ77 compression and may split the data into multiple blocks for better compression efficiency.
///
/// # Panics
///
/// This function will panic if any of the underlying functions it calls panic.
///
/// # Example
///
/// ```rust
/// let options = ZopfliOptions {
///     verbose: false,
///     verbose_more: false,
///     numiterations: 15,
///     blocksplitting: 1,
///     blocksplittinglast: 0,
///     blocksplittingmax: 15,
/// };
/// let btype = 2;
/// let final_ = 1;
/// let input = vec![/* input bytes */];
/// let instart = 0;
/// let inend = input.len();
/// let mut bp: u8 = 0;
/// let mut out: Vec<u8> = Vec::new();
/// let mut outsize: usize = 0;
/// 
/// zopfli_deflate_part(
///     &options,
///     btype,
///     final_,
///     &input,
///     instart,
///     inend,
///     &mut bp,
///     &mut out,
///     &mut outsize,
/// );
/// ```
pub fn zopfli_deflate_part(
    options: &ZopfliOptions,
    btype: i32,
    final_: i32,
    input: &[u8],
    instart: usize,
    inend: usize,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Declare variables
    // `splitpoints_uncompressed`: Coordinates in uncompressed data where to split
    let mut splitpoints_uncompressed: Vec<usize> = Vec::new();
    let mut npoints: usize = 0;
    // `splitpoints`: Indices in LZ77 data where to split
    let mut splitpoints: Vec<usize> = Vec::new();
    let mut totalcost: f64 = 0.0;

    // Initialize the main LZ77 store with the input data
    let mut lz77 = ZopfliLZ77Store {
        litlens: Vec::new(),
        dists: Vec::new(),
        size: 0,
        data: input.to_vec(),
        pos: Vec::new(),
        ll_symbol: Vec::new(),
        d_symbol: Vec::new(),
        ll_counts: Vec::new(),
        d_counts: Vec::new(),
    };

    // If `btype=2` is specified, it tries all block types.
    // If a lesser `btype` is given, it forces that one. Neither of the lesser types needs
    // block splitting as they have no dynamic Huffman trees.
    if btype == 0 {
        // Add a non-compressed block and return
        add_non_compressed_block(options, final_, input, instart, inend, bp, out, outsize);
        return;
    } else if btype == 1 {
        // Initialize a temporary LZ77 store and block state
        let mut store = ZopfliLZ77Store {
            litlens: Vec::new(),
            dists: Vec::new(),
            size: 0,
            data: input.to_vec(),
            pos: Vec::new(),
            ll_symbol: Vec::new(),
            d_symbol: Vec::new(),
            ll_counts: Vec::new(),
            d_counts: Vec::new(),
        };
        let mut s = ZopfliBlockState {
            options: options.clone(),
            lmc: None,
            blockstart: instart,
            blockend: inend,
        };
        ZopfliInitLZ77Store(input, &mut store);
        zopfli_init_block_state(options, instart, inend, 1, &mut s);

        // Perform optimal LZ77 compression with a fixed Huffman tree
        zopfli_lz77_optimal_fixed(&mut s, input, instart, inend, &mut store);
        // Add the LZ77 block to the output
        add_lz77_block(
            options,
            btype,
            final_,
            &store,
            0,
            store.size,
            0,
            bp,
            out,
            outsize,
        );

        // Clean up the block state and temporary store
        ZopfliCleanBlockState(&mut s);
        ZopfliCleanLZ77Store(&mut store);
        return;
    }

    if options.blocksplitting != 0 {
        // Perform block splitting on uncompressed data
        zopfli_block_split(
            options,
            input,
            instart,
            inend,
            options.blocksplittingmax as usize,
            &mut splitpoints_uncompressed,
            &mut npoints,
        );
        // Initialize `splitpoints` vector with the number of split points
        splitpoints = vec![0usize; npoints];
    }

    // Initialize the main LZ77 store
    ZopfliInitLZ77Store(input, &mut lz77);

    // Compress each block defined by the split points
    for i in 0..=npoints {
        let start = if i == 0 { instart } else { splitpoints_uncompressed[i - 1] };
        let end = if i == npoints { inend } else { splitpoints_uncompressed[i] };
        let mut s = ZopfliBlockState {
            options: options.clone(),
            lmc: None,
            blockstart: start,
            blockend: end,
        };
        let mut store = ZopfliLZ77Store {
            litlens: Vec::new(),
            dists: Vec::new(),
            size: 0,
            data: input.to_vec(),
            pos: Vec::new(),
            ll_symbol: Vec::new(),
            d_symbol: Vec::new(),
            ll_counts: Vec::new(),
            d_counts: Vec::new(),
        };
        ZopfliInitLZ77Store(input, &mut store);
        zopfli_init_block_state(options, start, end, 1, &mut s);
        // Perform optimal LZ77 compression
        zopfli_lz77_optimal(&mut s, input, start, end, options.numiterations, &mut store);
        // Accumulate the total cost
        totalcost += zopfli_calculate_block_size_auto_type(&store, 0, store.size);
        // Append the compressed data to the main LZ77 store
        zopfli_append_lz77_store(&store, &mut lz77);
        if i < npoints {
            splitpoints[i] = lz77.size;
        }
        // Clean up the block state and temporary store
        ZopfliCleanBlockState(&mut s);
        ZopfliCleanLZ77Store(&mut store);
    }

    // Second block splitting attempt
    if options.blocksplitting != 0 && npoints > 1 {
        let mut splitpoints2: Vec<usize> = Vec::new();
        let mut npoints2: usize = 0;
        let mut totalcost2: f64 = 0.0;

        // Perform block splitting on the LZ77 data
        zopfli_block_split_lz77(
            options,
            &lz77,
            options.blocksplittingmax as usize,
            &mut splitpoints2,
            &mut npoints2,
        );

        // Calculate the total cost with the new split points
        for i in 0..=npoints2 {
            let start = if i == 0 { 0 } else { splitpoints2[i - 1] };
            let end = if i == npoints2 { lz77.size } else { splitpoints2[i] };
            totalcost2 += zopfli_calculate_block_size_auto_type(&lz77, start, end);
        }

        // If the new total cost is lower, use the new split points
        if totalcost2 < totalcost {
            splitpoints = splitpoints2;
            npoints = npoints2;
        }
    }

    // Add the final LZ77 blocks to the output
    for i in 0..=npoints {
        let start = if i == 0 { 0 } else { splitpoints[i - 1] };
        let end = if i == npoints { lz77.size } else { splitpoints[i] };
        add_lz77_block_auto_type(
            options,
            if i == npoints && final_ != 0 { 1 } else { 0 },
            &lz77,
            start,
            end,
            0,
            bp,
            out,
            outsize,
        );
    }

    // Clean up the main LZ77 store
    ZopfliCleanLZ77Store(&mut lz77);
}


/// A block structure of huge, non-smart, blocks to divide the input into, to allow
/// operating on huge files without exceeding memory, such as the 1GB wiki9 corpus.
/// The whole compression algorithm, including the smarter block splitting, will
/// be executed independently on each huge block.
/// Dividing into huge blocks hurts compression, but not much relative to the size.
/// Set it to 0 to disable master blocks.
pub const zopfli_master_block_size: usize = 1_000_000;


/// Compresses according to the deflate specification and appends the compressed
/// result to the output. This function will usually output multiple deflate blocks.
/// If `final_` is `1`, then the final bit will be set on the last block.
///
/// # Arguments
///
/// * `options` - Reference to the `ZopfliOptions` containing compression options.
/// * `btype` - The deflate block type. Use `2` for best compression.
///              - `0`: Non-compressed blocks (00)
///              - `1`: Blocks with fixed tree (01)
///              - `2`: Blocks with dynamic tree (10)
/// * `final_` - Whether this is the last section of the input (`1` for true, `0` for false).
/// * `input` - The input data slice.
/// * `insize` - Number of input bytes.
/// * `bp` - Mutable reference to the bit pointer (`0`-`7`). This must initially be `0`,
///          and for consecutive calls must be reused (it can have values from `0`-`7`).
///          This is because deflate appends blocks as bit-based data, rather than on byte boundaries.
/// * `out` - Mutable reference to the output byte vector. The compressed result is appended to this vector.
///          Must be managed (e.g., freed) after use.
/// * `outsize` - Mutable reference to the current size of the output byte vector.
///
/// # Notes
///
/// This function uses master blocks to handle large inputs without exceeding memory.
/// It splits the input into chunks defined by `ZOPFLI_MASTER_BLOCK_SIZE` and compresses each part.
/// If the `verbose` option is enabled, it prints compression statistics.
///
/// # Example
///
/// ```rust
/// let options = ZopfliOptions {
///     verbose: true,
///     verbose_more: false,
///     numiterations: 15,
///     blocksplitting: 1,
///     blocksplittinglast: 0,
///     blocksplittingmax: 15,
/// };
/// let btype = 2;
/// let final_ = 1;
/// let input = vec![/* input bytes */];
/// let insize = input.len();
/// let mut bp: u8 = 0;
/// let mut out: Vec<u8> = Vec::new();
/// let mut outsize: usize = 0;
///
/// zopfli_deflate(
///     &options,
///     btype,
///     final_,
///     &input,
///     insize,
///     &mut bp,
///     &mut out,
///     &mut outsize,
/// );
/// ```
pub fn zopfli_deflate(
    options: &ZopfliOptions,
    btype: i32,
    final_: i32,
    input: &[u8],
    insize: usize,
    bp: &mut u8,
    out: &mut Vec<u8>,
    outsize: &mut usize,
) {
    // Store the initial output size to calculate compression statistics later
    let offset = *outsize;
    let mut i = 0;

    // Loop through the input, processing it in master blocks
    while i < insize {
        // Determine if this is the final master block
        let masterfinal = i + zopfli_master_block_size >= insize;
        // Set the final bit if this is the last master block and `final_` is true
        let final2 = (final_ != 0) && masterfinal;
        // Calculate the size of the current master block
        let size = if masterfinal {
            insize - i
        } else {
            zopfli_master_block_size
        };

        // Compress the current master block
        zopfli_deflate_part(
            options,
            btype,
            final2 as i32,
            input,
            i,
            i + size,
            bp,
            out,
            outsize,
        );

        // Move to the next master block
        i += size;
    }

    // If verbose output is enabled, print compression statistics
    if options.verbose {
        // Calculate the number of bytes compressed in this call
        let compressed_size = *outsize - offset;
        // Calculate the percentage of bytes removed
        // println!("compressed size: {}", compressed_size);
        // println!("insize: {}", insize);
        let compression_percentage = if insize > 0 {
            100.0 * ((insize.wrapping_sub(compressed_size)) as f64) / (insize as f64)
        } else {
            0.0
        };
        // Print the statistics to stderr
        eprintln!(
            "Original Size: {}, Deflate: {}, Compression: {:.2}%",
            insize, compressed_size, compression_percentage
        );
    }
}



pub fn single_test(input: &[u8], btype: i32, blocksplitting: i32, blocksplittingmax: i32){
    let options = ZopfliOptions {
        verbose: true,
        verbose_more: false,
        numiterations: 15,
        blocksplitting: blocksplitting,
        blocksplittinglast: 0,
        blocksplittingmax: blocksplittingmax,
    };

    let final_ = true;
    let insize = input.len();
    let mut bp: u8 = 0;
    let mut out: Vec<u8> = Vec::new();
    let mut outsize: usize = 0;

    zopfli_deflate(
        &options,
        btype,
        1,
        input,
        insize,
        &mut bp,
        &mut out,
        &mut outsize,
    );

}

pub fn run_all_tests(input: &[u8]){
    single_test(input, 2, 1, 15);
    single_test(input, 1, 1, 15);
    single_test(input, 0, 1, 15);

    single_test(input, 2, 0, 15);
    single_test(input, 2, 1, 5);
    single_test(input, 2, 1, 0);
    single_test(input, 2, 1, 1);
    single_test(input, 2, 1, 50);
    single_test(input, 2, 1, 30);
}

use std::io::{Read};

fn read_stdin_to_bytes() -> io::Result<Vec<u8>> {
    // Create a mutable buffer to store the input
    let mut buffer = Vec::new();
    
    // Create a handle to stdin
    let mut stdin = io::stdin();
    
    // Read all bytes from stdin into the buffer
    stdin.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

fn main() {
    match read_stdin_to_bytes() {
        Ok(bytes) => {
            // println!("Read {} bytes", bytes.len());
            // Optionally, you can print or process the bytes here
            // For example, to print as a string (if the input is text):
            // println!("Content: {}", String::from_utf8_lossy(&bytes));
            run_all_tests(&bytes);
        }
        Err(e) => eprintln!("Error reading stdin: {}", e),
    }
}


use std::fs::File;
use std::io::{BufReader};
fn read_input_from_file(file_name: &str) -> io::Result<Vec<u8>> {
    // Create a mutable buffer to store the input
    let mut buffer = Vec::new();
    
    // Open the file in read-only mode with a handle to the file
    let file = File::open(file_name)?;
    
    // Create a buffered reader to read the file
    let mut reader = BufReader::new(file);
    
    // Read all bytes from the file into the buffer
    reader.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use paste::paste;

    const TOTAL_TESTS: usize = 85;
    // const TOTAL_TESTS: usize = 27570;
    const BATCHES: usize = 85;
    const TESTS_PER_BATCH: usize = (TOTAL_TESTS + BATCHES - 1) / BATCHES; // ≈ 172 rounded up


    macro_rules! create_test_batches {
        ($($batch_num:literal),*) => {
            $(
                paste! {
                    #[test]
                    fn [<test_batch_ $batch_num>]() {
                        // Create a shuffled range of all test indices
                        // Immediate flush after each print
                        println!("Running batch {} of {} tests", $batch_num, TOTAL_TESTS);
                        io::stdout().flush().unwrap();
                        eprintln!("Running batch {} of {} tests", $batch_num, TOTAL_TESTS);
                        io::stderr().flush().unwrap();

                        let mut all_indices: Vec<usize> = (0..TOTAL_TESTS).collect();
                        let mut rng = thread_rng();
                        all_indices.shuffle(&mut rng);

                        eprintln!("Running batch {} of {} tests", $batch_num, TOTAL_TESTS);

                        // Calculate batch range
                        let start = $batch_num * TESTS_PER_BATCH;
                        let end = std::cmp::min(($batch_num + 1) * TESTS_PER_BATCH, TOTAL_TESTS);
                        
                        // Process this batch's portion of the shuffled indices
                        all_indices[start..end].iter().for_each(|&i| {
                            let file_path = format!("tests_folder/test_{}", i);
                            println!("Running test for file: {}", file_path);
                            if let Ok(input) = read_input_from_file(&file_path) {
                                run_all_tests(&input);
                            }
                        });
                    }
                }
            )*
        };
    }

    // Create 100 test batches
    create_test_batches!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
        80, 81, 82, 83, 84//, 85 86, 87, 88, 89,
        // 90, 91, 92, 93, 94, 95, 96, 97, 98, 99
    );
}