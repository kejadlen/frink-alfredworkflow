use std::env;

use alphred::{Item, Workflow};
use anyhow::Result;
use regex::Regex;
use reqwest::blocking::Client;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let query = args.join(" ");

    let html_re = Regex::new(r"(?i)<.*?>").unwrap();

    println!(
        "{}",
        Workflow::new(|| Ok(frink(&query)?
            .iter()
            .map(|x| html_re.replace_all(x, ""))
            .map(Item::new)
            .collect()))
    );
}

fn frink(query: &str) -> Result<Vec<String>> {
    let client = Client::new();
    let resp = client
        .get("https://frinklang.org/fsp/frink.fsp")
        .query(&[("fromVal", query)])
        .send()
        .unwrap();
    let body = resp.text().unwrap();

    let results_re = Regex::new(r"(?is)<a name=results>(.*?)</a>").unwrap();

    let cap = results_re.captures(&body).unwrap();
    let result = cap.get(1).unwrap().as_str();

    Ok(result.lines().map(str::to_string).collect())
}
