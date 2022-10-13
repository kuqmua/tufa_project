use crate::helpers::resource::Resource;
use crate::lazy_static::config::CONFIG;
use crate::mongo_integration::mongo_get_providers_link_parts::mongo_get_providers_link_parts;
use crate::mongo_integration::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts;
use crate::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use std::collections::HashMap;
use tufa_common::where_was::WhereWas;
// use crate::postgres_integration::postgres_get_providers_link_parts::postgres_get_providers_link_parts;
// use crate::postgres_integration::postgres_get_providers_link_parts::PostgresGetProviderLinksError;

#[derive(Debug)]
pub enum GetProvidersLinkPartsErrorEnum {
    Local {
        source: GetLocalProvidersLinkPartsError,
        where_was: WhereWas,
    },
    Mongodb {
        source: MongoGetProvidersLinkPartsError,
        where_was: WhereWas,
    },
    PostgreSql {
        // source: PostgresGetProviderLinksError,
        // where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn get_providers_link_parts(
    resource: &Resource,
) -> Result<HashMap<ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorEnum>> {
    match resource {
        Resource::Local => match get_local_providers_link_parts(false).await {
            Err(error_hashmap) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Local {
                source: *error_hashmap,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            })),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        Resource::Mongodb => match mongo_get_providers_link_parts().await {
            Err(e) => Err(Box::new(GetProvidersLinkPartsErrorEnum::Mongodb {
                source: e,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            })),
            Ok(success_hashmap) => Ok(success_hashmap),
        },
        // Resource::PostgreSql => match postgres_get_providers_link_parts().await {
        //     Err(e) => Err(GetProvidersLinkPartsError {
        // source: Box::new(GetProvidersLinkPartsError::PostgreSql(e))
        // }),
        //     Ok(success_hashmap) => Ok(success_hashmap),
        // },
        Resource::PostgreSql => todo!(),
    }
}
