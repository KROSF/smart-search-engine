use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!("https://github.com/search?q={}", encoded_query);

    uri
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://github.com/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "gh" {
        return String::from("https://github.com");
    } else if &query[..4] == "gh @" {
        return construct_profile_url(&query[4..]);
    } else if query[3..].contains("/") {
        return format!("https://github.com/{}", &query[3..]);
    }

    construct_search_url(&query[3..])
}
