mod twitter;

trait Site<T> {
    fn scrape(body: &str) -> Vec<T>;
}