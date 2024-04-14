pub fn generate_github_links(
    github_names: Vec<String>,
    config: &'static (impl config_lib::GetGithubToken
                  + std::marker::Send
                  + std::marker::Sync),
) -> Vec<String> {
    github_names
        .iter()
        .map(|name| {
            format!(
                "https://github.com/{name}.private.atom?token={}",
                config.get_github_token()
            )
        })
        .collect()
}
