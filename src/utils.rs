use std::collections::HashMap;
use std::ops::Range;

pub fn calculate_score_with_string(
    pattern: String,
    e: i32,
    x: i32,
    loc: i32,
    distance: i32) -> f64 {
        calculate_score(
            pattern.len(),
            e,
            x,
            loc,
            distance,
        )
}

pub fn calculate_score(
    pattern_length: usize,
    e: i32,
    x: i32,
    loc: i32,
    distance: i32) -> f64 {
    
    let accuracy = (e as f64) / (pattern_length as f64);
    let proximity = (x - loc).abs();
    if distance == 0 {
        return if proximity != 0 { 1. } else { accuracy as f64} 
    }
    accuracy + (proximity as f64) / ( distance as f64)
}

/// Initializes the alphabet for the Bitap algorithm
///
/// - Parameter pattern: The text to encode.
/// - Returns: Hash of character locations.
pub fn calculate_pattern_alphabet(pattern: &str) -> HashMap<char, u32> {
    let len = pattern.len();
    let mut mask = HashMap::new();
    for (i, c) in pattern.chars().enumerate() {
        mask.insert(c, mask.get(&c).unwrap_or(&0) | (1 << (len -i -1)));
    }
    mask
}

/// Returns an array of `CountableClosedRange<Int>`, where each range represents a consecutive list of `1`s.
///
///     let arr = [0, 1, 1, 0, 1, 1, 1 ]
///     let ranges = findRanges(arr)
///     // [{startIndex 1, endIndex 2}, {startIndex 4, endIndex 6}
///
/// - Parameter mask: A string representing the value to search for.
///
/// - Returns: `CountableClosedRange<Int>` array.
pub fn find_ranges(mask: &[u8]) -> Result<Vec<Range<usize>>, String> {
    if mask.is_empty() {
        return Err(String::from("Input array is empty"));
    }
    let mut ranges = vec!();
    let mut start: i32 = -1;
    for (n, bit) in mask.iter().enumerate() {
        if start == -1 && *bit >= 1{
            start = n as i32;
        } else if start != -1 && *bit == 0 {
            ranges.push(start as usize..n);
            start = -1;
        }
    }

    if *mask.last().unwrap() == 1 {
        ranges.push(start as usize..mask.len())
    }
    Ok(ranges)
}
