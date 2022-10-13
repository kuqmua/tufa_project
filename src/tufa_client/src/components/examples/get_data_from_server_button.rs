use crate::store::YewduxStore;
use gloo::console::log;
use lazy_static::__Deref;
use reqwasm::http::Request;
use serde_json::from_str;
use tufa_common::json_example::JsonExample;
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::use_store;

#[derive(Default, Clone, Debug)]
pub struct GetDataFromServerButtonState {
    username: String,
}

#[function_component(GetDataFromServerButton)]
pub fn get_data_from_server_button() -> Html {
    let state = use_state(GetDataFromServerButtonState::default);
    let cloned_state = state.clone();
    let store = use_store::<PersistentStore<YewduxStore>>();
    let handle_form_submit =
        store
            .dispatch()
            .reduce_callback_with(move |yewduxstore, event: FocusEvent| {
                event.prevent_default();
                yewduxstore.username = cloned_state.username.clone();
            });
    let onclick: Callback<MouseEvent> = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let f = Request::get("http://127.0.0.1:8081/api/json/json_example")
                    .send()
                    .await;
                match f {
                    Ok(k) => {
                        match k.text().await {
                            Ok(n) => {
                                log!(format!("ok {:#?}", n));
                                let json: Result<JsonExample, serde_json::Error> = from_str(&n);
                                match json {
                                    Ok(l) => {
                                        let mut state_cloned = state.deref().clone();
                                        state_cloned.username = l.second;
                                        state.set(state_cloned);
                                    }
                                    Err(e) => log!(format!("2err {:#?}", e)),
                                }
                            }
                            Err(e) => log!(format!("1err {:#?}", e)),
                        }
                        log!("ok {:#?}", k.body());
                    }
                    Err(_) => log!("0err"),
                }
                log!("Update2:");
            });
        })
    };
    let username = store
        .state()
        .map(|state| state.username.clone())
        .unwrap_or_default();
    html! {
        <div>
          <button onclick={onclick}>{"Log in"}</button>
          <div>{format!("!{}!", state.username.clone())}</div>
          <form onsubmit={handle_form_submit}>
            <div>
              <button>{"bif g"}</button>
            </div>
          </form>
          <div>{"username is"}</div>
          <div>{username}</div>
        </div>
    }
}
