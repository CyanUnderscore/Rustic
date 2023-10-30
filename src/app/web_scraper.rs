extern crate reqwest;
extern crate scraper;

use std::fmt::Error;
use scraper::{Html, Selector};
use reqwest::StatusCode;
use reqwest::Client;

pub async fn get_name(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url).send().await?;

    //if res.status() != StatusCode::OK {
      //  return Err();
    //}

    let body = res.text().await?;
    
    let document = Html::parse_document(&body);
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().unwrap().text().collect::<String>();


    Ok(title)
}