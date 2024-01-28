// pub async fn check_new_providers_posts<'a>(
//     providers_link_parts: std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     config: &'static (
//     impl crate::common::config::config_fields::GetGithubToken
// + std::marker::Send
// + std::marker::Sync
// )
// ) -> std::collections::HashMap<crate::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Result<Vec<crate::repositories_types::tufa_server::fetch::info_structures::common_rss_structures::CommonRssPostStruct>, crate::repositories_types::tufa_server::providers::provider_kind::functions::rss_part::RssPartErrorNamed>> {
//     let tasks_vec = providers_link_parts
//         .into_iter()
//         .map(|(pk, link_parts)| async move {
//             match crate::repositories_types::tufa_server::providers::provider_kind::functions::rss_part::rss_part(
//                 pk,
//                 {
// crate::repositories_types::tufa_server::traits::provider_kind_methods::ProviderKindMethods::generate_provider_links(
//     pk,
//     link_parts,
//     config
// )
//                 },
//             ).await {
//                 Ok(posts_vec) => (pk, Ok(posts_vec)),
//                 Err(e) => (pk, Err(*e)),
//             }
//         });
//     let posts_and_errors_to_return = futures::future::join_all(tasks_vec)
//         .await
//         .into_iter()
//         .map(|(pk, s)| (pk, s))
//         .collect();
//     posts_and_errors_to_return
// }
