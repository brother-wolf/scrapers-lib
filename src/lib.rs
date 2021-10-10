pub mod twitter;

pub trait Site<T> {
    fn scrape(body: &str) -> Vec<T>;
}