fn construct_profile_url(profile: &str) -> String {
    format!("https://mail.google.com/mail/u/{}", profile)
}

pub fn construct_url(query: &str) -> String {
    if query == "gm" {
        return String::from("https://mail.google.com/mail");
    }

    construct_profile_url(&query[3..])
}
