#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum NetCheckAvailabilityErrorNamed {
    ReqwestGet {
        #[eo_display]
        reqwest_get: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ResponseStatus {
        #[eo_display_foreign_type]
        status: reqwest::StatusCode,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn net_check_availability<T: app_state::GetStartingCheckLink + std::marker::Send + std::marker::Sync>(
    config: T,
) -> Result<(), Box<NetCheckAvailabilityErrorNamed>> {
    match reqwest::get(config.get_starting_check_link()).await {
        Err(error) => Err(Box::new(NetCheckAvailabilityErrorNamed::ReqwestGet {
            reqwest_get: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })),
        Ok(res) => {
            let status = res.status();
            match (status.is_client_error(), status.is_server_error()) {
                (true, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })),
                (true, false) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })),
                (false, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })),
                (false, false) => Ok(()),
            }
        }
    }
}
