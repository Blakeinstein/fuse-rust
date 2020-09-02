pub struct CountableCloseRange<T> 
where T: std::cmp::Ord{
    low: T,
    high: T,
}