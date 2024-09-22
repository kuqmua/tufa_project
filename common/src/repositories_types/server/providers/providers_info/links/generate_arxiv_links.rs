pub fn generate_arxiv_links(arxiv_names: &[std::string::String]) -> Vec<String> {
    //example http://export.arxiv.org/rss/astro-ph.CO
    arxiv_names.iter().map(|name| format!("http://export.arxiv.org/rss/{name}")).collect()
}
