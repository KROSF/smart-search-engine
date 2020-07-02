use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn construct_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let uri = format!(
        "https://hub.docker.com/search?q={}&type=image",
        encoded_query
    );

    uri
}

fn construct_profile_url(profile: &str) -> String {
    format!("https://hub.docker.com/u/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "dh" {
        return String::from("https://hub.docker.com");
    } else if &query[..4] == "dh @" {
        return construct_profile_url(&query[..4]);
    }

    construct_search_url(&query[3..])
}
