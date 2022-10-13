use crate::lazy_static::config::CONFIG;
use crate::providers::provider_kind::provider_kind_enum::CleanLogsDirError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use crate::providers::provider_kind::provider_kind_enum::RemoveDirError;
use crate::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links;
use crate::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links;
use crate::providers::providers_info::links::generate_github_links::generate_github_links;
use crate::providers::providers_info::links::generate_habr_links::generate_habr_links;
use crate::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links;
use crate::providers::providers_info::links::generate_reddit_links::generate_reddit_links;
use crate::providers::providers_info::links::generate_twitter_links::generate_twitter_links;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;

impl ProviderKindTrait for ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_item_handle(&self) -> Option<&'static str> {
        match self {
            ProviderKind::Arxiv => Some("</item>"),
            ProviderKind::Biorxiv => Some("</item>"),
            ProviderKind::Github => Some("</entry>"),
            ProviderKind::Habr => Some("</item>"),
            ProviderKind::Medrxiv => Some("</item>"),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some("</item>"),
        }
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_mongo_log_collection_name(&self) -> String {
        format!(
            "{self}{}",
            CONFIG.mongo_providers_logs_db_collection_handle_second_part //todo rename it into db log collection
        )
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_path_to_logs_directory(&self) -> String {
        format!("logs/{}/{self:?}", &CONFIG.warning_logs_directory_name)
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_path_to_provider_log_file(&self) -> String {
        format!(
            "logs/{}/{self:?}/{}",
            &CONFIG.warning_logs_directory_name,
            &CONFIG.unhandled_success_handled_success_are_there_items_initialized_posts_dir
        )
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_init_local_data_file_path(&self) -> String {
        format!(
            "{}{self}_link_parts{}",
            CONFIG.path_to_provider_link_parts_folder, CONFIG.log_file_extension
        )
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn remove_logs_directory(&self) -> Result<(), CleanLogsDirError> {
        let path = self.get_path_to_logs_directory();
        if !Path::new(&path).is_dir() {
            return Err(CleanLogsDirError::PathIsNotDir { path });
        }
        fs::remove_dir_all(&path)?; //todo: its blocking, rewrite to async //update: also its has vulnerability https://blog.rust-lang.org/2022/01/20/cve-2022-21658.html
        Ok(())
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn stringify(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => stringify!(ProviderKind::Arxiv),
            ProviderKind::Biorxiv => stringify!(ProviderKind::Biorxiv),
            ProviderKind::Github => stringify!(ProviderKind::Github),
            ProviderKind::Habr => stringify!(ProviderKind::Habr),
            ProviderKind::Medrxiv => stringify!(ProviderKind::Medrxiv),
            ProviderKind::Reddit => stringify!(ProviderKind::Reddit),
            ProviderKind::Twitter => stringify!(ProviderKind::Twitter),
        }
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn generate_provider_links(&self, names_vector: Vec<String>) -> Vec<String> {
        match self {
            ProviderKind::Arxiv => generate_arxiv_links(names_vector),
            ProviderKind::Biorxiv => generate_biorxiv_links(names_vector),
            ProviderKind::Github => generate_github_links(names_vector),
            ProviderKind::Habr => generate_habr_links(names_vector),
            ProviderKind::Medrxiv => generate_medrxiv_links(names_vector),
            ProviderKind::Reddit => generate_reddit_links(names_vector),
            ProviderKind::Twitter => generate_twitter_links(names_vector),
        }
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn generate_hashmap_with_empty_string_vecs_for_enabled_providers(
    ) -> HashMap<ProviderKind, Vec<String>> {
        ProviderKind::get_enabled_providers_vec()
            .iter()
            .map(|pk| (*pk, Vec::<String>::new()))
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_enabled_providers_vec() -> Vec<ProviderKind> {
        ProviderKind::iter().filter(|pk| pk.is_enabled()).collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_enabled_string_name_vec() -> Vec<String> {
        ProviderKind::iter()
            .filter_map(|pk| {
                if pk.is_enabled() {
                    return Some(format!("{pk}"));
                }
                None
            })
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_mongo_initialization_provider_kind_vec() -> Vec<ProviderKind> {
        ProviderKind::iter()
            .filter(|pk| pk.is_mongo_initialization_enabled())
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_mongo_initialization_string_name_vec() -> Vec<String> {
        ProviderKind::iter()
            .filter_map(|pk| {
                if pk.is_mongo_initialization_enabled() {
                    return Some(format!("{pk}"));
                }
                None
            })
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn into_string_name_and_kind_hashmap() -> HashMap<String, ProviderKind> {
        ProviderKind::iter()
            .map(|pk| (format!("{pk}"), pk))
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(String, ProviderKind)> {
        ProviderKind::iter()
            .map(|pk| (format!("{pk}"), pk))
            .collect()
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn remove_existing_providers_logs_directories(
    ) -> Result<(), HashMap<ProviderKind, RemoveDirError>> {
        if let Err(error_hashmap) = ProviderKind::remove_providers_logs_directories() {
            let return_hashmap = error_hashmap
                .into_iter()
                .filter_map(|(pk, error)| {
                    if let CleanLogsDirError::CannotRemoveDir { error: e } = error {
                        return Some((pk, e));
                    }
                    None
                })
                .collect::<HashMap<ProviderKind, RemoveDirError>>();
            if !return_hashmap.is_empty() {
                return Err(return_hashmap);
            }
            return Ok(());
        }
        Ok(())
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn remove_providers_logs_directories() -> Result<(), HashMap<ProviderKind, CleanLogsDirError>> {
        let result_hashmap = ProviderKind::iter()
            .filter_map(|pk| {
                if pk.is_cleaning_warning_logs_directory_enabled() {
                    if let Err(e) = pk.remove_logs_directory() {
                        return Some((pk, e));
                    }
                }
                None
            })
            .collect::<HashMap<ProviderKind, CleanLogsDirError>>();
        if !result_hashmap.is_empty() {
            return Err(result_hashmap);
        }
        Ok(())
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_db_tag(&self) -> String {
        format!("{self}")
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_postgres_table_name(&self) -> String {
        format!("{}_link_parts", self.to_lower_snake_case())
    }

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_dbs_initialization_enabled_vec() -> Vec<ProviderKind> {
        ProviderKind::iter()
            .filter(|pk| pk.is_dbs_initialization_enabled())
            .collect()
    }
}
