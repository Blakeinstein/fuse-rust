mod utils;

use std::collections::HashMap;
use std::ops::Range;
use std::sync::{ Arc, Mutex};
use crossbeam::thread;

pub struct FuseProperty {
    pub name: String,
    pub weight: f64,
}

impl FuseProperty {
    pub fn init(name: &str) -> Self{
        Self{
            name: String::from(name), 
            weight: 1.0,
        }
    }
    pub fn init_with_weight(name: &str, weight: f64) -> Self{
        Self{
            name: String::from(name),
            weight: weight,
        }
    }
}

pub struct Pattern{
    text: String,
    len: usize,
    mask: u32,
    alphabet: HashMap<char, u32>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub index: usize,
    pub score: f64,
    pub ranges: Vec<Range<usize>>,
}

#[derive(Debug)]
pub struct ScoreResult {
    pub score: f64,
    pub ranges: Vec<Range<usize>>,
}

pub struct FResult {
    pub key: String,
    pub score: f64,
    pub ranges: Vec<Range<usize>>,
}

pub struct FusableSearchResult {
    pub index: i32,
    pub score: f64,
    pub results: FResult,
}

pub struct Fuse {
    pub location: i32,
    pub distance: i32,
    pub threshold: f64,
    pub max_pattern_length: i32,
    pub is_case_sensitive: bool,
    pub tokenize: bool,
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
    pub fn create_pattern(&self, string: &str) -> Option<Pattern> {
        let lowercase = string.to_lowercase();
        let pattern = if self.is_case_sensitive { string } else { &lowercase };
        let len = pattern.len();

        if len == 0 {
            println!("Cannot create pattern, input string empty");
            None
        } else {
            let alphabet = utils::calculate_pattern_alphabet(&pattern);
            let new_pattern = Pattern{
                text: String::from(pattern),
                len: len,
                mask: 1 << (len - 1),
                alphabet: alphabet,
            };
            Some(new_pattern)
        }
     
    }

    fn search_util(
        &self,
        pattern: &Pattern,
        string: &str) -> ScoreResult {
            let string = if self.is_case_sensitive {
                String::from(string)
            } else {
                string.to_ascii_lowercase()
            };
            let string_chars = string.chars().collect::<Vec<_>>();
            let text_length = string.len();

            // Exact match
            if pattern.text == string {
                return ScoreResult {
                    score: 0.,
                    ranges: vec![0..text_length as usize]
                };
            }

            let location = self.location;
            let distance = self.distance;
            let mut threshold = self.threshold;

            let mut best_location = string.find(&pattern.text).unwrap_or(0 as usize);

            let mut match_mask_arr = vec![0; text_length];

            let mut index = string[
                    best_location..
                ].find(&pattern.text);

            let mut score;
            
            while index.is_some() {
                let i = best_location + index.unwrap();
                score = utils::calculate_score(
                    pattern.len,
                    0,
                    i as i32,
                    location,
                    distance
                );

                threshold = threshold.min(score);

                best_location = i + pattern.len;

                index = string[
                        best_location..
                    ].find(&pattern.text);
                
                for idx in 0..pattern.len {
                    match_mask_arr[i+idx] = 1;
                };
            }

            score = 1.;
            let mut bin_max = pattern.len + text_length;
            let mut last_bit_arr = vec!();

            let text_count = string.len();
            
            for i in 0..pattern.len {
                let mut bin_min = 0;
                let mut bin_mid = bin_max;
                while bin_min < bin_mid {
                    if utils::calculate_score(
                        pattern.len, i as i32, location, location + bin_mid as i32, distance) <= threshold {
                            bin_min = bin_mid;
                    } else {
                        bin_max = bin_mid;
                    }
                    bin_mid = ((bin_max - bin_min) / 2) + bin_min;
                }
                bin_max = bin_mid;

                let start = 1.max(location - bin_mid as i32 + 1) as usize;
                let finish = text_length.min(location as usize + bin_mid) + pattern.len;

                let mut bit_arr = vec!(0; finish + 2);

                bit_arr[finish+1] = (1 << i) - 1;
                
                if start > finish {
                    continue;
                };

                let mut current_location_index: usize = 0;

                for j in (start as u32..=finish as u32).rev() {
                    let current_location: usize = (j - 1) as usize;
                    let char_match: u32 = {
                        let mut result = None;
                        if current_location < text_count {
                            current_location_index = current_location_index.checked_sub(1).unwrap_or(current_location);
                            result = pattern.alphabet.get(
                                &string_chars[current_location_index]
                            );
                        }
                        *result.unwrap_or(&0)
                    };
                    
                    if char_match != 0 {
                        match_mask_arr[current_location] = 1;
                    }
                    
                    let j2 = j as usize;
                    bit_arr[j2] = ((bit_arr[j2+1] << 1) | 1) & char_match;
                    if i > 0 {
                        bit_arr[j2] |= (((last_bit_arr[j2+1] | last_bit_arr[j2]) << 1 as u32) | 1) | last_bit_arr[j2+1];
                    };

                    if (bit_arr[j2] & pattern.mask) != 0 {
                        score = utils::calculate_score(
                            pattern.len,
                            i as i32,
                            location,
                            current_location as i32,
                            distance
                        );
            
                        if score <= threshold {
                            threshold = score;
                            best_location = current_location;

                            if best_location as i32 <= location {
                                break;
                            };
                        }
                    }
                }
                if utils::calculate_score(
                    pattern.len,
                    i as i32 + 1,
                    location,
                    location,
                    distance
                ) > threshold {
                    break;  
                }

                last_bit_arr = bit_arr.clone();
            };

            ScoreResult {
                score: score,
                ranges: utils::find_ranges(&match_mask_arr).unwrap(),
            }
    }

    pub fn search(
        &self,
        pattern: Option<&Pattern>,
        string: &str
    ) -> Option<ScoreResult> {
        let pattern = pattern?;
        
        if self.tokenize {
            let word_patterns = pattern.text.split_whitespace().filter_map(
                |x| self.create_pattern(x)
            );

            let full_pattern_result = self.search_util(&pattern, string);

            let (length, results) = word_patterns.fold((0, full_pattern_result), |(n, mut total_result), pattern| {
                let result = self.search_util(&pattern, string);
                total_result.score += result.score;
                total_result.ranges.append(&mut result.ranges.clone());
                (n+1, total_result)
            });

            let averaged_result = ScoreResult{
                score: results.score / (length + 1) as f64,
                ranges: results.ranges
            };

            return if averaged_result.score == 1. {None} else {Some(averaged_result)};

        } else {
            let result = self.search_util(&pattern, string);
            return if result.score == 1. {None} else {Some(result)};
        }
    }
}

pub trait Fuseable {
    fn properties() -> Vec<FuseProperty> ;
}

impl Fuse {
    pub fn search_text_in_string(&self, text: &str, astring: &str) -> Option<ScoreResult>{
        self.search(self.create_pattern(text).as_ref(), astring)
    }

    pub fn search_text_in_iterable<It>(&self, text: &str, list: It) -> Vec<SearchResult>
    where 
        It: IntoIterator,
        It::Item: AsRef<str>
    {
        let pattern = self.create_pattern(text);
        let mut items = vec!();
        
        for (index, item) in list.into_iter().enumerate() {
            if let Some(result) = self.search(pattern.as_ref(), item.as_ref()) {
                items.push(
                    SearchResult {
                        index: index,
                        score: result.score,
                        ranges: result.ranges,
                    }
                )
            }
        }
        items.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        items
    }

    pub fn search_text_in_string_list(&self, text: &str, list: &[&str], chunk_size: usize, completion: &dyn Fn(Vec<SearchResult>)){
        let pattern = Arc::new(self.create_pattern(text));
        
        let item_queue = Arc::new(Mutex::new(Some(vec!())));
        let count = list.len();
        
        thread::scope(|scope| {
            (0..=count).step_by(chunk_size).for_each(|offset| {
                let chunk = &list[offset..count.min(offset + chunk_size)];
                let queue_ref = Arc::clone(&item_queue);
                let pattern_ref = Arc::clone(&pattern);
                scope.spawn(move|_| {
                    let mut chunk_items = vec!() ;
                    
                    for (index, item) in chunk.into_iter().enumerate() {
                        if let Some(result) = self.search((*pattern_ref).as_ref(), item) {
                            chunk_items.push(
                                SearchResult {
                                    index: offset + index,
                                    score: result.score,
                                    ranges: result.ranges,
                                }
                            );
                        }
                    }

                    let mut inner_ref = queue_ref.lock().unwrap();
                    if let Some(item_queue) = inner_ref.as_mut() {
                        item_queue.append(&mut chunk_items);
                    }
                });
            });
        }).unwrap();

        let mut items = Arc::try_unwrap(item_queue).ok().unwrap().into_inner().unwrap().unwrap();
        items.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        completion(items);
    }
}