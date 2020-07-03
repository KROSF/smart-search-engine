pub mod amazon;
pub mod dev;
pub mod docker;
pub mod github;
pub mod gmail;
pub mod google;
pub mod reddit;
pub mod twitter;
pub mod youtube;

pub fn get_command_from_query_string(query: &str) -> &str {
    if query.contains(' ') {
        let index_of_space = query.find(' ').unwrap_or(0);
        return &query[..index_of_space];
    }
    query
}
