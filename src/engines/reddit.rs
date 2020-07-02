use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!("https://www.reddit.com/search/?q={}", encoded_query);

    uri
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://www.reddit.com/user/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "rd" {
        return String::from("https://www.reddit.com");
    } else if &query[..4] == "rd @" {
        return construct_profile_url(&query[4..]);
    }

    construct_search_url(&query[3..])
}
