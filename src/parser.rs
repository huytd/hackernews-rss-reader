use super::rss;
use rss::{Channel, Item};

pub type FetchResult<T> = Result<T, rss::Error>;

#[derive(Serialize)]
pub struct RSSItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
}

impl From<Item> for RSSItem {
    fn from(item: Item) -> Self {
        RSSItem{
            title: item.title().unwrap_or_default().to_owned(),
            link: item.link().unwrap_or_default().to_owned(),
            description: item.description().unwrap_or_default().to_owned(),
            pub_date: item.pub_date().unwrap_or_default().to_owned(),
        }
    }
}

pub fn fetch_from(url: &str) -> FetchResult<Vec<RSSItem>> {
    Ok(
        Channel::from_url(url)?
                .items()
                .into_iter()
                .map(|item| RSSItem::from(item.clone()))
                .collect()
    )
}

#[test]
fn test_fetch_valid_rss_url() {
    let items = fetch_from("https://thefullsnack.com/rss.xml");
    assert!(items.is_ok());
    assert!(items.unwrap().len() > 0);
}

#[test]
fn test_fetch_invalid_url() {
    let items = fetch_from("https://where-superman-meet-wonderwoman.com/and-they-got-married/rss.xml");
    assert!(items.is_err());
}

#[test]
fn test_fetch_invalid_rss_feed() {
    let items = fetch_from("https://xkcd.com/info.0.json");
    assert!(items.is_err());
}

#[test]
fn test_fetch_is_convertable_to_json() {
    let items = fetch_from("https://thefullsnack.com/rss.xml");
    assert!(items.is_ok());
    let json_data = json!({ "items": items.unwrap() });
    assert!(json_data["items"].is_array());
    assert!(json_data["items"][0].is_object());
    assert!(json_data["items"][0]["title"].is_string());
}
