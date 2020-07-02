use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!("https://twitter.com/search?q={}", encoded_query);

    uri
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "tw" {
        return String::from("https://twitter.com");
    } else if &query[..4] == "tw @" {
        return construct_profile_url(&query[..4]);
    }

    construct_search_url(&query[3..])
}
