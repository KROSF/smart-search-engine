#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, response::Redirect, routes};
mod engines;
use engines::{amazon, dev, docker, github, gmail, google, reddit, twitter, youtube};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<q>")]
fn search(q: String) -> Redirect {
    let command = engines::get_command_from_query_string(&q);
    let redirect_uri = match command {
        "az" => amazon::construct_url(&q),
        "dev" => dev::construct_url(&q),
        "dh" => docker::construct_url(&q),
        "gh" => github::construct_url(&q),
        "gm" => gmail::construct_url(&q),
        "rd" => reddit::construct_url(&q),
        "tw" => twitter::construct_url(&q),
        "yt" => youtube::construct_url(&q),
        _ => google::construct_url(&q),
    };

    Redirect::to(redirect_uri)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
