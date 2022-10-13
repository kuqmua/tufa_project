use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use itertools::Itertools;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    ImplGetSourceWithMethod,
    ImplDisplayForErrorStruct,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas,
)]
pub struct GetLinkPartsFromLocalJsonFileError {
    source: GetLinkPartsFromLocalJsonFileErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethod, ImplDisplayForSimpleErrorEnum)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen(std::io::Error),
    TokioIoAsyncReadExtReadToEnd(std::io::Error),
    SerdeJsonFromSlice(serde_json::Error),
}

impl ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn get_link_parts_from_local_json_file(
        self,
        should_trace: bool,
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileError>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(Box::new(
                GetLinkPartsFromLocalJsonFileError::init_error_with_possible_trace(
                    GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen(e),
                    WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                ),
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd(e),
                            WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                            &CONFIG.source_place_type,
                            &GIT_INFO.data,
                            should_trace,
                        ),
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileError::init_error_with_possible_trace(
                            GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice(e),
                            WhereWas {
                                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                file: file!(),
                                line: line!(),
                                column: column!(),
                            },
                            &CONFIG.source_place_type,
                            &GIT_INFO.data,
                            should_trace,
                        ),
                    )),
                    Ok(file_content_as_struct) => {
                        let unique_vec: Vec<String> =
                            file_content_as_struct.data.into_iter().unique().collect();
                        let return_vec: Vec<String>;
                        //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
                        if CONFIG.is_links_limit_enabled_providers && self.is_links_limit_enabled()
                        {
                            let limit = self.links_limit();
                            if unique_vec.len() > limit {
                                return_vec = unique_vec
                                    .into_iter()
                                    .enumerate()
                                    .filter_map(|(index, element)| match index > limit {
                                        false => None,
                                        true => Some(element),
                                    })
                                    .collect::<Vec<String>>();
                            } else {
                                return_vec = unique_vec;
                            }
                        } else {
                            return_vec = unique_vec;
                        }
                        Ok(return_vec)
                    }
                }
            }
        }
    }
}
