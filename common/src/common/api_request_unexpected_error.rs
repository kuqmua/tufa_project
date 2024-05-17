// #[derive(Debug)]
// pub enum ApiRequestUnexpectedError {
//     StatusCode {
//         status_code: http::StatusCode,
//         headers: reqwest::header::HeaderMap,
//         response_text_result: ResponseTextResult,
//     },
//     FailedToGetResponseText {
//         reqwest: reqwest::Error,
//         status_code: http::StatusCode,
//         headers: reqwest::header::HeaderMap,
//     },
//     DeserializeBody {
//         serde: serde_json::Error,
//         status_code: http::StatusCode,
//         headers: reqwest::header::HeaderMap,
//         response_text: std::string::String,
//     },
// }

// #[derive(Debug)]
// pub enum ResponseTextResult {
//     ReqwestError(reqwest::Error),
//     ResponseText(std::string::String),
// }

// impl error_occurence_lib::ToStdStringString for ResponseTextResult {
//     fn to_std_string_string(&self) -> std::string::String {
//         match self {
//             Self::ReqwestError(reqwest_error) => format!("{reqwest_error}"),
//             Self::ResponseText(response_text) => response_text.to_string(),
//         }
//     }
// }
