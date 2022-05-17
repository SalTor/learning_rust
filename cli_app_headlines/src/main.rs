extern crate reqwest;
extern crate select;
extern crate scraper;

use select::document::Document;
use select::predicate::Name;
use scraper::{Html, Selector};

fn main() {
    scrape("https://saltor.nyc");
    hn_headlines("https://news.ycombinator.com")
}

fn hn_headlines(url: &str) {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();
    let fragment = Html::parse_document(&body);
    let stories = Selector::parse(".titlelink").unwrap();

    for story in fragment.select(&stories) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }
}

fn scrape(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
