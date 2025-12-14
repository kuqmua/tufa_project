use crate::store::YewduxStore;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::use_store;

#[function_component(YewduxFunctionalEditUsername)]
pub fn yewdux_functional_edit_username() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let handle_form_submit = store
        .dispatch()
        .reduce_callback_with(|state, event: FocusEvent| {
            event.prevent_default();
            // let username = event
            //     .target()
            //     .expect("error d89e1b8b-be57-432a-8aa2-9bde186046af")
            //     .unchecked_into::<HtmlInputElement>()
            //     .value();
            state.username = String::from("kekw");
            log!("handle_form_submit username", state.username.clone());
        });
    let handle_username_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username = event
                .target()
                .expect("error aa9d4567-227b-4d95-92ca-adceb69666e8")
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;
            log!("handle_username_change username", state.username.clone());
        });
    html! {
      <form onsubmit={handle_form_submit}>
        <h1>{"Login"}</h1>
        <div>
          <input type="text" placeholder="username" onchange={handle_username_change}/>
        </div>
        <div>
          <button>{"Log in"}</button>
        </div>
      </form>
    }
}
