use crate::helpers::resource::Resource;
use gen_enum::GenEnum;
use gen_enum_without_values::GenEnumWithoutValues;
use init_from_env_with_panic_if_failed::InitFromEnvWithPanicIfFailedWithPanicIfFailed;
use tufa_common::config::source_place_type::SourcePlaceType;
use tufa_common::config::tracing_type::TracingType;

#[derive(
    Debug,
    Clone,
    InitFromEnvWithPanicIfFailedWithPanicIfFailed,
    GenEnum,
    GenEnumWithoutValues,
    Default,
    PartialEq,
    Eq,
)]
pub struct ConfigStruct {
    pub server_ip: String,
    pub server_port: u16,
    pub hmac_secret: String,
    pub base_url: String,
    pub require_ssl: bool,

    pub github_name: String,
    pub github_token: String,

    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,

    pub providers_link_parts_source: Resource,

    pub timezone: i32,

    pub redis_ip: String,
    pub redis_port: u16,

    pub mongo_first_handle_url_part: String,
    pub mongo_second_handle_url_part: String,
    pub mongo_third_handle_url_part: String,
    pub mongo_fourth_handle_url_part: String,
    pub mongo_fifth_handle_url_part: String,

    pub mongo_login: String,
    pub mongo_password: String,
    pub mongo_ip: String, //todo: 4x u8
    pub mongo_port: u16,
    pub mongo_params: String,

    pub mongo_connection_timeout: u64,

    pub mongo_providers_link_parts_db_name: String,
    pub mongo_providers_logs_db_name: String,
    pub mongo_providers_logs_db_collection_handle_second_part: String,
    pub mongo_providers_logs_db_collection_document_field_name_handle: String,

    pub is_mongo_initialization_enabled: bool,
    pub is_mongo_initialization_enabled_providers: bool,
    pub is_mongo_initialization_enabled_arxiv: bool,
    pub is_mongo_initialization_enabled_biorxiv: bool,
    pub is_mongo_initialization_enabled_github: bool,
    pub is_mongo_initialization_enabled_habr: bool,
    pub is_mongo_initialization_enabled_medrxiv: bool,
    pub is_mongo_initialization_enabled_reddit: bool,
    pub is_mongo_initialization_enabled_twitter: bool,

    pub is_mongo_write_error_logs_enabled: bool,
    pub is_mongo_write_error_logs_enabled_providers: bool,
    pub is_mongo_write_error_logs_enabled_arxiv: bool,
    pub is_mongo_write_error_logs_enabled_biorxiv: bool,
    pub is_mongo_write_error_logs_enabled_github: bool,
    pub is_mongo_write_error_logs_enabled_habr: bool,
    pub is_mongo_write_error_logs_enabled_medrxiv: bool,
    pub is_mongo_write_error_logs_enabled_reddit: bool,
    pub is_mongo_write_error_logs_enabled_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_github: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_enabled_twitter: bool,

    pub is_mongo_cleaning_warning_logs_db_collections_enabled: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_providers: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_arxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_biorxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_github: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_habr: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_medrxiv: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_reddit: bool,
    pub is_mongo_cleaning_warning_logs_db_collections_enabled_twitter: bool,

    pub is_mongo_link_parts_randomize_order_enabled: bool,
    pub is_mongo_link_parts_randomize_order_enabled_providers: bool,
    pub is_mongo_link_parts_randomize_order_enabled_arxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_biorxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_github: bool,
    pub is_mongo_link_parts_randomize_order_enabled_habr: bool,
    pub is_mongo_link_parts_randomize_order_enabled_medrxiv: bool,
    pub is_mongo_link_parts_randomize_order_enabled_reddit: bool,
    pub is_mongo_link_parts_randomize_order_enabled_twitter: bool,

    pub postgres_first_handle_url_part: String,
    pub postgres_second_handle_url_part: String,
    pub postgres_third_handle_url_part: String,
    pub postgres_fourth_handle_url_part: String,
    pub postgres_fifth_handle_url_part: String,
    pub postgres_sixth_handle_url_part: String,

    pub postgres_login: String,
    pub postgres_password: String,
    pub postgres_ip: String, //todo: 4x u8
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_params: String,

    pub postgres_connection_timeout: u64,

    pub is_postgres_initialization_enabled: bool,
    pub is_postgres_initialization_enabled_providers: bool,
    pub is_postgres_initialization_enabled_arxiv: bool,
    pub is_postgres_initialization_enabled_biorxiv: bool,
    pub is_postgres_initialization_enabled_github: bool,
    pub is_postgres_initialization_enabled_habr: bool,
    pub is_postgres_initialization_enabled_medrxiv: bool,
    pub is_postgres_initialization_enabled_reddit: bool,
    pub is_postgres_initialization_enabled_twitter: bool,

    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub path_to_provider_link_parts_folder: String,
    pub log_file_extension: String,

    pub is_write_error_logs_in_local_folder_enabled: bool,
    pub is_write_error_logs_in_local_folder_enabled_providers: bool,
    pub is_write_error_logs_in_local_folder_enabled_arxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_biorxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_github: bool,
    pub is_write_error_logs_in_local_folder_enabled_habr: bool,
    pub is_write_error_logs_in_local_folder_enabled_medrxiv: bool,
    pub is_write_error_logs_in_local_folder_enabled_reddit: bool,
    pub is_write_error_logs_in_local_folder_enabled_twitter: bool,

    pub is_cleaning_warning_logs_directory_enabled: bool,
    pub is_cleaning_warning_logs_directory_enabled_providers: bool,
    pub is_cleaning_warning_logs_directory_enabled_arxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_biorxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_github: bool,
    pub is_cleaning_warning_logs_directory_enabled_habr: bool,
    pub is_cleaning_warning_logs_directory_enabled_medrxiv: bool,
    pub is_cleaning_warning_logs_directory_enabled_reddit: bool,
    pub is_cleaning_warning_logs_directory_enabled_twitter: bool,

    pub starting_check_link: String, //todo add browser url limit check
    pub check_link_arxiv: String,    //todo add browser url limit check
    pub check_link_biorxiv: String,  //todo add browser url limit check
    pub check_link_github: String,   //todo add browser url limit check
    pub check_link_habr: String,     //todo add browser url limit check
    pub check_link_medrxiv: String,  //todo add browser url limit check
    pub check_link_reddit: String,   //todo add browser url limit check
    pub check_link_twitter: String,  //todo add browser url limit check

    pub is_enabled_providers: bool,
    pub is_enabled_arxiv: bool,
    pub is_enabled_biorxiv: bool,
    pub is_enabled_github: bool,
    pub is_enabled_habr: bool,
    pub is_enabled_medrxiv: bool,
    pub is_enabled_reddit: bool,
    pub is_enabled_twitter: bool,

    pub is_dbs_initialization_enabled: bool,
    pub is_dbs_initialization_enabled_providers: bool,
    pub is_dbs_initialization_enabled_arxiv: bool,
    pub is_dbs_initialization_enabled_biorxiv: bool,
    pub is_dbs_initialization_enabled_github: bool,
    pub is_dbs_initialization_enabled_habr: bool,
    pub is_dbs_initialization_enabled_medrxiv: bool,
    pub is_dbs_initialization_enabled_reddit: bool,
    pub is_dbs_initialization_enabled_twitter: bool,

    pub is_prints_enabled: bool,
    pub is_prints_enabled_providers: bool,
    pub is_prints_enabled_arxiv: bool,
    pub is_prints_enabled_biorxiv: bool,
    pub is_prints_enabled_github: bool,
    pub is_prints_enabled_habr: bool,
    pub is_prints_enabled_medrxiv: bool,
    pub is_prints_enabled_reddit: bool,
    pub is_prints_enabled_twitter: bool,

    pub is_warning_high_prints_enabled: bool,
    pub is_warning_high_prints_enabled_providers: bool,
    pub is_warning_high_prints_enabled_arxiv: bool,
    pub is_warning_high_prints_enabled_biorxiv: bool,
    pub is_warning_high_prints_enabled_github: bool,
    pub is_warning_high_prints_enabled_habr: bool,
    pub is_warning_high_prints_enabled_medrxiv: bool,
    pub is_warning_high_prints_enabled_reddit: bool,
    pub is_warning_high_prints_enabled_twitter: bool,

    pub is_warning_low_prints_enabled: bool,
    pub is_warning_low_prints_enabled_providers: bool,
    pub is_warning_low_prints_enabled_arxiv: bool,
    pub is_warning_low_prints_enabled_biorxiv: bool,
    pub is_warning_low_prints_enabled_github: bool,
    pub is_warning_low_prints_enabled_habr: bool,
    pub is_warning_low_prints_enabled_medrxiv: bool,
    pub is_warning_low_prints_enabled_reddit: bool,
    pub is_warning_low_prints_enabled_twitter: bool,

    pub is_success_prints_enabled: bool,
    pub is_success_prints_enabled_providers: bool,
    pub is_success_prints_enabled_arxiv: bool,
    pub is_success_prints_enabled_biorxiv: bool,
    pub is_success_prints_enabled_github: bool,
    pub is_success_prints_enabled_habr: bool,
    pub is_success_prints_enabled_medrxiv: bool,
    pub is_success_prints_enabled_reddit: bool,
    pub is_success_prints_enabled_twitter: bool,

    pub is_partial_success_prints_enabled: bool,
    pub is_partial_success_prints_enabled_providers: bool,
    pub is_partial_success_prints_enabled_arxiv: bool,
    pub is_partial_success_prints_enabled_biorxiv: bool,
    pub is_partial_success_prints_enabled_github: bool,
    pub is_partial_success_prints_enabled_habr: bool,
    pub is_partial_success_prints_enabled_medrxiv: bool,
    pub is_partial_success_prints_enabled_reddit: bool,
    pub is_partial_success_prints_enabled_twitter: bool,

    pub is_error_prints_enabled: bool,
    pub is_error_prints_enabled_providers: bool,
    pub is_error_prints_enabled_arxiv: bool,
    pub is_error_prints_enabled_biorxiv: bool,
    pub is_error_prints_enabled_github: bool,
    pub is_error_prints_enabled_habr: bool,
    pub is_error_prints_enabled_medrxiv: bool,
    pub is_error_prints_enabled_reddit: bool,
    pub is_error_prints_enabled_twitter: bool,

    pub is_time_measurement_prints_enabled: bool,
    pub is_time_measurement_prints_enabled_providers: bool,
    pub is_time_measurement_prints_enabled_arxiv: bool,
    pub is_time_measurement_prints_enabled_biorxiv: bool,
    pub is_time_measurement_prints_enabled_github: bool,
    pub is_time_measurement_prints_enabled_habr: bool,
    pub is_time_measurement_prints_enabled_medrxiv: bool,
    pub is_time_measurement_prints_enabled_reddit: bool,
    pub is_time_measurement_prints_enabled_twitter: bool,

    pub is_info_prints_enabled: bool,
    pub is_info_prints_enabled_providers: bool,
    pub is_info_prints_enabled_arxiv: bool,
    pub is_info_prints_enabled_biorxiv: bool,
    pub is_info_prints_enabled_github: bool,
    pub is_info_prints_enabled_habr: bool,
    pub is_info_prints_enabled_medrxiv: bool,
    pub is_info_prints_enabled_reddit: bool,
    pub is_info_prints_enabled_twitter: bool,

    pub is_links_limit_enabled_providers: bool,
    pub is_links_limit_enabled_arxiv: bool,
    pub is_links_limit_enabled_biorxiv: bool,
    pub is_links_limit_enabled_github: bool,
    pub is_links_limit_enabled_habr: bool,
    pub is_links_limit_enabled_medrxiv: bool,
    pub is_links_limit_enabled_reddit: bool,
    pub is_links_limit_enabled_twitter: bool,

    pub links_limit_providers: usize, //override links limit for providers. this value applied for each provider
    pub links_limit_arxiv: usize,
    pub links_limit_biorxiv: usize,
    pub links_limit_github: usize,
    pub links_limit_habr: usize,
    pub links_limit_medrxiv: usize,
    pub links_limit_reddit: usize,
    pub links_limit_twitter: usize,

    pub is_preparation_enabled: bool,
    pub is_tracing_enabled: bool,
    pub tracing_type: TracingType,
    pub is_parent_tracing_enabled: bool,
    pub source_place_type: SourcePlaceType,
    pub is_tracing_time_enabled: bool,

    pub error_red: u8,
    pub error_green: u8,
    pub error_blue: u8,
    pub warning_high_red: u8,
    pub warning_high_green: u8,
    pub warning_high_blue: u8,
    pub warning_low_red: u8,
    pub warning_low_green: u8,
    pub warning_low_blue: u8,
    pub success_red: u8,
    pub success_green: u8,
    pub success_blue: u8,
    pub partial_success_red: u8,
    pub partial_success_green: u8,
    pub partial_success_blue: u8,
    pub cleaning_red: u8,
    pub cleaning_green: u8,
    pub cleaning_blue: u8,
    pub time_measurement_red: u8,
    pub time_measurement_green: u8,
    pub time_measurement_blue: u8,
    pub info_red: u8,
    pub info_green: u8,
    pub info_blue: u8,
}

#[cfg(test)]
mod tests {
    use crate::config_mods::config_struct::ConfigStruct;

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    #[test]
    fn ci_check_compromised_env_vars() {
        let config_from_env = ConfigStruct::new().expect("cannot create ConfigStruct::new()");
        let default_config = ConfigStruct::default();
        assert_eq!(config_from_env, default_config)
    }

    use crate::config_mods::config_struct::ConfigStructEnumWithoutValues;
    use crate::tests::constants::DOCKER_COMPOSE_FILE_NAME;
    use crate::tests::constants::PATH_TO_DOCKER_COMPOSE_FILE;
    use std::fs;
    use strum::IntoEnumIterator;

    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    #[test]
    pub fn ci_check_env_var_names_contains_in_docker_compose() {
        let result_of_reading_to_string = fs::read_to_string(&format!(
            "{PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}"
        ));
        match result_of_reading_to_string {
            Err(e) => {
                panic!("cannot read_to_string from file {PATH_TO_DOCKER_COMPOSE_FILE}{DOCKER_COMPOSE_FILE_NAME}, reason: {e}");
            }
            Ok(file_content) => {
                let mut vec = Vec::with_capacity(ConfigStructEnumWithoutValues::get_length());
                for i in ConfigStructEnumWithoutValues::iter() {
                    let env_name = i.to_upper_snake_case();
                    println!("env_namem {env_name}");
                    if !file_content.contains(&env_name) {
                        vec.push(env_name);
                    }
                }
                if !vec.is_empty() {
                    panic!("no such env name(s) inside docker-compose: {vec:?}");
                }
            }
        }
    }
}
