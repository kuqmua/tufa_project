// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum RssPartErrorNamed {
//     CheckLinkStatusCodeError {
//         #[eo_to_std_string_string]
//         reqwest_error: reqwest::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     StatusCode {
//         #[eo_to_std_string_string]
//         status_code: reqwest::StatusCode,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     // FetchAndParseProviderData {
//     //     source: crate::repositories_types::server::providers::provider_kind::provider_kind_enum::FetchAndParseProviderDataErrorEnum,
//     //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     // },
// }

// pub async fn rss_part<'a>(
//     pk: crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind,
//     vec_of_provider_links: Vec<String>,
// ) -> Result<Vec<crate::repositories_types::server::fetch::info_structures::common_rss_structures::CommonRssPostStruct>, Box<RssPartErrorNamed>> {
//     match reqwest::get({
//         // use crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
//         // pk.check_link()
//         "https://www.google.com/"
//     }).await {
//         Err(error) => Err(Box::new(RssPartErrorNamed::CheckLinkStatusCodeError {
//             reqwest_error: error,
//             code_occurence: error_occurence_lib::code_occurence!()
//         })),
//         Ok(res) => {
//             let status_code = res.status();
//             if !reqwest::StatusCode::is_success(&status_code) {
//                 return Err(Box::new(RssPartErrorNamed::StatusCode {
//                     status_code,
//                     code_occurence: error_occurence_lib::code_occurence!()
//                 }));
//             }
//             // match crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::fetch_and_parse_provider_data(pk, vec_of_provider_links).await {
//             //     Err(error) => Err(Box::new(RssPartErrorNamed::FetchAndParseProviderData {
//             //         source: *e,
//             //         code_occurence: error_occurence_lib::code_occurence!()
//             //     })),
//             //     Ok(vec) => Ok(vec),
//             // }
//             todo!()
//         }
//     }
// }
