use crate::helpers::request_result::RequestResult;
use gloo::console::log;
use reqwasm::http::Request;
use serde_json::json;
use yew::prelude::*;

#[function_component(PostDataToServerButton)]
pub fn post_data_to_server_button() -> Html {
    let request_result = use_state(|| RequestResult::NotExecuted);
    let onclick: Callback<MouseEvent> = {
        let request_result_cloned = request_result.clone();
        Callback::from(move |_| {
            let request_result_another_cloned = request_result_cloned.clone();
            wasm_bindgen_futures::spawn_local(async move {
                request_result_another_cloned.set(RequestResult::Pending);
                match Request::post("http://127.0.0.1:8081/api/json/json_example_post")
                    .header("content-type", "application/json")
                    .body(
                        json!({
                            "first": "first_from_tufa_client".to_string(),
                            "second": "second_from_tufa_client".to_string(),
                        })
                        .to_string(),
                    )
                    .send()
                    .await
                {
                    Err(e) => {
                        request_result_another_cloned.set(RequestResult::Error);
                        log!("error 57435634753434 ", e.to_string());
                    }
                    Ok(response) => {
                        let status = response.status();
                        match status {
                            200 => {
                                request_result_another_cloned.set(RequestResult::Success);
                            }
                            _ => {
                                request_result_another_cloned.set(RequestResult::Error);
                                log!("response status is ", response.status());
                            }
                        }
                    }
                }
            });
        })
    };
    html! {
      <div>
        {format!("{}",*request_result)}
        <button onclick={onclick}>{"post json example"}</button>
      </div>
    }
}
