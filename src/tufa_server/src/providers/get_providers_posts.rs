use super::check_providers_link_parts_on_empty::CheckProvidersLinkPartsEmptyError;
use super::provider_kind::functions::rss_part::RssPartErrorEnum;
use super::provider_kind::provider_kind_enum::ProviderKind;
use super::providers_info::get_providers_link_parts::GetProvidersLinkPartsErrorEnum;
use crate::check_new_providers_posts::check_new_providers_posts;
use crate::lazy_static::config::CONFIG;
use crate::providers::check_providers_link_parts_on_empty::check_providers_link_parts_on_empty;
use crate::providers::providers_info::get_providers_link_parts::get_providers_link_parts;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use std::collections::HashMap;
use tufa_common::where_was::WhereWas;
// use crate::write_error_posts_wrapper::write_error_posts_wrapper;
//     let future_possible_drop_collection = mongo_drop_collection_wrapper(
//         mongo_url,
//         db_name_handle,
//         &format!("{key}{db_collection_handle_second_part}"),
//         false,
//     );
//     match future_possible_drop_collection {
//         Ok(result_flag) => {
//             if result_flag {
//                 println!("drop done!");
//             } else {
//                 println!("drop fail with flag");
//             }
//         }
//         Err(e) => {
//             println!("drop fail with error {e:#?}",);
//         }
//     }

// use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
// use reqwest::StatusCode;
// #[derive(Debug)]
// pub enum PostErrorVariant {
//     //todo: think about this naming
//     NoItems {
//         link: String,
//         no_items_error: NoItemsError,
//         provider_kind: ProviderKind,
//     },
//     RssFetchAndParseProviderDataError {
//         link: String,
//         provider_kind: ProviderKind,
//         error: RssFetchLinkError,
//     }, //rewrite this error coz it must not be string. dont know to to clone error between threads
// }

// #[derive(Debug)]
// pub enum RssPartError {
//     ReqwestError(reqwest::Error),
//     StatusCode(StatusCode),
// }
//TODO: WRITE CONVERSION FUNCTION INTO COMMON ERROR ENUM AND MOVE IT INTO write_error_posts_wrapper

#[derive(Debug)]
pub enum GetProviderPostsErrorEnum {
    GetLocalProvidersLinkParts {
        source: GetProvidersLinkPartsErrorEnum,
        where_was: WhereWas,
    },
    CheckProvidersLinkPartsEmpty {
        source: CheckProvidersLinkPartsEmptyError,
        where_was: WhereWas,
    },
    GetNewProvidersPosts {
        source: HashMap<ProviderKind, RssPartErrorEnum>,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn get_providers_posts() -> Result<(), Box<GetProviderPostsErrorEnum>> {
    match get_providers_link_parts(&CONFIG.providers_link_parts_source).await {
        Err(e) => {
            return Err(Box::new(
                GetProviderPostsErrorEnum::GetLocalProvidersLinkParts {
                    source: *e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                },
            ));
        }
        Ok(providers_link_parts) => {
            match check_providers_link_parts_on_empty(providers_link_parts) {
                Err(e) => {
                    return Err(Box::new(
                        GetProviderPostsErrorEnum::CheckProvidersLinkPartsEmpty {
                            source: *e,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        },
                    ));
                }
                Ok(non_empty_providers_link_parts) => {
                    let hm = check_new_providers_posts(non_empty_providers_link_parts).await;
                    let mut error_hashmap = HashMap::with_capacity(hm.len());
                    let mut success_hashmap = HashMap::with_capacity(hm.len());
                    for (key, value) in hm {
                        match value {
                            Err(e) => {
                                error_hashmap.insert(key, e);
                            }
                            Ok(vec) => {
                                success_hashmap.insert(key, vec);
                            }
                        }
                    }
                    if !error_hashmap.is_empty() {
                        return Err(Box::new(GetProviderPostsErrorEnum::GetNewProvidersPosts {
                            source: error_hashmap,
                            where_was: WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                        }));
                    }
                    Ok(())
                }
            }
            // //todo: conversion function before write_error_posts_wrapper
            // //commented before conversion function implementation
            // // if !vec.is_empty() {
            // //     for (pk, result_vec) in vec {
            // //         match result_vec {
            // //             Ok((vec_common_rss_post_structs, vec_post_error_variants)) => {
            // //                 let wrong_cases_thread = thread::spawn(move || {
            // //                     block_on(write_error_posts_wrapper(vec_post_error_variants));
            // //                 });
            // //                 match wrong_cases_thread.join() {
            // //                     Ok(_) => {}
            // //                     Err(e) => {
            // //                         print_colorful_message(
            // //                             None,
            // //                             PrintType::Error,
            // //                             file!().to_string(),
            // //                             line!().to_string(),
            // //                             format!("wrong_cases_thread.join() error: {e:#?}"),
            // //                         );
            // //                     }
            // //                 }
            // //             }
            // //             Err(e) => {}
            // //         }
            // //     }
            // // }
        }
    }
}
