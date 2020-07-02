use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'+');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!("https://www.amazon.es/s?k={}", encoded_query);

    uri
}

pub fn construct_url(query: &str) -> String {
    if query == "az" {
        return String::from("https://www.amazon.es");
    }
    construct_search_url(&query[3..])
}
