#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum NetCheckAvailabilityErrorNamed {
    ReqwestGet {
        #[eo_display_foreign_type]
        reqwest_get: reqwest::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    ResponseStatus {
        #[eo_display_foreign_type]
        status: reqwest::StatusCode,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub async fn net_check_availability<'a>(
    config: &'static (impl crate::common::config::config_fields::GetStartingCheckLink
                  + std::marker::Send
                  + std::marker::Sync),
) -> Result<(), Box<NetCheckAvailabilityErrorNamed>> {
    match reqwest::get(config.get_starting_check_link()).await {
        Err(e) => Err(Box::new(NetCheckAvailabilityErrorNamed::ReqwestGet {
            reqwest_get: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        })),
        Ok(res) => {
            let status = res.status();
            match (status.is_client_error(), status.is_server_error()) {
                (true, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (true, false) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (false, true) => Err(Box::new(NetCheckAvailabilityErrorNamed::ResponseStatus {
                    status,
                    code_occurence: crate::code_occurence_tufa_common!(),
                })),
                (false, false) => Ok(()),
            }
        }
    }
}
