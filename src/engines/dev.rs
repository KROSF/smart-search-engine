use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!("https://dev.to/search?q={}", encoded_query);

    uri
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://dev.to/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "dev" {
        return String::from("https://dev.to");
    } else if &query[..5] == "dev @" {
        return construct_profile_url(&query[5..]);
    }

    construct_search_url(&query[4..])
}
