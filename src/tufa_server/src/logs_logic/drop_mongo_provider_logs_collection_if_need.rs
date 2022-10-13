use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::mongo_integration::mongo_drop_empty_collection::mongo_drop_empty_collection;
use crate::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorEnum;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn drop_mongo_provider_logs_collection_if_need(
    pk: &ProviderKind,
    mongo_url: String,
) -> Result<(), Box<MongoDropEmptyCollectionErrorEnum>> {
    if !pk.is_cleaning_warning_logs_directory_enabled() {
        return Ok(());
    }
    if let Err(e) = mongo_drop_empty_collection(
        &mongo_url,
        &CONFIG.mongo_providers_logs_db_name,
        &format!(
            "{pk:#?}{}",
            &CONFIG.mongo_providers_logs_db_collection_handle_second_part
        ),
    )
    .await
    {
        print_colorful_message(
            Some(pk),
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            format!("drop fail with error {e:#?}"),
        );
        return Err(e);
    }
    Ok(())
}
