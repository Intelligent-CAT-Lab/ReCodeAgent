#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
use crate::internal::utils::utils::equal;
use crate::internal::utils::utils::min;
//Translated from: github.com/hbollon/go-edlib.DamerauLevenshteinDistance
pub fn damerau_levenshtein_distance(str1: &str, str2: &str) -> Result<i32, Error> {
    // Convert string parameters to character vectors to handle non-ASCII characters
    let str1_chars: Vec<char> = str1.chars().collect();
    let str2_chars: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let str1_len = str1_chars.len();
    let str2_len = str2_chars.len();

    // Base cases
    if str1_len == 0 {
        return Ok(str2_len as i32);
    } else if str2_len == 0 {
        return Ok(str1_len as i32);
    } else if equal(&str1_chars, &str2_chars)? {
        return Ok(0);
    }

    // Create a set of unique characters from both input strings
    let mut char_set: HashSet<char> = HashSet::new();
    str1_chars.iter().for_each(|c| { char_set.insert(*c); });
    str2_chars.iter().for_each(|c| { char_set.insert(*c); });

    // 2D Array for distance matrix: matrix[0..str1.len()+2][0..str2.len()+2]
    let mut matrix = vec![vec![0; str2_len + 2]; str1_len + 2];

    // Maximum possible distance
    let max_dist = (str1_len + str2_len) as i32;

    // Initialize matrix
    matrix[0][0] = max_dist;
    for i in 0..=str1_len {
        matrix[i + 1][0] = max_dist;
        matrix[i + 1][1] = i as i32;
    }
    for j in 0..=str2_len {
        matrix[0][j + 1] = max_dist;
        matrix[1][j + 1] = j as i32;
    }

    // Process edit distance
    for i in 1..=str1_len {
        let mut db = 0;
        for j in 1..=str2_len {
            let i1 = char_set.get(&str2_chars[j - 1]).map_or(i, |idx| *idx as usize);
            let j1 = db;
            let cost = if str1_chars[i - 1] == str2_chars[j - 1] { 0 } else { 1 };
            db = j;

            matrix[i + 1][j + 1] = min(
                min(matrix[i + 1][j] + 1, matrix[i][j + 1] + 1),
                min(
                    matrix[i][j] + cost,
                    matrix[i1][j1] + (i - i1) as i32 + (j - j1) as i32
                )
            );
        }

        let i_idx = str1_len - i + 1;
        char_set.insert(str1_chars[i_idx]);
    }

    Ok(matrix[str1_len + 1][str2_len + 1])
}


//Translated from: github.com/hbollon/go-edlib.LevenshteinDistance
pub fn levenshtein_distance(str1: &str, str2: &str) -> Result<i32, Error> {
    // Convert string parameters to char vectors to be compatible with non-ASCII
    let runestr1: Vec<char> = str1.chars().collect();
    let runestr2: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let runestr1len = runestr1.len();
    let runestr2len = runestr2.len();
    if runestr1len == 0 {
        return Ok(runestr2len as i32);
    } else if runestr2len == 0 {
        return Ok(runestr1len as i32);
    } else if equal(&runestr1, &runestr2)? {
        return Ok(0);
    }

    let mut column = vec![0; runestr1len + 1];

    for y in 1..=runestr1len {
        column[y] = y as i32;
    }
    for x in 1..=runestr2len {
        column[0] = x as i32;
        let mut lastkey = (x - 1) as i32;
        for y in 1..=runestr1len {
            let oldkey = column[y];
            let i = if runestr1[y - 1] != runestr2[x - 1] {
                1
            } else {
                0
            };
            column[y] = min(
                min(column[y] + 1, // insert
                    column[y - 1] + 1), // delete
                lastkey + i); // substitution
            lastkey = oldkey;
        }
    }

    Ok(column[runestr1len])
}

//Translated from: github.com/hbollon/go-edlib.OSADamerauLevenshteinDistance
pub fn osa_damerau_levenshtein_distance(str1: &str, str2: &str) -> Result<i32> {
    // Convert string parameters to vectors of chars to be compatible with non-ASCII
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let len1 = chars1.len();
    let len2 = chars2.len();

    if len1 == 0 {
        return Ok(len2 as i32);
    } else if len2 == 0 {
        return Ok(len1 as i32);
    } else if equal(&chars1, &chars2).unwrap() {
        return Ok(0);
    } else if len1 < len2 {
        return osa_damerau_levenshtein_distance(str2, str1);
    }

    // 2D Vector
    let row = min((len1 + 1).try_into().unwrap(), 3);
    let mut matrix = vec![vec![0; len2 + 1]; row.try_into().unwrap()];

    for i in 0..row {
        matrix[i as usize][0] = i as i32;
    }

        for j in 0..=len2 {
            matrix[0][j as usize] = j as i32;
        }

    for i in 1..=len1 {
        matrix[(i % 3) as usize][0] = i as i32;
        for j in 1..=len2 {
            let count = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };

            matrix[(i % 3) as usize][j as usize] = min(
                min(matrix[((i - 1) % 3) as usize][j as usize] + 1, matrix[(i % 3) as usize][(j - 1) as usize] + 1),
                matrix[((i - 1) % 3) as usize][(j - 1) as usize] + count,
            ); // insertion, deletion, substitution

            if i > 1 && j > 1 && chars1[i - 1] == chars2[j - 2] && chars1[i - 2] == chars2[j - 1] {
                matrix[(i % 3) as usize][j as usize] = min(matrix[(i % 3) as usize][j as usize], matrix[((i - 2) % 3) as usize][(j - 2) as usize] + 1); // translation
            }
        }
    }

    Ok(matrix[(len1 % 3) as usize][len2 as usize])
}

