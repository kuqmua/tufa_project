// use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;

// // use std::time::Instant;
// //async
// // let res = reqwest::get(link).await?;
// //NOT WORKING FOR SOME REASON (returning 404)
// // let client = reqwest::blocking::Client::new();
// // let res = reqwest::blocking::Client::head(&client, link).send()?;
// // let mut result_tuplefff: (bool, HandledReachProviderStatusInfo) =
// //     (false, HandledReachProviderStatusInfo::Initialized);
// // if res.status() == reqwest::StatusCode::OK {
// //     println!("fetch_link res.status() ok");
// //     result_tuplefff = (true, HandledReachProviderStatusInfo::Success)
// // } else {
// //     println!("fetch_link res.status() not ok");
// //     result_tuplefff.1 = HandledReachProviderStatusInfo::ResStatusError(res.status());
// // }
// #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
// pub fn rss_check_provider_status(
//     link: &str,
// ) -> Result<(bool, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
//     let res = reqwest::blocking::get(link)?;
//     if res.status() == reqwest::StatusCode::OK {
//         Ok((true, HandledFetchStatusInfo::Success))
//     } else {
//         print_colorful_message(
//             None,
//             PrintType::Error,
//             vec![format!("{}:{}:{}", file!(), line!(), column!())],
//             vec![get_git_source_file_link(file!(), line!())],
//             format!("{link} {}", res.status()),
//         );
//         Ok((false, HandledFetchStatusInfo::ResStatusError(res.status())))
//     }
// }
