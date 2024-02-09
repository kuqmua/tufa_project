//todo openapi
//todo test if create\update\delete empty array

#[derive(
    Debug,
    postgresql_crud::GeneratePostgresqlCrud,
)]
#[postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::commit_extractor::)]
    enum CommitExtractorCheckErrorNamed {
        #[tvfrr_400_bad_request]
        CommitExtractorNotEqual {
            #[eo_display_with_serialize_deserialize]
            commit_not_equal: std::string::String,
            #[eo_display_with_serialize_deserialize]
            commit_to_use: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        CommitExtractorToStrConversion {
            #[eo_display]
            commit_to_str_conversion: http::header::ToStrError,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        NoCommitExtractorHeader {
            #[eo_display_with_serialize_deserialize]
            no_commit_header: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}


///////////////////////////////
async fn tvfrr_extraction_logic_try_delete_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryDeleteOneRequestError> {
    let response = match future.await {
        Ok(response) => response, 
        Err(e) => {
            return Err(TryDeleteOneRequestError::Reqwest {
                reqwest: e, 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                    file!().to_string(),
                    line!(), 
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 916,
                        column: 13,
                    })
                ),
            });
        },
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text, 
        Err(e) => {
            return Err(TryDeleteOneRequestError::FailedToGetResponseText{
                reqwest: e, 
                status_code, 
                headers, 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                    file!().to_string(), 
                    line!(), 
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence{
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 904, 
                        column: 13,
                    })
                )
            });
        },
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value), 
            Err(e) => {
                return Err(TryDeleteOneRequestError::DeserializeResponse{
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text, 
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence{
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 910, 
                            column: 13,
                        })
                    )
                });
            },
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr500InternalServerError>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value), 
            Err(e) => {
                return Err(TryDeleteOneRequestError::DeserializeResponse{
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text, 
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence{
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 910, 
                            column: 13,
                        })
                    )
                });
            },
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr400BadRequest>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value), 
            Err(e) => {
                return Err(TryDeleteOneRequestError::DeserializeResponse{
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text, 
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence{
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 910, 
                            column: 13,
                        })
                    )
                });
            },
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryDeleteOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryDeleteOneResponseVariants::from(value), 
            Err(e) => {
                return Err(TryDeleteOneRequestError::DeserializeResponse{
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text, 
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence{
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 910, 
                            column: 13,
                        })
                    )
                });
            },
        }
    } else {
        return Err(
            TryDeleteOneRequestError::UnexpectedStatusCode {
                status_code, 
                headers, 
                response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(response_text), 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence{
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 898, 
                        column: 13,
                    })
                )
            }
        );
    };
    match crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::try_from(variants) {
        Ok(value) => Ok(value), 
        Err(e) => Err(TryDeleteOneRequestError :: ExpectedType {
            expected_type: e, 
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                file!().to_string(), 
                line!(), 
                column!(),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence {
                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                    line: 882, 
                    column: 17,
                })
            ),
        }),
    }
}