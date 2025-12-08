// pub fn generate_github_links<
//     T: app_state::GetGithubToken
//     + Send
//     + Sync
// >(
//     github_names: &[String],
//     config: &T,
// ) -> Vec<String> {
//     github_names
//         .iter()
//         .map(|name| {
//             format!(
//                 "https://github.com/{name}.private.atom?token={}",
//                 config.get_github_token()
//             )
//         })
//         .collect()
// }
