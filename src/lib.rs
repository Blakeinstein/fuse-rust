mod swiftcom;
mod utils;

use swiftcom::*;
use std::collections::HashMap;

pub struct FuseProperty {
    name: String,
    weight: f64,
}

impl FuseProperty {
    pub fn init(name: String) -> Self{
        Self{
            name: name, 
            weight: 1.0
        }
    }
    pub fn init_with_weight(name: String, weight: f64) -> Self{
        Self{
            name,
            weight
        }
    }
}

pub trait Fuseable {
    fn properties() -> vec![FuseProperty];
}

pub struct Pattern{
    text: String,
    len: usize,
    mask: i32,
    alphabet: Vec<char>,
}
    
pub struct SearchResult {
    index: i32,
    score: f64,
    ranges: [CountableCloseRange<i32>],
}

struct FResult {
    key: String,
    score: f64,
    ranges: [CountableCloseRange<i32>],
}

pub struct FusableSearchResult {
    index: i32,
    score: f64,
    results: FResult,
}

pub struct Fuse {
    location: i32,
    distance: i32,
    threshold: f64,
    max_pattern_length: i32,
    is_case_sensitive: bool,
    tokenize: bool,
}

impl std::default::Default for Fuse {
    fn default() -> Self {
        Self{
            location: 0,
            distance: 100,
            threshold: 0.6,
            max_pattern_length: 32,
            is_case_sensitive: false,
            tokenize: false,
        }
    }
}

impl Fuse {
    pub fn create_pattern(&self, string: &str) -> Result<Pattern, String> {
        let pattern = if self.is_case_sensitive { string } else { &string.to_lowercase() };
        let len = pattern.len();

        if len == 0 {
            Err(|| String::from("Cannot create pattern"))
        } else {
            let new_pattern = Pattern{
                text: String::from(pattern),
                len: len,
                mask: 1 << (len - 1),
                alphabet: utils::,
            };
            Ok(new_pattern)
        }

        
    }
}