use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::helpers::fetch::async_fetch_link::async_fetch_link;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use git_info::GitInfo;
use std::time::Instant;
use tufa_common::where_was::WhereWas;

#[derive(Debug, GitInfo)]
pub enum FetchAndParseProviderDataErrorEnum {
    AsyncFetchLinks {
        source: Vec<(String, Box<FetchLinkErrorEnum>)>, //link, error
        where_was: WhereWas,
    },
    NoItems {
        source: Vec<(String, NoItemsError)>, //link, error
        where_was: WhereWas,
    },
}

impl ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn fetch_and_parse_provider_data(
        self,
        links: Vec<String>,
    ) -> Result<Vec<CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorEnum>> {
        let time = Instant::now();
        let capacity = links.len();
        let vec_to_return = join_all(links.iter().map(|link| async move {
            let result = async_fetch_link(link).await;
            print_colorful_message(
                None,
                PrintType::TimeMeasurement,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                format!(
                    "fetch_link {link} in {}.{}ms",
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis() / 10,
                ),
            );
            (link, result)
        }))
        .await;
        let mut half_success_vec = Vec::with_capacity(capacity);
        let mut async_fetch_links_error_vec = Vec::new();
        for (link, result) in vec_to_return {
            match result {
                Err(e) => {
                    async_fetch_links_error_vec.push((link.to_string(), e));
                }
                Ok(str) => {
                    half_success_vec.push((link, str));
                }
            }
        }
        if !async_fetch_links_error_vec.is_empty() {
            //todo: maybe not all links must return Ok ?
            for (link, e) in &async_fetch_links_error_vec {
                if let FetchLinkErrorEnum::StatusCode {
                    source,
                    where_was: _,
                } = **e
                {
                    handle_error_status_code(source, &link);
                }
            }
            return Err(Box::new(
                FetchAndParseProviderDataErrorEnum::AsyncFetchLinks {
                    source: async_fetch_links_error_vec,
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
        let mut success_vec = Vec::with_capacity(capacity);
        let mut no_items_error_vec = Vec::new();
        for (link, response_text) in half_success_vec {
            match rss_parse_string_into_struct(response_text, link, self) {
                Err(e) => no_items_error_vec.push((link.to_string(), e)),
                Ok(post_struct) => {
                    success_vec.push(post_struct); //todo maybe add link here?
                }
            }
        }
        if !no_items_error_vec.is_empty() {
            return Err(Box::new(FetchAndParseProviderDataErrorEnum::NoItems {
                source: no_items_error_vec,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }));
        }
        Ok(success_vec)
    }
}
