use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!(
        "https://www.youtube.com/results?search_query={}",
        encoded_query
    );

    uri
}

pub fn construct_url(query: &str) -> String {
    if query == "yt" {
        return String::from("https://www.youtube.com");
    }
    construct_search_url(&query[3..])
}
