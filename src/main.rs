use std::env;

use alphred::Item;
use regex::Regex;
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let query = args.join(" ");

    let client = Client::new();
    let resp = client
        .get("https://frinklang.org/fsp/frink.fsp")
        .query(&[("fromVal", query)])
        .send()
        .unwrap();
    let body = resp.text().unwrap();

    let results_re = Regex::new(r"(?is)<a name=results>(.*?)</a>").unwrap();
    let html_re = Regex::new(r"(?i)<.*?>").unwrap();

    let cap = results_re.captures(&body).unwrap();

    let result = cap.get(1).unwrap().as_str();
    let items: Vec<_> = result
        .lines()
        .map(|x| html_re.replace_all(x, ""))
        .map(Item::new)
        .collect();
    println!("{}", json!({ "items": items }));
}
