#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::internal::utils::utils::equal;
use crate::internal::utils::utils::max;
use crate::internal::utils::utils::min;
//Translated from: github.com/hbollon/go-edlib.JaroSimilarity
pub fn jaro_similarity(str1: &str, str2: &str) -> Result<f32> {
    // Convert string parameters to Vec<char>
    let runevec_str1: Vec<char> = str1.chars().collect();
    let runevec_str2: Vec<char> = str2.chars().collect();

    // Get and store length of these strings
    let runevec_str1len = runevec_str1.len();
    let runevec_str2len = runevec_str2.len();
    if runevec_str1len == 0 || runevec_str2len == 0 {
        return Ok(0.0);
    } else if equal(&runevec_str1, &runevec_str2)? {
        return Ok(1.0);
    }

    let mut match_count = 0;
    // Maximum matching distance allowed
    let max_dist = max(runevec_str1len.try_into().unwrap(), runevec_str2len.try_into().unwrap()) / 2 - 1;
    // Correspondence tables (1 for matching and 0 if it's not the case)
    let mut str1_table = vec![0; runevec_str1len];
    let mut str2_table = vec![0; runevec_str2len];

    // Check for matching characters in both strings
    for i in 0..runevec_str1len {
        for j in max(0, i as i32 - max_dist as i32)..min(runevec_str2len as i32, i as i32 + max_dist as i32 + 1) {
            let j = j as usize;
            if runevec_str1[i] == runevec_str2[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                match_count += 1;
                break;
            }
        }
    }
    if match_count == 0 {
        return Ok(0.0);
    }

    let mut t = 0.0;
    let mut p = 0;
    // Check for possible translations
    for i in 0..runevec_str1len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if runevec_str1[i] != runevec_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;

    Ok((match_count as f32 / runevec_str1len as f32
        + match_count as f32 / runevec_str2len as f32
        + (match_count as f32 - t) / match_count as f32)
        / 3.0)
}


//Translated from: github.com/hbollon/go-edlib.JaroWinklerSimilarity
pub fn jaro_winkler_similarity(str1: &str, str2: &str) -> Result<f32, Error> {
    // Get Jaro similarity index between str1 and str2
    let jaro_sim = jaro_similarity(str1, str2)?;

    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        // Convert string parameters to Vec<char> to be compatible with non-ASCII
        let str1_chars: Vec<char> = str1.chars().collect();
        let str2_chars: Vec<char> = str2.chars().collect();

        // Get and store length of these strings
        let str1_len = str1_chars.len();
        let str2_len = str2_chars.len();

        let mut prefix = 0;

        // Find length of the common prefix
        for (i, (c1, c2)) in str1_chars.iter().zip(str2_chars.iter()).enumerate() {
            if c1 == c2 {
                prefix += 1;
            } else {
                break;
            }
        }

        // Normalized prefix count with Winkler's constraint
        // (prefix length must be inferior or equal to 4)
        let prefix = min(prefix, 4);

        // Return calculated Jaro-Winkler similarity index
        Ok(jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim))
    } else {
        Ok(jaro_sim)
    }
}
