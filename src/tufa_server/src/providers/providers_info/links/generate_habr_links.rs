use crate::project_constants::HABR_LINK_FIRST_PART;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn generate_habr_links(habr_names: Vec<String>) -> Vec<String> {
    //example https://habr.com/ru/rss/all/all/?fl=ru?with_hubs=true:?with_tags=true:
    habr_names
        .iter()
        .map(|name| format!("{HABR_LINK_FIRST_PART}{name}"))
        .collect()
}
