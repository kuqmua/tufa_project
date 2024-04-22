pub fn generate_github_links<
    T: app_state::GetGithubToken
    + std::marker::Send
    + std::marker::Sync
>(
    github_names: Vec<String>,
    config: T,
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
