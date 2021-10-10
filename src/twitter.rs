use std::convert::TryFrom;
use chrono::{DateTime, NaiveDateTime, Utc};
use scraper::{Html, Selector};
use serde_derive::Serialize;
use crate::Site;


#[derive(Debug, Clone, Serialize)]
pub struct Tweet {
    pub id: u64,
    pub epoch: i64,
    pub date_time: String,
    pub content: String
}

impl std::fmt::Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {} ({}): {}", self.id, self.date_time, self.epoch, self.content)
    }
}

impl Tweet {
    pub fn new(id: u64, date_time: DateTime::<Utc>, content: String) -> Tweet {
        Tweet { id, epoch: date_time.timestamp_millis(), date_time: date_time.format("%Y-%m-%d %H:%M:%S").to_string(), content}
    }
}

pub struct Twitter {}

impl Site<Tweet> for Twitter {
    fn scrape(body: &str) -> Vec<Tweet> {
        let html_body = Html::parse_fragment(body);
        let content_selector = Selector::parse("div.content").unwrap();
        let tweet_contents_selector = Selector::parse("p.tweet-text").unwrap();
        let date_time_selector = Selector::parse("span._timestamp").unwrap();
        let tweet_id_selector = Selector::parse("a.tweet-timestamp").unwrap();

        html_body.select(&content_selector).map(|element| {
            let tweet = element.select(&tweet_contents_selector).next().unwrap().inner_html();
            let tweet_id = element.select(&tweet_id_selector).next().unwrap().value().attr("href").unwrap().split("/status/").last().unwrap().parse::<u64>().unwrap();
            let date_time = element.select(&date_time_selector).last().unwrap().value().attr("data-time-ms").unwrap().to_string().parse::<i64>().unwrap();
            let n_date_time = NaiveDateTime::from_timestamp(date_time / 1000, u32::try_from(date_time % 1000).unwrap());
            let time = DateTime::<Utc>::from_utc(n_date_time, Utc);
            Tweet::new(tweet_id, time, tweet)
        }).collect::<Vec<Tweet>>()
    }
}

#[cfg(test)]
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[cfg(test)]
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[test]
fn should_extract_tweet_elements_from_html() {
    let body = lines_from_file("data/test/subset-of-twitter-page.html").expect("Could not load lines").join("\n");
    let tweets = Twitter::scrape(&body);
    assert_eq!(1, tweets.len());
    let tweet = tweets.first().unwrap();
    assert_eq!(1443075780444098563, tweet.id);
    assert_eq!("2021-09-29 04:50:51", tweet.date_time);
    assert_eq!(1632891051000, tweet.epoch);
    assert_eq!("Good morning, we are currently on westerly operations, landing on the northern runway 27R and taking off from the southern runway 27L.", tweet.content);
}